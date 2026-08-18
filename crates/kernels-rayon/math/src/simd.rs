//! Explicit-SIMD transcendentals that are **bit-identical to the scalar
//! calls the scalar kernels make**.
//!
//! The SIMD kernels (`docs/perf/simd-kernels.md`) previously used `wide`'s
//! transcendentals, which are ~1 ulp. That was both an accuracy loss (the
//! derivative expressions amplify 1 ulp by orders of magnitude — `gga_c_lyp`
//! measured 4.7e-12 against the scalar kernel, past the project's 1e-12
//! contract) and a verification loss (a SIMD kernel's output fingerprint could
//! never be compared exactly against its scalar form).
//!
//! These functions remove both problems:
//!
//! * [`exp`] and [`ln`] replicate, lane by lane, the exact operation schedule
//!   of glibc's `__ieee754_exp_fma` / `__ieee754_log_fma` — the ifunc variants
//!   every FMA-capable x86-64 machine runs when the scalar kernel calls
//!   `f64::exp` / `f64::ln`. The algorithm and tables are the Szabolcs Nagy
//!   double-precision routines, transcribed from ARM optimized-routines (MIT);
//!   the FMA contraction points were taken from a disassembly of glibc 2.43's
//!   compiled `_fma` variants, so every intermediate rounds identically. Each
//!   IEEE operation (`+ - * /`, fused multiply-add) rounds the same regardless
//!   of vector width, so lane results are bit-identical to the scalar calls.
//! * [`cbrt`] (and the `pow_n_3` family on top of it) replicates
//!   [`crate::powers::cbrt_f64`] — the cube root every scalar kernel uses —
//!   with the identical operation sequence, so it is bit-identical to the
//!   scalar kernels' `POW_1_3` by construction (and therefore sits at the same
//!   1 ulp / 91.5% agreement with glibc's `cbrt` that they do).
//!
//! Consequence: a SIMD kernel whose only transcendentals are `exp`, `ln`,
//! `sqrt` and the cube-root family produces output **bit-identical** to its
//! scalar form, so adding it to `SIMD_FUNCS` is checkable by exact fingerprint
//! comparison and can no longer move the oracle. `atan`/`tanh` still come from
//! `wide` (~1 ulp): glibc's `atan` is the branchy IBM implementation and has
//! not been replicated, so kernels that use it (e.g. `lda_c_vwn`) stay
//! tolerance-checked.
//!
//! Lanes outside a routine's main path (|x| ≥ 512 for `exp`; non-positive,
//! subnormal or non-finite inputs for `ln`) fall back to the scalar libm call
//! for exactly those lanes, keeping bit-identity unconditional. The fast-path
//! test is a lane mask, so grids that never hit the edges pay one vector
//! compare per call.
//!
//! Bit-identity is asserted, not assumed: `tests/simd_exact.rs` sweeps
//! millions of physical and adversarial inputs per function and compares
//! every lane bitwise against the scalar call.

use crate::powers::{CBRT_F_M1, CBRT_F_M2, CBRT_F_P1, CBRT_F_P2};
use crate::simd_data::*;
use wide::{CmpEq, CmpGe, CmpLt, f64x8};

#[inline(always)]
fn lane_bits(v: f64x8) -> [u64; 8] {
    let a: [f64; 8] = v.into();
    a.map(f64::to_bits)
}

#[inline(always)]
fn from_lane_bits(b: [u64; 8]) -> f64x8 {
    f64x8::new(b.map(f64::from_bits))
}

#[inline(always)]
fn splat(v: f64) -> f64x8 {
    f64x8::splat(v)
}

