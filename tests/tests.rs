extern crate tempdir;
extern crate tuf;
extern crate url;

#[ignore(dead_code, unused_variables)] // TODO remove when stable

use std::fs;
use tempdir::TempDir;
use tuf::{Tuf, Config};
use url::Url;

#[test]
fn init() {
    let tempdir = TempDir::new("rust-tuf").expect("couldn't make temp dir");

    fs::copy("./tests/repo-1/meta/root.json", tempdir.path().join("root.json"))
        .expect("copy failed");

    let config = Config::build()
        .url(Url::parse("http://localhost:8080").expect("bad url"))
        .local_path(tempdir.into_path())
        .finish()
        .expect("bad config");
    let tuf = Tuf::new(config).expect("failed to initialize TUF");
}
