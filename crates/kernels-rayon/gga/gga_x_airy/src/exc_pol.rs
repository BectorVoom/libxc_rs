//! GGA_X_AIRY exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_airy_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = t28 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t34 = rmath::sqrt(sigma0);
        let t35 = pow_1_3(rho0);
        let t37 = 1.0 / t35 / rho0;
        let t39 = t33 * t34 * t37;
        let t40 = rmath::pow(t39, 2.626712);
        let t42 = 1.0 + 0.00013471619689594795 * t40;
        let t43 = rmath::pow(t42, -0.657946);
        let t46 = rmath::pow(t39, 3.217063);
        let t48 = rmath::pow(t39, 3.223476);
        let t50 = 1.0 - 0.04521241301076986 * t46 + 0.04540222195662038 * t48;
        let t51 = rmath::pow(t39, 3.473804);
        let t53 = 1.0 + 0.0004770218022490335 * t51;
        let t54 = 1.0 / t53;
        let t56 = 6.014601922021111e-05 * t40 * t43 + t50 * t54;
        let t60 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t56);
        let t61 = rho1 <= dens_threshold;
        let t62 = -t16;
        let t64 = piecewise5(t14, t11, t10, t15, t62 * t7);
        let t65 = 1.0 + t64;
        let t66 = t65 <= zeta_threshold;
        let t67 = pow_1_3(t65);
        let t69 = piecewise3(t66, t22, t67 * t65);
        let t70 = t69 * t26;
        let t71 = rmath::sqrt(sigma2);
        let t72 = pow_1_3(rho1);
        let t74 = 1.0 / t72 / rho1;
        let t76 = t33 * t71 * t74;
        let t77 = rmath::pow(t76, 2.626712);
        let t79 = 1.0 + 0.00013471619689594795 * t77;
        let t80 = rmath::pow(t79, -0.657946);
        let t83 = rmath::pow(t76, 3.217063);
        let t85 = rmath::pow(t76, 3.223476);
        let t87 = 1.0 - 0.04521241301076986 * t83 + 0.04540222195662038 * t85;
        let t88 = rmath::pow(t76, 3.473804);
        let t90 = 1.0 + 0.0004770218022490335 * t88;
        let t91 = 1.0 / t90;
        let t93 = 6.014601922021111e-05 * t77 * t80 + t87 * t91;
        let t97 = piecewise3(t61, 0.0, -3.0 / 8.0 * t5 * t70 * t93);
        let tzk0 = t60 + t97;
        zk[ip] += tzk0;
    }
}
