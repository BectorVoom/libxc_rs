//! Bit-identity of the `simd` transcendentals against the scalar calls they
//! replace: `simd::exp`/`simd::ln` vs the system libm (`f64::exp`/`f64::ln`),
//! `simd::cbrt` and the `pow_n_3` family vs `powers::cbrt_f64` and the scalar
//! `pow_n_3` sequence.
//!
//! These are exact (`to_bits()`) comparisons, not tolerance checks: the whole
//! point of the module is that a SIMD kernel built on it produces the same
//! bits as the scalar kernel. NaN lanes must round-trip the same payload,
//! which holds because non-main-path lanes call the scalar function directly.
//!
//! Run in release (`cargo test --release -p libxc-rkernel-math`) — the sweeps
//! are millions of points.

use libxc_rkernel_math::powers;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::f64x8;

/// xorshift64* — deterministic, no dev-dependency.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn f01(&mut self) -> f64 {
        (self.next() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    /// Uniform in [a, b].
    fn uniform(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.f01()
    }
    /// Log-uniform magnitude in [lo, hi], random sign if `signed`.
    fn logmag(&mut self, lo: f64, hi: f64, signed: bool) -> f64 {
        let m = (self.uniform(lo.ln(), hi.ln())).exp();
        if signed && self.next() & 1 == 0 { -m } else { m }
    }
}

fn check_all(vals: &[f64], scalar: impl Fn(f64) -> f64, vector: impl Fn(f64x8) -> f64x8, what: &str) {
    let mut mism = 0usize;
    for chunk in vals.chunks(8) {
        let mut lanes = [chunk[0]; 8];
        lanes[..chunk.len()].copy_from_slice(chunk);
        let got: [f64; 8] = vector(f64x8::new(lanes)).into();
        for l in 0..chunk.len() {
            let want = scalar(lanes[l]);
            if want.to_bits() != got[l].to_bits() {
                mism += 1;
                if mism <= 10 {
                    eprintln!(
                        "{what}: x={:e} ({:#018x}) scalar={:e} ({:#018x}) simd={:e} ({:#018x})",
                        lanes[l],
                        lanes[l].to_bits(),
                        want,
                        want.to_bits(),
                        got[l],
                        got[l].to_bits()
                    );
                }
            }
        }
    }
    assert_eq!(mism, 0, "{what}: {mism} of {} lanes differ", vals.len());
}

/// Edge values every function is fed, plus per-test extras.
fn specials() -> Vec<f64> {
    let mut v = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        f64::MIN_POSITIVE,
        f64::MIN_POSITIVE / 2.0, // subnormal
        f64::from_bits(1),       // smallest subnormal
        f64::MAX,
        f64::MIN,
    ];
    // Neighborhoods of every branch boundary in exp/ln/cbrt.
    for c in [
        f64::from_bits(0x3c90000000000000), // 2^-54 (exp tiny cut)
        512.0,                              // exp main-path cut
        709.782712893383973096,             // exp overflow cut
        -708.39641853226410622,
        -745.13321910194110842,
        0.9375,             // ln near-1 low cut
        1.06475830078125,   // ln near-1 high cut (1 + 0x1.09p-4)
        f64::MIN_POSITIVE,  // ln normal cut
    ] {
        for d in [-2i64, -1, 0, 1, 2] {
            let b = c.to_bits().wrapping_add(d as u64);
            v.push(f64::from_bits(b));
            v.push(-f64::from_bits(b));
        }
    }
    v
}

#[test]
fn exp_bit_identical_to_glibc() {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut vals = specials();
    for _ in 0..1_000_000 {
        vals.push(rng.uniform(-40.0, 40.0)); // physical kernel range
    }
    for _ in 0..500_000 {
        vals.push(rng.uniform(-760.0, 720.0)); // full range incl. under/overflow
    }
    for _ in 0..500_000 {
        vals.push(rng.logmag(1e-30, 1e3, true)); // tiny-|x| branch coverage
    }
    for _ in 0..250_000 {
        vals.push(f64::from_bits(rng.next())); // adversarial bit patterns
    }
    check_all(&vals, f64::exp, simd::exp, "exp");
}

#[test]
fn ln_bit_identical_to_glibc() {
    let mut rng = Rng(0xD1B54A32D192ED03);
    let mut vals = specials();
    for _ in 0..1_000_000 {
        vals.push(rng.logmag(1e-30, 1e10, false)); // physical densities & co.
    }
    for _ in 0..500_000 {
        vals.push(rng.uniform(0.85, 1.15)); // dense across the near-1 window
    }
    for _ in 0..500_000 {
        vals.push(rng.logmag(1e-320, 1e308, true)); // full range, incl. negatives
    }
    for _ in 0..250_000 {
        vals.push(f64::from_bits(rng.next()));
    }
    check_all(&vals, f64::ln, simd::ln, "ln");
}

#[test]
fn cbrt_bit_identical_to_scalar_kernels() {
    let mut rng = Rng(0xA0761D6478BD642F);
    let mut vals = specials();
    for _ in 0..1_000_000 {
        vals.push(rng.logmag(1e-30, 1e10, true));
    }
    for _ in 0..500_000 {
        vals.push(f64::from_bits(rng.next()));
    }
    check_all(&vals, powers::cbrt_f64, simd::cbrt, "cbrt");
    check_all(&vals, powers::pow_2_3, simd::pow_2_3, "pow_2_3");
    check_all(&vals, powers::pow_4_3, simd::pow_4_3, "pow_4_3");
    check_all(&vals, powers::pow_5_3, simd::pow_5_3, "pow_5_3");
    check_all(&vals, powers::pow_7_3, simd::pow_7_3, "pow_7_3");
}

/// A special-cased lane must not perturb its neighbours (mask-patch check).
#[test]
fn mixed_special_lanes_do_not_leak() {
    let cases = [
        [1.3, f64::NAN, -2.0, 0.0, 700.0, -745.0, 1e-300, 2.5],
        [f64::INFINITY, 0.9375, 1.0, -0.0, 5e-324, 513.0, -1.0, 1.000001],
    ];
    for lanes in cases {
        let ge: [f64; 8] = simd::exp(f64x8::new(lanes)).into();
        let gl: [f64; 8] = simd::ln(f64x8::new(lanes)).into();
        let gc: [f64; 8] = simd::cbrt(f64x8::new(lanes)).into();
        for l in 0..8 {
            assert_eq!(ge[l].to_bits(), lanes[l].exp().to_bits(), "exp lane {l}");
            assert_eq!(gl[l].to_bits(), lanes[l].ln().to_bits(), "ln lane {l}");
            assert_eq!(gc[l].to_bits(), powers::cbrt_f64(lanes[l]).to_bits(), "cbrt lane {l}");
        }
    }
}
