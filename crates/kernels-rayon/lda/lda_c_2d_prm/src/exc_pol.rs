//! LDA_C_2D_PRM exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_prm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI};
use libxc_rkernel_math::powers::{pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_2d_prm_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_N: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = rmath::sqrt(t1);
        let t4 = rmath::sqrt(M_PI);
        let t6 = 3.9274 * t2 + t4 / 2.0;
        let t7 = 1.0 / t6;
        let t8 = t2 * t7;
        let t10 = 3.9274 * t8 - 1.0;
        let t11 = t2 * t10;
        let t12 = 2.0 + param_c;
        let t13 = rmath::sqrt(t12);
        let t14 = 1.0 / t13;
        let t16 = 0.3544538369424879 * t11 * t14;
        let t17 = 1.0 / t12;
        let t18 = t10 * t17;
        let t20 = 0.3999583253029731 * t8 * t18;
        let t21 = t6 * t6;
        let t22 = 1.0 / t21;
        let t24 = 1.0/pow_3_2(t12);
        let t26 = 0.17722691847124394 * t2 * t22 * t24;
        let t27 = 1.0 + param_c;
        let t28 = rmath::sqrt(t27);
        let t29 = 1.0 / t28;
        let t31 = 0.7089076738849758 * t11 * t29;
        let t32 = 1.0 / t27;
        let t34 = 0.3999583253029731 * t8 * t32;
        let tzk0 = t16 + t20 + t26 + t31 + t34;
        zk[ip] += tzk0;
    }
}
