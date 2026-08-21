//! MGGA_C_B88 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_b88.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_b88_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = 1.0 / M_PI;
        let t7 = pow_1_3(t6);
        let t8 = 1.0 / t7;
        let t9 = t5 * t8;
        let t10 = M_CBRT4;
        let t11 = t9 * t10;
        let t12 = M_CBRT2;
        let t13 = 1.0 <= zeta_threshold;
        let t14 = zeta_threshold - 1.0;
        let t16 = piecewise5(t13, t14, t13, -t14, 0.0);
        let t17 = 1.0 + t16;
        let t18 = t17 * rho[ip];
        let t19 = pow_1_3(t18);
        let t20 = 1.0 / t19;
        let t21 = t12 * t20;
        let t22 = t12 * t12;
        let t23 = sigma[ip] * t22;
        let t24 = rho[ip] * rho[ip];
        let t25 = pow_1_3(rho[ip]);
        let t26 = t25 * t25;
        let t28 = 1.0 / t26 / t24;
        let t29 = t23 * t28;
        let t31 = 1.0 + 0.007 * t29;
        let t32 = rmath::pow(t31, 1.0 / 5.0);
        let t33 = t32 * t32;
        let t34 = t33 * t33;
        let t35 = 1.0 / t34;
        let t40 = 1.0 + 0.0008333333333333334 * t11 * t23 * t28 * t35;
        let t41 = 1.0 / t40;
        let t43 = t11 * t21 * t41;
        let t45 = piecewise3(t3, 0.0, t43 / 9.0);
        let t46 = rho[ip] * t45;
        let t47 = 1.26 * t45;
        let t48 = 1.0 + t47;
        let t49 = rmath::ln(t48);
        let t50 = t47 - t49;
        let t52 = 0.252 * t46 * t50;
        let t53 = t17 * t17;
        let t54 = pow_1_3(t17);
        let t55 = t54 * t54;
        let t56 = t55 * t53;
        let t57 = t56 * t22;
        let t58 = t26 * rho[ip];
        let t59 = tau[ip] * t22;
        let t64 = 2.0 * t59 / t58 - t29 / 4.0;
        let t66 = t58 * t64 * t5;
        let t67 = t57 * t66;
        let t69 = 1.0 / t7 / t6;
        let t70 = t69 * t10;
        let t72 = 1.0 / t19 / t18;
        let t73 = t40 * t40;
        let t74 = t73 * t73;
        let t75 = 1.0 / t74;
        let t76 = t72 * t75;
        let t78 = 1.0 + 0.10666666666666667 * t43;
        let t79 = rmath::ln(t78);
        let t80 = t79 * t4;
        let t81 = t80 * t7;
        let t82 = t10 * t10;
        let t83 = t82 * t22;
        let t84 = t19 * t40;
        let t85 = t83 * t84;
        let t88 = 1.0 - 0.390625 * t81 * t85;
        let t90 = t70 * t76 * t88;
        let t93 = piecewise3(t3, 0.0, -0.0001864135111111111 * t67 * t90);
        let t94 = 2.0 * t93;
        let tzk0 = -t52 + t94;
        zk[ip] += tzk0;
    }
}
