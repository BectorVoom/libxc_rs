//! Modified Bessel functions of the first kind for CubeCL kernels.
//!
//! Translates libxc `bessel.c` (`xc_bessel_I0`, `xc_bessel_I0_scaled`,
//! `xc_bessel_I1`, `xc_bessel_I1_scaled`) into CubeCL-compatible form. The
//! C implementation is a SLATEC routine (W. Fullerton) that branches on |x|
//! and evaluates a Chebyshev series. We follow the same control flow with
//! mutable result + `if/else` (no early returns), and inline each Chebyshev
//! series as a separate `#[cube]` function with hardcoded coefficients —
//! identical pattern to `expint_e1::cheb_ae11` etc.
//!
//! Operation order matches the C source so the f64 result is bit-identical
//! to libxc 7.0.0 for finite inputs.

#![allow(clippy::excessive_precision, clippy::needless_return, non_snake_case)]


// `rmath` below is `crate::rmath` -- this crate's BitExact surface, not the
// upstream crate, whose free functions are deliberately the Fast path.
use crate::rmath;

// ---------------------------------------------------------------------------
// Useful constants (from libxc `util.h`)
// ---------------------------------------------------------------------------

// SQRT_DBL_EPSILON = sqrt(2.220446049250313e-16)
const SQRT_DBL_EPSILON: f64 = 1.4901161193847656e-8;
// LOG_DBL_MAX = ln(1.7976931348623157e+308)
const LOG_DBL_MAX: f64 = 709.7827128933840;
// 2 * DBL_MIN
const TWO_DBL_MIN: f64 = 4.450147717014403e-308;
// 2 * sqrt(2) * SQRT_DBL_EPSILON
const TWO_SQRT2_SQRT_DBL_EPSILON: f64 = 4.214684242274519e-8;

// ---------------------------------------------------------------------------
// Chebyshev evaluators (Clenshaw recurrence, manual unrolling)
// ---------------------------------------------------------------------------

/// `bi0_data` (12 coefficients) — series for I0(x) on |x| <= 3.
fn cheb_bi0(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000245_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000053339_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000009579451_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000001396650044_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000161384906966_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000014340062895106_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000942265768600193_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00043442709008164874_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.01304891466707290428_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.22826445869203013390_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  1.92733795399380827000_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.07660547252839144951_f64;
    0.5_f64 * (b0 - b2)
}

/// `ai0_data` (21 coefficients) — series for I0(x) scaled, 3 <= |x| <= 8.
fn cheb_ai0(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000007_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000071_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000314_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000608_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000002415_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000027155_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000114684_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000112822_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000001757854_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000011916228_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000022925563_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000155964859_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000001204463945_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000825247260_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000027838499429_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000078261435014_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000790117997921_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000001070076463439_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000041531313389237_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00759138081082334_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.07575994494023796_f64;
    0.5_f64 * (b0 - b2)
}

/// `ai02_data` (22 coefficients) — series for I0(x) scaled, |x| > 8.
fn cheb_ai02(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000003_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000027_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000034_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000176_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000382_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000954_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000004151_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000001539_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000038529_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000071801_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000179419_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000001321580_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000003149915_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000001188914_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000049406022_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000339623203_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000002266668991_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000020489185893_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000289137052082_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000006889758346918_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00336911647825569_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.05449041101410882_f64;
    0.5_f64 * (b0 - b2)
}

/// `bi1_data` (11 coefficients) — series for I1(x) on |x| <= 3.
fn cheb_bi1(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.000000000000000024_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.000000000000004741_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.000000000000766380_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.000000000099322077_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.000000010042493924_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.000000764902676483_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.000041888521098377_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.001545394556300123_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.034838994299959456_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.407348876675464810_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.001971713261099859_f64;
    0.5_f64 * (b0 - b2)
}

/// `ai1_data` (21 coefficients) — series for I1(x) scaled, 3 <= |x| <= 8.
fn cheb_ai1(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000006_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000071_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000333_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000730_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000002023_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000027315_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000124260_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000166665_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000001664947_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000012663889_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000029085122_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000144842341_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000001318012367_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000001559378146_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000029183389184_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000104949824671_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000858561914581_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000002069971253350_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000061151858579437_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.01922953231443221_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.02846744181881479_f64;
    0.5_f64 * (b0 - b2)
}

