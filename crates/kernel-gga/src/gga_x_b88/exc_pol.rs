//! GGA_X_B88 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b88.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_b88_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t28 = t2 * t2;
        let t29 = param_beta * t28;
        let t31 = pow_1_3(1.0 / M_PI);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t34 = M_CBRT4;
        let t35 = t34 * sigma0;
        let t36 = rho0 * rho0;
        let t37 = pow_1_3(rho0);
        let t38 = t37 * t37;
        let t40 = 1.0 / t38 / t36;
        let t41 = param_gamma * param_beta;
        let t42 = f64::sqrt(sigma0);
        let t44 = 1.0 / t37 / rho0;
        let t45 = t42 * t44;
        let t46 = f64::ln(t45 + f64::sqrt(t45 * t45 + 1.0));
        let t49 = t41 * t45 * t46 + 1.0;
        let t50 = 1.0 / t49;
        let t55 = 1.0 + 2.0 / 9.0 * t33 * t35 * t40 * t50;
        let t59 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t16;
        let t63 = piecewise5(t14, t11, t10, t15, t61 * t7);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t22, t66 * t64);
        let t69 = t68 * t26;
        let t70 = t34 * sigma2;
        let t71 = rho1 * rho1;
        let t72 = pow_1_3(rho1);
        let t73 = t72 * t72;
        let t75 = 1.0 / t73 / t71;
        let t76 = f64::sqrt(sigma2);
        let t78 = 1.0 / t72 / rho1;
        let t79 = t76 * t78;
        let t80 = f64::ln(t79 + f64::sqrt(t79 * t79 + 1.0));
        let t83 = t41 * t79 * t80 + 1.0;
        let t84 = 1.0 / t83;
        let t89 = 1.0 + 2.0 / 9.0 * t33 * t70 * t75 * t84;
        let t93 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t89);
        let tzk0 = t59 + t93;
        zk[ip] += tzk0;
    }
}
