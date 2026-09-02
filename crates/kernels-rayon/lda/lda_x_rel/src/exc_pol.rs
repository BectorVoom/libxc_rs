//! LDA_X_REL exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_rel.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_rel_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t8 = rho0 * t7;
        let t10 = 2.0 * t8 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t12 = t11 * zeta_threshold;
        let t13 = M_CBRT2;
        let t14 = t13 * rho0;
        let t15 = pow_1_3(t8);
        let t19 = piecewise3(t10, t12, 2.0 * t14 * t7 * t15);
        let t20 = pow_1_3(t6);
        let t24 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t19 * t20);
        let t25 = rho1 <= dens_threshold;
        let t26 = rho1 * t7;
        let t28 = 2.0 * t26 <= zeta_threshold;
        let t29 = t13 * rho1;
        let t30 = pow_1_3(t26);
        let t34 = piecewise3(t28, t12, 2.0 * t29 * t7 * t30);
        let t38 = piecewise3(t25, 0.0, -3.0 / 8.0 * t5 * t34 * t20);
        let t39 = t24 + t38;
        let t40 = pow_1_3(9.0);
        let t41 = t40 * t40;
        let t42 = t41 * t2;
        let t43 = 1.0 / M_PI;
        let t44 = pow_1_3(t43);
        let t45 = t44 * t44;
        let t46 = 1.0 / t45;
        let t47 = t20 * t20;
        let t51 = 1.0 + 3.8075239991386495e-05 * t42 * t46 * t47;
        let t52 = rmath::sqrt(t51);
        let t53 = t52 * t41;
        let t54 = t2 * t44;
        let t59 = t2 * t2;
        let t60 = t40 * t59;
        let t61 = 1.0 / t44;
        let t65 = rmath::ln(0.0035625477770544352 * t60 * t61 * t20 + rmath::sqrt(pow_2(0.0035625477770544352 * t60 * t61 * t20) + 1.0));
        let t66 = t65 * t40;
        let t67 = t59 * t45;
        let t68 = 1.0 / t47;
        let t72 = 10.396221848752237 * t53 * t54 / t20 - 972.7328585562606 * t66 * t67 * t68;
        let t73 = t72 * t72;
        let t75 = 1.0 - 1.5 * t73;
        let tzk0 = t39 * t75;
        zk[ip] += tzk0;
    }
}
