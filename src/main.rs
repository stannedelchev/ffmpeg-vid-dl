use std::env;
use std::iter::Peekable;
use std::process::{Command, Stdio};

fn main() {
    let args = env::args().skip(1).skip_gif_arg().collect::<Vec<String>>();

    let mut child = Command::new("ffmpeg-orig.exe")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("ffmpeg-orig command failed to start");

    let _ = child.wait();
}

struct GifOverlayFilter<I>
where
    I: Iterator<Item = String>,
{
    inner: Peekable<I>,
}

impl<I> GifOverlayFilter<I>
where
    I: Iterator<Item = String>,
{
    fn new(inner: I) -> GifOverlayFilter<I> {
        GifOverlayFilter {
            inner: inner.peekable(),
        }
    }
}

impl<I> Iterator for GifOverlayFilter<I>
where
    I: Iterator<Item = String>,
{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_val) = self.inner.next() {
            match next_val {
                arg if &arg == "-i" => {
                    if let Some(i_arg) = self.inner.peek() {
                        if i_arg.ends_with(".gif") {
                            let _ = self.inner.next();
                            continue;
                        }
                    }

                    return Some(arg);
                }
                arg if &arg == "-filter_complex" => continue,
                arg if &arg == "[1:v][2:v] overlay=25:25" => continue,
                arg => return Some(arg),
            }
        }

        None
    }
}

trait GifArgFilteringIterator: Iterator<Item = String> {
    fn skip_gif_arg(self) -> GifOverlayFilter<Self>
    where
        Self: Sized,
    {
        GifOverlayFilter::new(self)
    }
}

impl<T> GifArgFilteringIterator for T where T: Iterator<Item = String> {}
