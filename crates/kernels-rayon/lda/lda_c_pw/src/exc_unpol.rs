//! LDA_C_PW exc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_pw_exc_unpol(
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
        let v_rho = load(rho, ip, np);
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
            let t10 = (simd::cbrt(v_rho));
            let t11 = f64x8::splat(1.0) / t10;
            let t12 = t9 * t11;
            let t15 = f64x8::splat(1.0) + t4 * t12 / f64x8::splat(4.0);
            let t17 = f64x8::splat(1.0) / t1;
            let t18 = param_beta1_0;
            let t19 = t3 * t6;
            let t21 = t19 * t8 * t11;
            let t22 = ((t21).sqrt());
            let t26 = param_beta2_0 * t3;
            let t29 = param_beta3_0;
            let t30 = ((t21) * (t21).sqrt());
            let t34 = t21 / f64x8::splat(4.0);
            let t36 = param_pp_0 + f64x8::splat(1.0);
            let t37 = (simd::pow(t34, t36));
            let t38 = param_beta4_0 * t37;
            let t39 = t18 * t22 / f64x8::splat(2.0) + t26 * t12 / f64x8::splat(4.0) + f64x8::splat(0.125) * t29 * t30 + t38;
            let t43 = f64x8::splat(1.0) + t17 / t39 / f64x8::splat(2.0);
            let t44 = (simd::ln(t43));
            let t45 = t1 * t15 * t44;
            let t47 = (simd::cbrt(zeta_threshold));
            let t49 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t47 * zeta_threshold, f64x8::splat(1.0)));
            let t52 = f64x8::splat(M_CBRT2);
            let t56 = (f64x8::splat(2.0) * t49 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t52 - f64x8::splat(2.0));
            let t57 = param_a_2;
            let t59 = param_alpha1_2;
            let t60 = t59 * t3;
            let t63 = f64x8::splat(1.0) + t60 * t12 / f64x8::splat(4.0);
            let t64 = f64x8::splat(1.0) / t57;
            let t65 = param_beta1_2;
            let t69 = param_beta2_2 * t3;
            let t72 = param_beta3_2;
            let t77 = param_pp_2 + f64x8::splat(1.0);
            let t78 = (simd::pow(t34, t77));
            let t79 = param_beta4_2 * t78;
            let t80 = t65 * t22 / f64x8::splat(2.0) + t69 * t12 / f64x8::splat(4.0) + f64x8::splat(0.125) * t72 * t30 + t79;
            let t84 = f64x8::splat(1.0) + t64 / t80 / f64x8::splat(2.0);
            let t85 = (simd::ln(t84));
            let t87 = f64x8::splat(1.0) / param_fz20;
            let t89 = t56 * t57 * t63 * t85 * t87;
            let tzk0 = -f64x8::splat(2.0) * t45 + f64x8::splat(2.0) * t89;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
