//! LDA_C_PZ exc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_pz_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t1 * t3 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = (f64x8::splat(1.0)).simd_le(t11);
            let t13 = param_gamma_0;
            let t14 = param_beta1_0;
            let t15 = ((t10).sqrt());
            let t19 = param_beta2_0 * t1;
            let t20 = t3 * t6;
            let t21 = t20 * t8;
            let t24 = f64x8::splat(1.0) + t14 * t15 / f64x8::splat(2.0) + t19 * t21 / f64x8::splat(4.0);
            let t27 = param_a_0;
            let t28 = (simd::ln(t11));
            let t32 = param_c_0 * t1;
            let t33 = t32 * t3;
            let t34 = t9 * t28;
            let t38 = param_d_0 * t1;
            let t42 = ((t12).select(t13 / t24, t27 * t28 + param_b_0 + t33 * t34 / f64x8::splat(4.0) + t38 * t21 / f64x8::splat(4.0)));
            let t43 = param_gamma_1;
            let t44 = param_beta1_1;
            let t48 = param_beta2_1 * t1;
            let t51 = f64x8::splat(1.0) + t44 * t15 / f64x8::splat(2.0) + t48 * t21 / f64x8::splat(4.0);
            let t54 = param_a_1;
            let t58 = param_c_1 * t1;
            let t59 = t58 * t3;
            let t63 = param_d_1 * t1;
            let t67 = ((t12).select(t43 / t51, t54 * t28 + param_b_1 + t59 * t34 / f64x8::splat(4.0) + t63 * t21 / f64x8::splat(4.0)));
            let t70 = (simd::cbrt(zeta_threshold));
            let t72 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t70 * zeta_threshold, f64x8::splat(1.0)));
            let t74 = f64x8::splat(2.0) * t72 - f64x8::splat(2.0);
            let t76 = f64x8::splat(M_CBRT2);
            let t79 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t76 - f64x8::splat(2.0));
            let t80 = (t67 - t42) * t74 * t79;
            let tzk0 = t42 + t80;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
