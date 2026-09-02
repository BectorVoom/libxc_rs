//! LDA_K_ZLP fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_zlp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_k_zlp_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t10 = pow_1_3(zeta_threshold);
        let t11 = t10 * t10;
        let t13 = piecewise3(1.0 <= zeta_threshold, t11 * zeta_threshold, 1.0);
        let t14 = pow_1_3(rho[ip]);
        let t15 = t14 * t14;
        let t16 = t13 * t15;
        let t17 = 1.0 / t14;
        let t19 = 1.0 + 510.2040816326531 * t17;
        let t20 = rmath::ln(t19);
        let t23 = 1.0 - 0.00196 * t14 * t20;
        let t25 = t8 * t16 * t23;
        let tzk0 = 1.0790666666666666 * t25;
        zk[ip] += tzk0;
        let t27 = t15 * rho[ip];
        let t29 = t27 * t2 * t5;
        let t30 = t7 * t13;
        let t35 = 1.0 / t19;
        let t38 = -0.0006533333333333333 / t15 * t20 + 0.3333333333333333 / rho[ip] * t35;
        let tvrho0 = 1.7984444444444445 * t25 + 1.0790666666666666 * t29 * t30 * t38;
        vrho[ip] += tvrho0;
        let t42 = t13 * t17;
        let t52 = rho[ip] * rho[ip];
        let t57 = 1.0 / t14 / t52;
        let t58 = t19 * t19;
        let t59 = 1.0 / t58;
        let t62 = 0.00043555555555555557 / t27 * t20 - 0.2222222222222222 / t52 * t35 + 56.68934240362812 * t57 * t59;
        let tv2rho20 = 1.198962962962963 * t8 * t42 * t23 + 3.596888888888889 * t8 * t16 * t38 + 1.0790666666666666 * t29 * t30 * t62;
        v2rho2[ip] += tv2rho20;
    }
}