/// `e^x`, bit-identical per lane to glibc's `exp` (`__ieee754_exp_fma`).
///
/// Main path covers |x| in [2^-54, 512); |x| < 2^-54 returns `1.0 + x` as
/// glibc does; anything else (overflow range, ±inf, NaN) is patched with the
/// scalar `f64::exp` for those lanes only.
#[inline(always)]
pub fn exp(x: f64x8) -> f64x8 {
    let shift = splat(EXP_SHIFT);
    // glibc contracts `z = InvLn2N*x; kd = z + Shift` into one fma.
    let kd_s = x.mul_add(splat(EXP_INVLN2N), shift);
    let ki = lane_bits(kd_s);
    let kd = kd_s - shift;
    let r = kd.mul_add(splat(EXP_NEGLN2LON), kd.mul_add(splat(EXP_NEGLN2HIN), x));

    let mut tail_b = [0u64; 8];
    let mut sbits = [0u64; 8];
    for l in 0..8 {
        let idx = ((ki[l] & 127) * 2) as usize;
        tail_b[l] = EXP_TAB[idx];
        sbits[l] = EXP_TAB[idx + 1].wrapping_add(ki[l] << 45);
    }
    let tail = from_lane_bits(tail_b);

    let p12 = r.mul_add(splat(EXP_C3), splat(EXP_C2));
    let t3 = tail + r;
    let r2 = r * r;
    let p45 = r.mul_add(splat(EXP_C5), splat(EXP_C4));
    let s1 = r2.mul_add(p12, t3);
    let r4 = r2 * r2;
    let tmp = r4.mul_add(p45, s1);
    let scale = from_lane_bits(sbits);
    let main = scale.mul_add(tmp, scale);

    let ax = x.abs();
    // false for NaN and |x| >= 512 — exactly glibc's `abstop` range test.
    let m_ok = ax.simd_lt(splat(512.0));
    let m_tiny = ax.simd_lt(splat(f64::from_bits(0x3c90000000000000))); // 2^-54
    if m_ok.all() {
        if !m_tiny.any() {
            return main;
        }
        return m_tiny.select(splat(1.0) + x, main);
    }
    let mut out: [f64; 8] = m_tiny.select(splat(1.0) + x, main).into();
    let xs: [f64; 8] = x.into();
    let okb = lane_bits(m_ok);
    for l in 0..8 {
        if okb[l] == 0 {
            out[l] = xs[l].exp();
        }
    }
    f64x8::new(out)
}

