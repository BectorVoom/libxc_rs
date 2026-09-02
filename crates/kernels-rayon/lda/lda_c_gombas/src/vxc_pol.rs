//! LDA_C_GOMBAS vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gombas.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_gombas_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t3 = 1.0 / t2;
        let t5 = 1.0 + 0.0562 * t3;
        let t7 = 0.0357 / t5;
        let t8 = t3 + 2.39;
        let t10 = rmath::ln(t8 * t2);
        let t11 = 0.0311 * t10;
        let tzk0 = -t7 - t11;
        zk[ip] += tzk0;
        let t12 = t5 * t5;
        let t13 = 1.0 / t12;
        let t15 = 1.0 / t2 / t1;
        let t16 = t13 * t15;
        let t19 = t2 * t2;
        let t23 = -1.0 / t1 / 3.0 + t8 / t19 / 3.0;
        let t24 = 1.0 / t8;
        let t25 = t23 * t24;
        let t26 = t25 * t3;
        let tvrho0 = -t7 - t11 + t1 * (-0.00066878 * t16 - 0.0311 * t26);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
