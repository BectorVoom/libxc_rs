//! GGA_X_PBEA exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbea.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbea_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t28 = rho0 * rho0;
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / t28;
        let t35 = 1.0 + 0.86399408095363255118e-2 * sigma0 * t32;
        let t36 = f64::powf(t35, -0.52e0);
        let t38 = 0.1804e1 - 0.804e0 * t36;
        let t42 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t25 * t26 * t38);
        let t43 = rho1 <= dens_threshold;
        let t44 = -t16;
        let t46 = piecewise5(t14, t11, t10, t15, t44 * t7);
        let t47 = 1.0 + t46;
        let t48 = t47 <= zeta_threshold;
        let t49 = pow_1_3(t47);
        let t51 = piecewise3(t48, t22, t49 * t47);
        let t53 = rho1 * rho1;
        let t54 = pow_1_3(rho1);
        let t55 = t54 * t54;
        let t57 = 1.0 / t55 / t53;
        let t60 = 1.0 + 0.86399408095363255118e-2 * sigma2 * t57;
        let t61 = f64::powf(t60, -0.52e0);
        let t63 = 0.1804e1 - 0.804e0 * t61;
        let t67 = piecewise3(t43, 0.0, -3.0 / 8.0 * t5 * t51 * t26 * t63);
        let tzk0 = t42 + t67;
        zk[ip] += tzk0;
    }
}
