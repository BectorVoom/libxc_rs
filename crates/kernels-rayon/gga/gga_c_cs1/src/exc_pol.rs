//! GGA_C_CS1 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_cs1.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_cs1_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = t1 * t1;
            let t3 = v_rho0 + v_rho1;
            let t4 = t3 * t3;
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = -t2 * t5 + f64x8::splat(1.0);
            let t8 = (simd::cbrt(t3));
            let t9 = f64x8::splat(1.0) / t8;
            let t11 = f64x8::splat(1.0) + f64x8::splat(0.349) * t9;
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t7 * t12;
            let t15 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t16 = t15 * t15;
            let t17 = t4 * t4;
            let t18 = t17 * t3;
            let t20 = f64x8::splat(1.0) / t8 / t18;
            let t22 = t8 * t8;
            let t24 = f64x8::splat(1.0) / t22 / t4;
            let t27 = f64x8::splat(1.0) + f64x8::splat(0.006) * t15 * t24;
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t32 = -f64x8::splat(0.159068) + f64x8::splat(2.86308e-07) * t16 * t20 * t29;
            let t34 = t13 * t32 / f64x8::splat(4.0);
            let t35 = f64x8::splat(1.0) / t3;
            let t36 = t1 * t35;
            let t37 = f64x8::splat(1.0) + t36;
            let t38 = (t37).simd_le(zeta_threshold);
            let t39 = ((t38).select(zeta_threshold, t37));
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t39 * t40;
            let t42 = t40 + f64x8::splat(0.349);
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = v_sigma0 * v_sigma0;
            let t45 = v_rho0 * v_rho0;
            let t46 = t45 * t45;
            let t47 = t46 * v_rho0;
            let t49 = f64x8::splat(1.0) / t40 / t47;
            let t51 = t40 * t40;
            let t53 = f64x8::splat(1.0) / t51 / t45;
            let t56 = f64x8::splat(1.0) + f64x8::splat(0.006) * v_sigma0 * t53;
            let t57 = t56 * t56;
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = -f64x8::splat(0.018897) + f64x8::splat(5.58864e-06) * t44 * t49 * t58;
            let t62 = t43 * t61;
            let t64 = t41 * t62 / f64x8::splat(2.0);
            let t65 = f64x8::splat(1.0) - t36;
            let t66 = (t65).simd_le(zeta_threshold);
            let t67 = ((t66).select(zeta_threshold, t65));
            let t68 = (simd::cbrt(v_rho1));
            let t69 = t67 * t68;
            let t70 = t68 + f64x8::splat(0.349);
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = v_sigma2 * v_sigma2;
            let t73 = v_rho1 * v_rho1;
            let t74 = t73 * t73;
            let t75 = t74 * v_rho1;
            let t77 = f64x8::splat(1.0) / t68 / t75;
            let t79 = t68 * t68;
            let t81 = f64x8::splat(1.0) / t79 / t73;
            let t84 = f64x8::splat(1.0) + f64x8::splat(0.006) * v_sigma2 * t81;
            let t85 = t84 * t84;
            let t86 = f64x8::splat(1.0) / t85;
            let t89 = -f64x8::splat(0.018897) + f64x8::splat(5.58864e-06) * t72 * t77 * t86;
            let t90 = t71 * t89;
            let t92 = t69 * t90 / f64x8::splat(2.0);
            let tzk0 = t34 + t64 + t92;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
