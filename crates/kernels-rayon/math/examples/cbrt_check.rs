//! Is the bit-exact `cbrt` the kernels call actually the platform libm, and
//! what does it cost relative to calling the platform libm directly?
//!
//! `rmath_bitexact` exists so every transcendental agrees bit-for-bit with the
//! libm C libxc calls. `f64::cbrt` on Linux *is* that libm. If the two agree,
//! the reimplementation buys nothing the direct call does not already have,
//! and any cost difference is pure loss.
//!
//!     cargo run --release --manifest-path crates/kernels-rayon/math/Cargo.toml \
//!         --example cbrt_check

use libxc_rkernel_math::rmath;
use std::hint::black_box;
use std::time::Instant;

fn lcg(s: &mut u64) -> f64 {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    // physical density-like range: 1e-8 .. 1e2
    let u = ((*s >> 11) as f64) / ((1u64 << 53) as f64);
    10f64.powf(-8.0 + 10.0 * u)
}

fn main() {
    // ---- exactness -------------------------------------------------------
    let mut s = 0x1234_5678_9abc_def0u64;
    let xs: Vec<f64> = (0..2_000_000).map(|_| lcg(&mut s)).collect();

    let mut diff = 0usize;
    let mut worst_ulp = 0i64;
    for &x in &xs {
        let a = rmath::cbrt(x);
        let b = x.cbrt();
        if a.to_bits() != b.to_bits() {
            diff += 1;
            let d = (a.to_bits() as i64 - b.to_bits() as i64).abs();
            if d > worst_ulp {
                worst_ulp = d;
            }
        }
    }
    println!("rmath::cbrt (BitExact) vs f64::cbrt (platform libm)");
    println!("  inputs           : {}", xs.len());
    println!("  differing        : {diff}  ({:.4} %)", 100.0 * diff as f64 / xs.len() as f64);
    println!("  worst ulp        : {worst_ulp}");

    // also check the exact-integer-cube cases and specials
    for &x in &[0.0f64, -0.0, 1.0, 8.0, 27.0, 64.0, 1e-300, 1e300, -8.0] {
        let (a, b) = (rmath::cbrt(x), x.cbrt());
        if a.to_bits() != b.to_bits() {
            println!("  SPECIAL DIFFERS  : cbrt({x:e}) rmath={a:e} libm={b:e}");
        }
    }

    // ---- cost ------------------------------------------------------------
    let time = |label: &str, f: &dyn Fn(f64) -> f64| {
        // warm
        let mut acc = 0.0f64;
        for &x in xs.iter().take(100_000) {
            acc += f(x);
        }
        black_box(acc);
        let mut best = f64::INFINITY;
        for _ in 0..5 {
            let t = Instant::now();
            let mut acc = 0.0f64;
            for &x in &xs {
                acc += f(black_box(x));
            }
            black_box(acc);
            let ns = t.elapsed().as_secs_f64() * 1e9 / xs.len() as f64;
            if ns < best {
                best = ns;
            }
        }
        println!("  {label:24} {best:6.2} ns/elem");
        best
    };

    println!("\nscalar cost (best of 5 over {} elems)", xs.len());
    let a = time("rmath::cbrt BitExact", &|x| rmath::cbrt(x));
    let b = time("f64::cbrt (libm)", &|x| x.cbrt());
    let c = time("rmath::fast::cbrt", &|x| rmath::fast::cbrt(x));
    println!("\n  BitExact / libm  = {:.2}x", a / b);
    println!("  fast     / libm  = {:.2}x", c / b);
}
