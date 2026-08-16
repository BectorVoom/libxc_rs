//! LDA_C_2D_PRM exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_prm.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI};
use libxc_rkernel_math::powers::{pow_3_2};

/// LDA_C_2D_PRM exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_2d_prm_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = f64::sqrt(rho[ip]);
        let t3 = f64::sqrt(M_PI);
        let t5 = 3.9274 * t1 + t3 / 2.0;
        let t6 = 1.0 / t5;
        let t7 = t1 * t6;
        let t9 = 3.9274 * t7 - 1.0;
        let t10 = t1 * t9;
        let t11 = 2.0 + param_c;
        let t12 = f64::sqrt(t11);
        let t13 = 1.0 / t12;
        let t15 = 0.3544538369424879 * t10 * t13;
        let t16 = 1.0 / t11;
        let t17 = t9 * t16;
        let t19 = 0.3999583253029731 * t7 * t17;
        let t20 = t5 * t5;
        let t21 = 1.0 / t20;
        let t23 = 1.0/pow_3_2(t11);
        let t25 = 0.17722691847124394 * t1 * t21 * t23;
        let t26 = 1.0 + param_c;
        let t27 = f64::sqrt(t26);
        let t28 = 1.0 / t27;
        let t30 = 0.7089076738849758 * t10 * t28;
        let t31 = 1.0 / t26;
        let t33 = 0.3999583253029731 * t7 * t31;
        let tzk0 = t15 + t19 + t25 + t30 + t33;
        zk[ip] += tzk0;
    }
}
