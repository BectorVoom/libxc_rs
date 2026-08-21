//! GGA_X_B86 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b86.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_b86_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    param_omega: f64,
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
        let t28 = param_beta * sigma0;
        let t29 = rho0 * rho0;
        let t30 = pow_1_3(rho0);
        let t31 = t30 * t30;
        let t33 = 1.0 / t31 / t29;
        let t36 = param_gamma * sigma0 * t33 + 1.0;
        let t37 = rmath::pow(t36, param_omega);
        let t38 = 1.0 / t37;
        let t41 = t28 * t33 * t38 + 1.0;
        let t45 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t41);
        let t46 = rho1 <= dens_threshold;
        let t47 = -t16;
        let t49 = piecewise5(t14, t11, t10, t15, t47 * t7);
        let t50 = 1.0 + t49;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t54 = piecewise3(t51, t22, t52 * t50);
        let t55 = t54 * t26;
        let t56 = param_beta * sigma2;
        let t57 = rho1 * rho1;
        let t58 = pow_1_3(rho1);
        let t59 = t58 * t58;
        let t61 = 1.0 / t59 / t57;
        let t64 = param_gamma * sigma2 * t61 + 1.0;
        let t65 = rmath::pow(t64, param_omega);
        let t66 = 1.0 / t65;
        let t69 = t56 * t61 * t66 + 1.0;
        let t73 = piecewise3(t46, 0.0, -3.0 / 8.0 * t5 * t55 * t69);
        let tzk0 = t45 + t73;
        zk[ip] += tzk0;
    }
}