/// `ai12_data` (22 coefficients) — series for I1(x) scaled, |x| > 8.
fn cheb_ai12(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000003_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000028_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000033_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000186_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000000382_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000001041_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000004273_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000002101_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000040836_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000071985_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000203564_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000001412580_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000003252602_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000001897495_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000055897433_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000383538039_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000002631468847_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000025122362377_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000388256480887_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000011058893876263_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00976109749136147_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.02857623501828014_f64;
    0.5_f64 * (b0 - b2)
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Exponentially scaled modified Bessel function of the first kind, order 0:
/// `exp(-|x|) * I0(x)`. Stays bounded for all real `x`.
///
/// Mirrors libxc `xc_bessel_I0_scaled` in `bessel.c`.
pub fn xc_bessel_I0_scaled(x: f64) -> f64 {
    let y = rmath::abs(x);
    let mut r: f64 = 0.0_f64;

    if y < 2.0_f64 * (SQRT_DBL_EPSILON as f64) {
        r = 1.0_f64 - y;
    } else if y <= 3.0_f64 {
        r = rmath::exp(-y) * (2.75_f64 + cheb_bi0(y * y / 4.5_f64 - 1.0_f64));
    } else if y <= 8.0_f64 {
        r = (0.375_f64 + cheb_ai0((48.0_f64 / y - 11.0_f64) / 5.0_f64)) / rmath::sqrt(y);
    } else {
        r = (0.375_f64 + cheb_ai02(16.0_f64 / y - 1.0_f64)) / rmath::sqrt(y);
    }

    r
}

/// Modified Bessel function of the first kind, order 0: `I0(x)`.
///
/// For very large `|x|` the result overflows; libxc prints a warning and
/// returns 0 — we match the silent zero behavior since the GPU path also
/// can't print, and overflow is not expected for XC inputs.
///
/// Mirrors libxc `xc_bessel_I0` in `bessel.c`.
pub fn xc_bessel_I0(x: f64) -> f64 {
    let y = rmath::abs(x);
    let mut r: f64 = 0.0_f64;

    if y < 2.0_f64 * (SQRT_DBL_EPSILON as f64) {
        r = 1.0_f64;
    } else if y <= 3.0_f64 {
        r = 2.75_f64 + cheb_bi0(y * y / 4.5_f64 - 1.0_f64);
    } else if y < (LOG_DBL_MAX as f64) - 1.0_f64 {
        r = rmath::exp(y) * xc_bessel_I0_scaled(x);
    }
    // else: overflow path — leave r = 0.0 (no stderr in #[cube]).

    r
}

/// Exponentially scaled modified Bessel function of the first kind, order 1:
/// `sign(x) * exp(-|x|) * I1(x)`.
///
/// Mirrors libxc `xc_bessel_I1_scaled` in `bessel.c`.
pub fn xc_bessel_I1_scaled(x: f64) -> f64 {
    let y = rmath::abs(x);
    let mut r: f64 = 0.0_f64;

    if y == 0.0_f64 {
        r = 0.0_f64;
    } else if y < (TWO_DBL_MIN as f64) {
        // Underflow: leave r = 0.0 (libxc prints to stderr; not available here).
        r = 0.0_f64;
    } else if y < (TWO_SQRT2_SQRT_DBL_EPSILON as f64) {
        r = 0.5_f64 * x * rmath::exp(-y);
    } else if y <= 3.0_f64 {
        r = x * rmath::exp(-y) * (0.875_f64 + cheb_bi1(y * y / 4.5_f64 - 1.0_f64));
    } else {
        let mut rr: f64 = 0.0_f64;
        if y <= 8.0_f64 {
            rr = (0.375_f64 + cheb_ai1((48.0_f64 / y - 11.0_f64) / 5.0_f64)) / rmath::sqrt(y);
        } else {
            rr = (0.375_f64 + cheb_ai12(16.0_f64 / y - 1.0_f64)) / rmath::sqrt(y);
        }
        if x > 0.0_f64 { r = rr; } else { r = -rr; }
    }

    r
}

/// Modified Bessel function of the first kind, order 1: `I1(x)`.
///
/// Mirrors libxc `xc_bessel_I1` in `bessel.c`.
pub fn xc_bessel_I1(x: f64) -> f64 {
    let y = rmath::abs(x);
    let mut r: f64 = 0.0_f64;

    if y == 0.0_f64 {
        r = 0.0_f64;
    } else if y < (TWO_DBL_MIN as f64) {
        // Underflow: leave r = 0.0.
        r = 0.0_f64;
    } else if y < (TWO_SQRT2_SQRT_DBL_EPSILON as f64) {
        r = 0.5_f64 * x;
    } else if y <= 3.0_f64 {
        r = x * (0.875_f64 + cheb_bi1(y * y / 4.5_f64 - 1.0_f64));
    } else {
        r = rmath::exp(x) * xc_bessel_I1_scaled(x);
    }

    r
}

// ---------------------------------------------------------------------------
// Modified Bessel of the second kind: Chebyshev series tables (K0, K1)
// ---------------------------------------------------------------------------

/// `bk0_data` (11 coefficients) — series for K0(x) on x ≤ 2.
fn cheb_bk0(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000000035_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000013744_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000004259816_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000001034969525_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000190451637722_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000025347910790261_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00002286212103119451_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00126461541144692592_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.03597993651536150163_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.34428989992462848690_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.03532739323390276872_f64;
    0.5_f64 * (b0 - b2)
}

/// `ak0_data` (17 coefficients) — series for K0(x) scaled, 2 ≤ x ≤ 8.
fn cheb_ak0(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000005_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000033_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000215_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000001427_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000009744_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000068895_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000506804_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000003902353_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000031694296_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000274270554_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000002563713036_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000026393672220_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000308170017386_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00004281006688886_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00077341811546938_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.02235652605699819_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.07643947903327941_f64;
    0.5_f64 * (b0 - b2)
}

/// `ak02_data` (14 coefficients) — series for K0(x) scaled, x > 8.
fn cheb_ak02(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000002_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000020_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000192_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000001925_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000020743_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000243501_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000003158592_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000046111825_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000777011043_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000015678318108_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000401361417543_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00014445509317750_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00917485269102569_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.01201869826307592_f64;
    0.5_f64 * (b0 - b2)
}

/// `bk1_data` (11 coefficients) — series for K1(x) on x ≤ 2.
fn cheb_bk1(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.0000000000000000070_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.0000000000000024274_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.0000000000006666901_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.0000000001411488392_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.0000000221338763073_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.0000024334061415659_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.0001730288957513052_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.0069757238596398643_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.1226111808226571480_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.3531559607765448760_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.0253002273389477705_f64;
    0.5_f64 * (b0 - b2)
}

/// `ak1_data` (17 coefficients) — series for K1(x) scaled, 2 ≤ x ≤ 8.
fn cheb_ak1(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000006_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000038_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000248_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000001654_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000011386_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000081284_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000604783_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000004720819_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000038989323_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000344597758_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000003311163779_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000035402774997_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000436998470952_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00006650116955125_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00144105155647540_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.07571989953199368_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.27443134069738830_f64;
    0.5_f64 * (b0 - b2)
}

/// `ak12_data` (14 coefficients) — series for K1(x) scaled, x > 8.
fn cheb_ak12(x: f64) -> f64 {
    let twox = 2.0_f64 * x;
    let mut b0: f64 = 0.0_f64;
    let mut b1: f64 = 0.0_f64;
    let mut b2: f64 = 0.0_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000002_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000000022_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000000215_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000002176_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000000023720_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000000282505_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000003732996_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000000055853361_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000000973998344_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00000020689392195_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.00000577197245160_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 + -0.00024753706739052_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.02832887813049721_f64;
    b2 = b1; b1 = b0; b0 = twox * b1 - b2 +  0.06379308343739001_f64;
    0.5_f64 * (b0 - b2)
}

// ---------------------------------------------------------------------------
// Public API (K0, K1)
// ---------------------------------------------------------------------------

/// Exponentially scaled modified Bessel function of the second kind, order 0:
/// `exp(x) * K0(x)`. Stays bounded for x > 0.
///
/// Mirrors libxc `xc_bessel_K0_scaled` in `bessel.c`. Domain error for x ≤ 0
/// silently returns 0 (libxc emits stderr; not available in `#[cube]`).
pub fn xc_bessel_K0_scaled(x: f64) -> f64 {
    let mut r: f64 = 0.0_f64;

    if x <= 0.0_f64 {
        // Domain error: leave r = 0.0.
        r = 0.0_f64;
    } else if x <= 2.0_f64 {
        r = rmath::exp(x) * (-rmath::ln(0.5_f64 * x) * xc_bessel_I0(x) - 0.25_f64
            + cheb_bk0(0.5_f64 * x * x - 1.0_f64));
    } else if x <= 8.0_f64 {
        r = (1.25_f64 + cheb_ak0((16.0_f64 / x - 5.0_f64) / 3.0_f64)) / rmath::sqrt(x);
    } else {
        r = (1.25_f64 + cheb_ak02(16.0_f64 / x - 1.0_f64)) / rmath::sqrt(x);
    }

    r
}

/// Modified Bessel function of the second kind, order 0: `K0(x)`.
///
/// Defined for x > 0; `K0(x) → +∞` as `x → 0⁺`. For x ≤ 0 silently returns 0
/// (libxc emits stderr; not available in `#[cube]`).
///
/// Mirrors libxc `xc_bessel_K0` in `bessel.c`.
pub fn xc_bessel_K0(x: f64) -> f64 {
    let mut r: f64 = 0.0_f64;

    if x <= 0.0_f64 {
        // Domain error: leave r = 0.0.
        r = 0.0_f64;
    } else if x <= 2.0_f64 {
        r = -rmath::ln(0.5_f64 * x) * xc_bessel_I0(x) - 0.25_f64
            + cheb_bk0(0.5_f64 * x * x - 1.0_f64);
    } else {
        r = rmath::exp(-x) * xc_bessel_K0_scaled(x);
    }

    r
}

/// Exponentially scaled modified Bessel function of the second kind, order 1:
/// `exp(x) * K1(x)`.
///
/// Mirrors libxc `xc_bessel_K1_scaled` in `bessel.c`.
pub fn xc_bessel_K1_scaled(x: f64) -> f64 {
    let mut r: f64 = 0.0_f64;

    if x <= 0.0_f64 {
        // Domain error: leave r = 0.0.
        r = 0.0_f64;
    } else if x <= 2.0_f64 {
        r = rmath::exp(x) * (rmath::ln(0.5_f64 * x) * xc_bessel_I1(x)
            + (0.75_f64 + cheb_bk1(0.5_f64 * x * x - 1.0_f64)) / x);
    } else if x <= 8.0_f64 {
        r = (1.25_f64 + cheb_ak1((16.0_f64 / x - 5.0_f64) / 3.0_f64)) / rmath::sqrt(x);
    } else {
        r = (1.25_f64 + cheb_ak12(16.0_f64 / x - 1.0_f64)) / rmath::sqrt(x);
    }

    r
}

/// Modified Bessel function of the second kind, order 1: `K1(x)`.
///
/// Defined for x > 0; `K1(x) → +∞` as `x → 0⁺`. For x ≤ 0 or underflow silently
/// returns 0 (libxc emits stderr; not available in `#[cube]`).
///
/// Mirrors libxc `xc_bessel_K1` in `bessel.c`.
pub fn xc_bessel_K1(x: f64) -> f64 {
    let mut r: f64 = 0.0_f64;

    if x <= 0.0_f64 {
        // Domain error: leave r = 0.0.
        r = 0.0_f64;
    } else if x < (TWO_DBL_MIN as f64) {
        // Overflow error: leave r = 0.0.
        r = 0.0_f64;
    } else if x <= 2.0_f64 {
        r = rmath::ln(0.5_f64 * x) * xc_bessel_I1(x)
            + (0.75_f64 + cheb_bk1(0.5_f64 * x * x - 1.0_f64)) / x;
    } else {
        r = rmath::exp(-x) * xc_bessel_K1_scaled(x);
    }

    r
}

// ---------------------------------------------------------------------------
// CPU-side reference tests (run on libxc-cpu backend)
// ---------------------------------------------------------------------------
