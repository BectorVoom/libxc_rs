//! LDA_C_PZ exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pz.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_pz_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_gamma_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_a_0: f64,
    param_c_0: f64,
    param_d_0: f64,
    param_b_0: f64,
    param_gamma_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_a_1: f64,
    param_c_1: f64,
    param_d_1: f64,
    param_b_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma_0 = f64x8::splat(param_gamma_0);
    let param_beta1_0 = f64x8::splat(param_beta1_0);
    let param_beta2_0 = f64x8::splat(param_beta2_0);
    let param_a_0 = f64x8::splat(param_a_0);
    let param_c_0 = f64x8::splat(param_c_0);
    let param_d_0 = f64x8::splat(param_d_0);
    let param_b_0 = f64x8::splat(param_b_0);
    let param_gamma_1 = f64x8::splat(param_gamma_1);
    let param_beta1_1 = f64x8::splat(param_beta1_1);
    let param_beta2_1 = f64x8::splat(param_beta2_1);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_c_1 = f64x8::splat(param_c_1);
    let param_d_1 = f64x8::splat(param_d_1);
    let param_b_1 = f64x8::splat(param_b_1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t1 * t3 * t10;
            let t12 = t11 / f64x8::splat(4.0);
            let t13 = (f64x8::splat(1.0)).simd_le(t12);
            let t14 = param_gamma_0;
            let t15 = param_beta1_0;
            let t16 = ((t11).sqrt());
            let t20 = param_beta2_0 * t1;
            let t21 = t3 * t6;
            let t22 = t21 * t9;
            let t25 = f64x8::splat(1.0) + t15 * t16 / f64x8::splat(2.0) + t20 * t22 / f64x8::splat(4.0);
            let t28 = param_a_0;
            let t29 = (simd::ln(t12));
            let t33 = param_c_0 * t1;
            let t34 = t33 * t3;
            let t35 = t10 * t29;
            let t39 = param_d_0 * t1;
            let t43 = ((t13).select(t14 / t25, t28 * t29 + param_b_0 + t34 * t35 / f64x8::splat(4.0) + t39 * t22 / f64x8::splat(4.0)));
            let t44 = param_gamma_1;
            let t45 = param_beta1_1;
            let t49 = param_beta2_1 * t1;
            let t52 = f64x8::splat(1.0) + t45 * t16 / f64x8::splat(2.0) + t49 * t22 / f64x8::splat(4.0);
            let t55 = param_a_1;
            let t59 = param_c_1 * t1;
            let t60 = t59 * t3;
            let t64 = param_d_1 * t1;
            let t68 = ((t13).select(t44 / t52, t55 * t29 + param_b_1 + t60 * t35 / f64x8::splat(4.0) + t64 * t22 / f64x8::splat(4.0)));
            let t69 = t68 - t43;
            let t70 = v_rho0 - v_rho1;
            let t71 = f64x8::splat(1.0) / t7;
            let t72 = t70 * t71;
            let t73 = f64x8::splat(1.0) + t72;
            let t74 = (t73).simd_le(zeta_threshold);
            let t75 = (simd::cbrt(zeta_threshold));
            let t76 = t75 * zeta_threshold;
            let t77 = (simd::cbrt(t73));
            let t79 = ((t74).select(t76, t77 * t73));
            let t80 = f64x8::splat(1.0) - t72;
            let t81 = (t80).simd_le(zeta_threshold);
            let t82 = (simd::cbrt(t80));
            let t84 = ((t81).select(t76, t82 * t80));
            let t85 = t79 + t84 - f64x8::splat(2.0);
            let t87 = f64x8::splat(M_CBRT2);
            let t90 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t87 - f64x8::splat(2.0));
            let t91 = t69 * t85 * t90;
            let tzk0 = t43 + t91;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