/// `ln(x)`, bit-identical per lane to glibc's `log` (`__ieee754_log_fma`).
///
/// Both glibc paths — the table-based main path and the near-1.0 polynomial —
/// are evaluated vectorised and blended by lane mask, so inputs on either
/// side of 1.0 stay on the fast path. Non-positive, subnormal and non-finite
/// lanes are patched with the scalar `f64::ln`.
#[inline(always)]
pub fn ln(x: f64x8) -> f64x8 {
    const OFF: u64 = 0x3fe6000000000000;
    let xb = lane_bits(x);

    let mut invc_a = [0.0f64; 8];
    let mut logc_a = [0.0f64; 8];
    let mut z_b = [0u64; 8];
    let mut kd_a = [0.0f64; 8];
    for l in 0..8 {
        let ix = xb[l];
        let tmp = ix.wrapping_sub(OFF);
        let i = ((tmp >> 45) & 127) as usize;
        let k = (tmp as i64) >> 52;
        z_b[l] = ix.wrapping_sub(tmp & (0xfffu64 << 52));
        invc_a[l] = LOG_TAB[2 * i];
        logc_a[l] = LOG_TAB[2 * i + 1];
        kd_a[l] = k as f64;
    }
    let z = from_lane_bits(z_b);
    let kd = f64x8::new(kd_a);

    // Main path. FMA placement matches the disassembly of __ieee754_log_fma.
    let w = kd.mul_add(splat(LOG_LN2HI), f64x8::new(logc_a));
    let r = z.mul_add(f64x8::new(invc_a), splat(-1.0));
    let q12 = r.mul_add(splat(LOG_A2), splat(LOG_A1));
    let hi = r + w;
    let r2 = r * r;
    let t = (w - hi) + r;
    let lo = kd.mul_add(splat(LOG_LN2LO), t);
    let r3 = r * r2;
    let q34 = r.mul_add(splat(LOG_A4), splat(LOG_A3));
    let s1 = r2.mul_add(splat(LOG_A0), lo);
    let q = r2.mul_add(q34, q12);
    let y_main = r3.mul_add(q, s1) + hi;

    // Lane classification, bit-equivalent to glibc's integer range tests.
    let m_near = x.simd_ge(splat(0.9375)) & x.simd_lt(splat(f64::from_bits(0x3ff1090000000000)));
    let m_ok = x.simd_ge(splat(f64::MIN_POSITIVE)) & x.simd_lt(splat(f64::INFINITY));

    // Near-1.0 path (0.9375 <= x < 1.0 + 0x1.09p-4), computed only when some
    // lane is in the window — it is ~12 extra FMAs, and kernel arguments are
    // rarely there. `rr * 0x1p27` is exact, so GCC's contractions in the
    // Veltkamp split leave the values unchanged; the schedule below is still
    // copied from the disassembly.
    let y = if !m_near.any() {
        y_main
    } else {
        let rr = x - splat(1.0);
        let p12b = rr.mul_add(splat(LOG_B2), splat(LOG_B1));
        let p45b = rr.mul_add(splat(LOG_B5), splat(LOG_B4));
        let rr2 = rr * rr;
        let p78 = rr.mul_add(splat(LOG_B8), splat(LOG_B7));
        let p123 = rr2.mul_add(splat(LOG_B3), p12b);
        let p456 = rr2.mul_add(splat(LOG_B6), p45b);
        let rr3 = rr * rr2;
        let p789 = rr2.mul_add(splat(LOG_B9), p78);
        let p78910 = rr3.mul_add(splat(LOG_B10), p789);
        let pin = p78910.mul_add(rr3, p456);
        let poly = pin.mul_add(rr3, p123);
        let c27 = splat(134217728.0); // 0x1p27
        let rhi_t = rr.mul_add(c27, rr);
        let rhi = rr.mul_neg_add(c27, rhi_t);
        let rlo = rr - rhi;
        let s = rhi * rhi;
        let hi_b = s.mul_add(splat(LOG_B0), rr);
        let t8 = rr - hi_b;
        let rpr = rr + rhi;
        let lo_b = s.mul_add(splat(LOG_B0), t8);
        let lo2 = (splat(LOG_B0) * rlo).mul_add(rpr, lo_b);
        let y_near = poly.mul_add(rr3, lo2) + hi_b;
        m_near.select(y_near, y_main)
    };
    if m_ok.all() {
        return y;
    }
    let mut out: [f64; 8] = y.into();
    let xs: [f64; 8] = x.into();
    let okb = lane_bits(m_ok);
    for l in 0..8 {
        if okb[l] == 0 {
            out[l] = xs[l].ln();
        }
    }
    f64x8::new(out)
}

