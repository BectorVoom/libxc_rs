//! GGA_X_Q2D exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q2d.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_q2d_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3::<f64>(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t19 * t19;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = 0.804e0 + 5.0 / 972.0 * t34;
        let t39 = 0.1804e1 - 0.646416e0 / t36;
        let t40 = t20 * t20;
        let t42 = 1.0 / t22 / t21;
        let t43 = t40 * t42;
        let t44 = sigma[ip] * sigma[ip];
        let t45 = t44 * t26;
        let t46 = t29 * t29;
        let t47 = t46 * rho[ip];
        let t49 = 1.0 / t19 / t47;
        let t53 = 100.0 - t43 * t45 * t49 / 288.0;
        let t55 = 1.0 / t22;
        let t56 = t40 * t55;
        let t57 = f64::sqrt(sigma[ip]);
        let t60 = 1.0 / t19 / rho[ip];
        let t62 = t56 * t57 * t26 * t60;
        let t63 = f64::powf(t62, 0.35e1);
        let t65 = 1.0 + t34 / 24.0;
        let t68 = t39 * t53 + 0.87153829697982569831e-4 * t63 * t65;
        let t70 = t21 * t21;
        let t71 = 1.0 / t70;
        let t72 = t44 * sigma[ip];
        let t74 = t46 * t46;
        let t75 = 1.0 / t74;
        let t78 = 100.0 + t71 * t72 * t75 / 576.0;
        let t79 = 1.0 / t78;
        let t83 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t68 * t79);
        let tzk0 = 2.0 * t83;
        zk[ip] += tzk0;
    }
}
