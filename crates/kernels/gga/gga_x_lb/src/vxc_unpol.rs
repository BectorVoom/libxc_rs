//! GGA_X_LB vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_vxc/gga_x_lb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lb_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alpha: f64,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let t1 = M_CBRT3;
        let t4 = pow_1_3::<f64>(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t10 = f64::sqrt(sigma[ip]);
        let t11 = M_CBRT2;
        let t12 = t10 * t11;
        let t13 = pow_1_3::<f64>(rho[ip]);
        let t15 = 1.0 / t13 / rho[ip];
        let t17 = t12 * t15 < 300.0;
        let t18 = param_beta * sigma[ip];
        let t19 = t11 * t11;
        let t20 = rho[ip] * rho[ip];
        let t21 = t13 * t13;
        let t23 = 1.0 / t21 / t20;
        let t24 = t19 * t23;
        let t25 = param_beta * t10;
        let t26 = t11 * t15;
        let t28 = param_gamma * t10 * t26;
        let t29 = f64::ln(t28 + f64::sqrt(t28 * t28 + 1.0));
        let t30 = t26 * t29;
        let t33 = 3.0 * t25 * t30 + 1.0;
        let t34 = 1.0 / t33;
        let t38 = f64::ln(2.0 * t28);
        let t39 = 1.0 / t38;
        let t40 = t15 * t39;
        let t43 = piecewise3::<f64>(t17, t18 * t24 * t34, t12 * t40 / 3.0);
        let t45 = (-param_alpha * t1 * t4 * t6 / 2.0 - t43) * t19;
        let tvrho0 = t45 * t13 / 2.0;
        vrho[ip] += tvrho0;
    }
}