/// Cube root, bit-identical per lane to the scalar kernels'
/// [`crate::powers::cbrt_f64`]: the identical operation sequence, evaluated
/// eight lanes at a time. (Not glibc's `cbrt` — the scalar kernels don't call
/// that either; keeping the two trees identical is what makes a SIMD kernel's
/// fingerprint exactly comparable.)
#[inline(always)]
pub fn cbrt(x: f64x8) -> f64x8 {
    let a = x.abs();
    let bits = lane_bits(a);
    // frexp, with subnormals rescaled by 2^54 exactly as the scalar code does.
    let scaled_b = lane_bits(a * splat(18014398509481984.0)); // 2^54
    let mut xm_b = [0u64; 8];
    let mut fac_b = [0u64; 8];
    let mut pow2_b = [0u64; 8];
    // Branchless per-lane integer work: lane branches (the subnormal pick, the
    // 5-way `fac` chain) mispredict and serialise; as mask arithmetic LLVM
    // keeps the whole fixed-8 loop in vector registers.
    for l in 0..8 {
        let b = bits[l];
        let raw = (b >> 52) & 0x7ff;
        let m_sub = ((raw == 0) as u64).wrapping_neg();
        let bu = (scaled_b[l] & m_sub) | (b & !m_sub);
        let raw_u = ((bu >> 52) & 0x7ff) as i64 - ((54u64 & m_sub) as i64);
        xm_b[l] = (bu & 0x800f_ffff_ffff_ffff) | (1022u64 << 52);
        let xe = raw_u - 1022;
        // Truncating /3 and %3 (i32 semantics of the scalar code) via
        // floor-division: xe is in [-1128, 1024], so (xe + 1077)/3 is a small
        // non-negative division LLVM lowers to a vector multiply-shift.
        let qf = (xe + 1077) / 3 - 359; // floor(xe/3)
        let rf = xe - 3 * qf; // {0, 1, 2}
        let m_neg_adj = (((xe < 0) & (rf != 0)) as i64).wrapping_neg() as u64;
        let q = qf + (1 & m_neg_adj) as i64; // trunc(xe/3)
        let r = rf - 3 * (1 & m_neg_adj) as i64; // xe % 3, in {-2..2}
        // fac by equality masks over the five cases.
        let me = |v: i64| ((r == v) as u64).wrapping_neg();
        fac_b[l] = (CBRT_F_M2.to_bits() & me(-2))
            | (CBRT_F_M1.to_bits() & me(-1))
            | (1.0f64.to_bits() & me(0))
            | (CBRT_F_P1.to_bits() & me(1))
            | (CBRT_F_P2.to_bits() & me(2));
        pow2_b[l] = ((q + 1023) as u64) << 52;
    }
    let xm = from_lane_bits(xm_b);
    let fac = from_lane_bits(fac_b);

    // Degree-6 minimax seed; plain mul/add chain in the scalar association
    // (Rust never contracts, so this rounds identically to cbrt_f64).
    let u = splat(0.354895765043919860)
        + (splat(1.50819193781584896)
            + (splat(-2.11499494167371287)
                + (splat(2.44693122563534430)
                    + (splat(-1.83469277483613086)
                        + (splat(0.784932344976639262) - splat(0.145263899385486377) * xm)
                            * xm)
                        * xm)
                    * xm)
                * xm)
            * xm;

    let t2 = u * u * u;
    let ym = u * (t2 + splat(2.0) * xm) / (splat(2.0) * t2 + xm) * fac;
    let y0 = ym * from_lane_bits(pow2_b);

    let t = y0 * y0;
    let err = t.mul_add(y0, -a);
    let y = y0 - err / (splat(3.0) * t);

    let signed = x.simd_lt(splat(0.0)).select(-y, y);
    let degenerate = x.simd_eq(splat(0.0)) | x.is_nan() | x.is_inf();
    degenerate.select(x + x, signed)
}

/// x^(2/3), the scalar `pow_2_3` sequence over [`cbrt`].
#[inline(always)]
pub fn pow_2_3(x: f64x8) -> f64x8 {
    let c = cbrt(x);
    c * c
}

/// x^(4/3), the scalar `pow_4_3` sequence over [`cbrt`].
#[inline(always)]
pub fn pow_4_3(x: f64x8) -> f64x8 {
    x * cbrt(x)
}

/// x^(5/3), the scalar `pow_5_3` sequence over [`cbrt`].
#[inline(always)]
pub fn pow_5_3(x: f64x8) -> f64x8 {
    let c = cbrt(x);
    x * c * c
}

/// x^(7/3), the scalar `pow_7_3` sequence over [`cbrt`].
#[inline(always)]
pub fn pow_7_3(x: f64x8) -> f64x8 {
    x * x * cbrt(x)
}
