//! GGA_X_S12 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_s12.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_s12_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    param_bx: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t19 * param_bx;
        let t21 = param_C * sigma[ip];
        let t22 = M_CBRT2;
        let t23 = t22 * t22;
        let t24 = rho[ip] * rho[ip];
        let t25 = t19 * t19;
        let t27 = 1.0 / t25 / t24;
        let t28 = t23 * t27;
        let t30 = sigma[ip] * sigma[ip];
        let t31 = param_D * t30;
        let t32 = t24 * t24;
        let t33 = t32 * rho[ip];
        let t35 = 1.0 / t19 / t33;
        let t36 = t22 * t35;
        let t39 = t21 * t28 + 2.0 * t31 * t36 + 1.0;
        let t42 = param_B * (1.0 - 1.0 / t39);
        let t43 = param_E * sigma[ip];
        let t45 = t43 * t28 + 1.0;
        let t47 = 1.0 - 1.0 / t45;
        let t49 = t42 * t47 + param_A;
        let t53 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t18 * t20 * t49);
        let tzk0 = 2.0 * t53;
        zk[ip] += tzk0;
    }
}
