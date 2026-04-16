//! GGA_X_OL2 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ol2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ol2_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
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
        let t28 = param_bb * sigma0;
        let t29 = rho0 * rho0;
        let t30 = pow_1_3(rho0);
        let t31 = t30 * t30;
        let t33 = 1.0 / t31 / t29;
        let t36 = f64::sqrt(sigma0);
        let t37 = param_cc * t36;
        let t39 = 1.0 / t30 / rho0;
        let t40 = M_CBRT2;
        let t43 = 4.0 * t36 * t39 + t40;
        let t44 = 1.0 / t43;
        let t45 = t39 * t44;
        let t47 = param_aa + 0.13888888888888888889e-1 * t28 * t33 + t37 * t45;
        let t51 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t47);
        let t52 = rho1 <= dens_threshold;
        let t53 = -t16;
        let t55 = piecewise5(t14, t11, t10, t15, t53 * t7);
        let t56 = 1.0 + t55;
        let t57 = t56 <= zeta_threshold;
        let t58 = pow_1_3(t56);
        let t60 = piecewise3(t57, t22, t58 * t56);
        let t61 = t60 * t26;
        let t62 = param_bb * sigma2;
        let t63 = rho1 * rho1;
        let t64 = pow_1_3(rho1);
        let t65 = t64 * t64;
        let t67 = 1.0 / t65 / t63;
        let t70 = f64::sqrt(sigma2);
        let t71 = param_cc * t70;
        let t73 = 1.0 / t64 / rho1;
        let t76 = 4.0 * t70 * t73 + t40;
        let t77 = 1.0 / t76;
        let t78 = t73 * t77;
        let t80 = param_aa + 0.13888888888888888889e-1 * t62 * t67 + t71 * t78;
        let t84 = piecewise3(t52, 0.0, -3.0 / 8.0 * t5 * t61 * t80);
        let tzk0 = t51 + t84;
        zk[ip] += tzk0;
    }
}
