
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "Rust easy install",
about = "Testing installation of compiled rust program"
)]
pub struct CliOptions {
    /// Url of tar.gz or tar.lz4 file
    #[structopt(long = "url", default_value = "")]
    pub option: String,
}

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const REVISION: Option<&str> = option_env!("GIT_REVISION");
    let revision = REVISION.unwrap_or("unknown");
    let options:CliOptions = CliOptions::from_args();

    println!("Rust easy install VERSION: {VERSION}, REVISION: {revision}");
}
