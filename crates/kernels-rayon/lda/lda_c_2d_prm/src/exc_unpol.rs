//! LDA_C_2D_PRM exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_prm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI};
use libxc_rkernel_math::powers::{pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_2d_prm_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_N: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = rmath::sqrt(rho[ip]);
        let t3 = rmath::sqrt(M_PI);
        let t5 = 3.9274 * t1 + t3 / 2.0;
        let t6 = 1.0 / t5;
        let t7 = t1 * t6;
        let t9 = 3.9274 * t7 - 1.0;
        let t10 = t1 * t9;
        let t11 = 2.0 + param_c;
        let t12 = rmath::sqrt(t11);
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
        let t27 = rmath::sqrt(t26);
        let t28 = 1.0 / t27;
        let t30 = 0.7089076738849758 * t10 * t28;
        let t31 = 1.0 / t26;
        let t33 = 0.3999583253029731 * t7 * t31;
        let tzk0 = t15 + t19 + t25 + t30 + t33;
        zk[ip] += tzk0;
    }
}
