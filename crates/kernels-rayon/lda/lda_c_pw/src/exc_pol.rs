//! LDA_C_PW exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw.c`
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
pub fn lda_c_pw_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_a_0: f64,
    param_alpha1_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_beta3_0: f64,
    param_pp_0: f64,
    param_beta4_0: f64,
    param_a_2: f64,
    param_alpha1_2: f64,
    param_beta1_2: f64,
    param_beta2_2: f64,
    param_beta3_2: f64,
    param_pp_2: f64,
    param_beta4_2: f64,
    param_fz20: f64,
    param_a_1: f64,
    param_alpha1_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_beta3_1: f64,
    param_pp_1: f64,
    param_beta4_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_0 = f64x8::splat(param_a_0);
    let param_alpha1_0 = f64x8::splat(param_alpha1_0);
    let param_beta1_0 = f64x8::splat(param_beta1_0);
    let param_beta2_0 = f64x8::splat(param_beta2_0);
    let param_beta3_0 = f64x8::splat(param_beta3_0);
    let param_pp_0 = f64x8::splat(param_pp_0);
    let param_beta4_0 = f64x8::splat(param_beta4_0);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_alpha1_2 = f64x8::splat(param_alpha1_2);
    let param_beta1_2 = f64x8::splat(param_beta1_2);
    let param_beta2_2 = f64x8::splat(param_beta2_2);
    let param_beta3_2 = f64x8::splat(param_beta3_2);
    let param_pp_2 = f64x8::splat(param_pp_2);
    let param_beta4_2 = f64x8::splat(param_beta4_2);
    let param_fz20 = f64x8::splat(param_fz20);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_alpha1_1 = f64x8::splat(param_alpha1_1);
    let param_beta1_1 = f64x8::splat(param_beta1_1);
    let param_beta2_1 = f64x8::splat(param_beta2_1);
    let param_beta3_1 = f64x8::splat(param_beta3_1);
    let param_pp_1 = f64x8::splat(param_pp_1);
    let param_beta4_1 = f64x8::splat(param_beta4_1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t1 = param_a_0;
            let t2 = param_alpha1_0;
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t2 * t3;
            let t5 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t6 = (simd::cbrt(t5));
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = t7 * t7;
            let t9 = t6 * t8;
            let t10 = v_rho0 + v_rho1;
            let t11 = (simd::cbrt(t10));
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t9 * t12;
            let t16 = f64x8::splat(1.0) + t4 * t13 / f64x8::splat(4.0);
            let t18 = f64x8::splat(1.0) / t1;
            let t19 = param_beta1_0;
            let t20 = t3 * t6;
            let t22 = t20 * t8 * t12;
            let t23 = ((t22).sqrt());
            let t27 = param_beta2_0 * t3;
            let t30 = param_beta3_0;
            let t31 = ((t22) * (t22).sqrt());
            let t35 = t22 / f64x8::splat(4.0);
            let t37 = param_pp_0 + f64x8::splat(1.0);
            let t38 = (simd::pow(t35, t37));
            let t39 = param_beta4_0 * t38;
            let t40 = t19 * t23 / f64x8::splat(2.0) + t27 * t13 / f64x8::splat(4.0) + f64x8::splat(0.125) * t30 * t31 + t39;
            let t44 = f64x8::splat(1.0) + t18 / t40 / f64x8::splat(2.0);
            let t45 = (simd::ln(t44));
            let t46 = t1 * t16 * t45;
            let t47 = f64x8::splat(2.0) * t46;
            let t48 = v_rho0 - v_rho1;
            let t49 = t48 * t48;
            let t50 = t49 * t49;
            let t51 = t10 * t10;
            let t52 = t51 * t51;
            let t53 = f64x8::splat(1.0) / t52;
            let t54 = t50 * t53;
            let t55 = f64x8::splat(1.0) / t10;
            let t56 = t48 * t55;
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(zeta_threshold));
            let t60 = t59 * zeta_threshold;
            let t61 = (simd::cbrt(t57));
            let t63 = ((t58).select(t60, t61 * t57));
            let t64 = f64x8::splat(1.0) - t56;
            let t65 = (t64).simd_le(zeta_threshold);
            let t66 = (simd::cbrt(t64));
            let t68 = ((t65).select(t60, t66 * t64));
            let t69 = t63 + t68 - f64x8::splat(2.0);
            let t70 = f64x8::splat(M_CBRT2);
            let t73 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t70 - f64x8::splat(2.0));
            let t74 = t69 * t73;
            let t75 = param_a_1;
            let t76 = param_alpha1_1;
            let t77 = t76 * t3;
            let t80 = f64x8::splat(1.0) + t77 * t13 / f64x8::splat(4.0);
            let t82 = f64x8::splat(1.0) / t75;
            let t83 = param_beta1_1;
            let t87 = param_beta2_1 * t3;
            let t90 = param_beta3_1;
            let t95 = param_pp_1 + f64x8::splat(1.0);
            let t96 = (simd::pow(t35, t95));
            let t97 = param_beta4_1 * t96;
            let t98 = t83 * t23 / f64x8::splat(2.0) + t87 * t13 / f64x8::splat(4.0) + f64x8::splat(0.125) * t90 * t31 + t97;
            let t102 = f64x8::splat(1.0) + t82 / t98 / f64x8::splat(2.0);
            let t103 = (simd::ln(t102));
            let t105 = param_a_2;
            let t106 = param_alpha1_2;
            let t107 = t106 * t3;
            let t110 = f64x8::splat(1.0) + t107 * t13 / f64x8::splat(4.0);
            let t112 = f64x8::splat(1.0) / t105;
            let t113 = param_beta1_2;
            let t117 = param_beta2_2 * t3;
            let t120 = param_beta3_2;
            let t125 = param_pp_2 + f64x8::splat(1.0);
            let t126 = (simd::pow(t35, t125));
            let t127 = param_beta4_2 * t126;
            let t128 = t113 * t23 / f64x8::splat(2.0) + t117 * t13 / f64x8::splat(4.0) + f64x8::splat(0.125) * t120 * t31 + t127;
            let t132 = f64x8::splat(1.0) + t112 / t128 / f64x8::splat(2.0);
            let t133 = (simd::ln(t132));
            let t134 = f64x8::splat(1.0) / param_fz20;
            let t135 = t133 * t134;
            let t138 = -f64x8::splat(2.0) * t75 * t80 * t103 - f64x8::splat(2.0) * t105 * t110 * t135 + f64x8::splat(2.0) * t46;
            let t139 = t74 * t138;
            let t140 = t54 * t139;
            let t143 = t110 * t133 * t134;
            let t145 = f64x8::splat(2.0) * t74 * t105 * t143;
            let tzk0 = -t47 + t140 + t145;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
