#![feature(test)]
mod match_u8 {
    extern crate test;
    use self::test::Bencher;
    extern crate tree_magic;

    #[cfg(not(feature="staticmime"))]
    macro_rules! convmime {
        ($x:expr) => {$x.to_string()}
    }
    #[cfg(feature="staticmime")]
    macro_rules! convmime {
        ($x:expr) => {$x}
    }

    ///Image benchmarks
    #[bench]
    fn image_gif(b: &mut Bencher) {
        b.iter(|| tree_magic::match_u8("image/gif", include_bytes!("image/gif")));
    }
    #[bench]
    fn image_png(b: &mut Bencher) {
        b.iter(|| tree_magic::match_u8("image/png", include_bytes!("image/png")));
    }

    /// Archive tests
    #[bench]
    fn application_zip(b: &mut Bencher) {
        b.iter(|| tree_magic::match_u8("application/zip", include_bytes!("application/zip")));
    }

    /// Text tests
    #[bench]
    fn text_plain(b: &mut Bencher) {
        b.iter(|| tree_magic::match_u8("text/plain", include_bytes!("text/plain")));
    }

}
