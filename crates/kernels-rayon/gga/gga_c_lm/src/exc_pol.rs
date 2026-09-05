//! GGA_C_LM exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lm.c`
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
pub fn gga_c_lm_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_lm_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_lm_f = f64x8::splat(param_lm_f);
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
            let t1 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t6 = f64x8::splat(1.0) + t1 * t3 / f64x8::splat(36000.0);
            let t7 = f64x8::splat(M_CBRT3);
            let t8 = t7 * t7;
            let t9 = (simd::cbrt(t1));
            let t10 = f64x8::splat(1.0) / t9;
            let t11 = t8 * t10;
            let t12 = f64x8::splat(M_CBRT4);
            let t13 = (simd::cbrt(t2));
            let t15 = t11 * t12 * t13;
            let t17 = f64x8::splat(1.0) + f64x8::splat(10.0) * t15;
            let t18 = (simd::ln(t17));
            let t20 = f64x8::splat(0.0252) * t6 * t18;
            let t21 = t9 * t9;
            let t22 = t8 * t21;
            let t23 = t13 * t13;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t12 * t24;
            let t26 = t22 * t25;
            let t27 = f64x8::splat(7e-06) * t26;
            let t28 = t7 * t9;
            let t29 = t12 * t12;
            let t32 = t28 * t29 / t13;
            let t33 = f64x8::splat(0.000105) * t32;
            let t34 = v_rho0 - v_rho1;
            let t35 = t34 * t3;
            let t36 = f64x8::splat(1.0) + t35;
            let t37 = (t36).simd_le(zeta_threshold);
            let t38 = (simd::cbrt(zeta_threshold));
            let t39 = t38 * zeta_threshold;
            let t40 = (simd::cbrt(t36));
            let t41 = t40 * t36;
            let t42 = ((t37).select(t39, t41));
            let t43 = f64x8::splat(1.0) - t35;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = (simd::cbrt(t43));
            let t46 = t45 * t43;
            let t47 = ((t44).select(t39, t46));
            let t49 = f64x8::splat(M_CBRT2);
            let t52 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t49 - f64x8::splat(2.0));
            let t53 = (t42 + t47 - f64x8::splat(2.0)) * t52;
            let t55 = f64x8::splat(1.0) + f64x8::splat(5.658842421045167e-07) * t3;
            let t57 = f64x8::splat(1.0) + f64x8::splat(25.0) * t15;
            let t58 = (simd::ln(t57));
            let t63 = -f64x8::splat(0.0127) * t55 * t58 - f64x8::splat(6.435555555555556e-06) * t26 + f64x8::splat(8.383333333333333e-05) * t32 - f64x8::splat(0.004166666666666667) + t20;
            let t64 = t53 * t63;
            let t65 = f64x8::splat(M_PI) * t8;
            let t66 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t67 = (simd::cbrt(t66));
            let t69 = f64x8::splat(1.0) / t67 / t66;
            let t70 = v_rho0 * v_rho0;
            let t71 = (simd::cbrt(v_rho0));
            let t72 = t71 * t71;
            let t74 = f64x8::splat(1.0) / t72 / t70;
            let t75 = v_sigma0 * t74;
            let t77 = v_rho1 * v_rho1;
            let t78 = (simd::cbrt(v_rho1));
            let t79 = t78 * t78;
            let t81 = f64x8::splat(1.0) / t79 / t77;
            let t82 = v_sigma2 * t81;
            let t87 = t38 * t38;
            let t88 = t87 * zeta_threshold;
            let t89 = t40 * t40;
            let t90 = t89 * t36;
            let t91 = ((t37).select(t88, t90));
            let t92 = t45 * t45;
            let t93 = t92 * t43;
            let t94 = ((t44).select(t88, t93));
            let t95 = t91 + t94;
            let t96 = ((t95).sqrt());
            let t98 = f64x8::splat(M_SQRT2);
            let t99 = f64x8::splat(1.0) / t96 * t98;
            let t100 = t7 * param_lm_f;
            let t101 = (simd::pow(t1, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t102 = f64x8::splat(1.0) / t101;
            let t104 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t105 = ((t104).sqrt());
            let t106 = t102 * t105;
            let t107 = (simd::pow(t2, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t112 = (simd::exp(-t100 * t106 / t107 / t2));
            let t113 = t112 * t104;
            let t114 = t2 * t2;
            let t116 = f64x8::splat(1.0) / t23 / t114;
            let t121 = t69 * (-f64x8::splat(7.0) / f64x8::splat(36.0) * t49 * (t75 * t42 + t82 * t47) + f64x8::splat(2.0) * t99 * t113 * t116);
            let t124 = t65 * t121 * t13 / f64x8::splat(144.0);
            let tzk0 = -t20 + t27 - t33 + f64x8::splat(0.0084) + t64 + t124;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
