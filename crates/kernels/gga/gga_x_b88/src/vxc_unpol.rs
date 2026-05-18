//! GGA_X_B88 vxc unpol kernel.
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
pub fn gga_x_b88_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t19 = t17 * t18;
        let t20 = t3 * t3;
        let t21 = param_beta * t20;
        let t23 = pow_1_3::<f64>(1.0 / M_PI);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = sigma[ip] * t29;
        let t31 = rho[ip] * rho[ip];
        let t32 = t18 * t18;
        let t34 = 1.0 / t32 / t31;
        let t35 = param_gamma * param_beta;
        let t36 = f64::sqrt(sigma[ip]);
        let t37 = t35 * t36;
        let t39 = 1.0 / t18 / rho[ip];
        let t43 = f64::ln(t36 * t28 * t39 + f64::sqrt(pow_2::<f64>(t36 * t28 * t39) + 1.0));
        let t44 = t28 * t39 * t43;
        let t46 = t37 * t44 + 1.0;
        let t47 = 1.0 / t46;
        let t48 = t34 * t47;
        let t52 = 1.0 + 2.0 / 9.0 * t27 * t30 * t48;
        let t56 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t52);
        let tzk0 = 2.0 * t56;
        zk[ip] += tzk0;
        let t58 = t17 / t32;
        let t62 = t31 * rho[ip];
        let t64 = 1.0 / t32 / t62;
        let t65 = t64 * t47;
        let t69 = t46 * t46;
        let t70 = 1.0 / t69;
        let t71 = t34 * t70;
        let t75 = t28 / t18 / t31 * t43;
        let t77 = t35 * sigma[ip];
        let t78 = t29 * t64;
        let t80 = t30 * t34 + 1.0;
        let t81 = f64::sqrt(t80);
        let t82 = 1.0 / t81;
        let t83 = t78 * t82;
        let t86 = -4.0 / 3.0 * t37 * t75 - 4.0 / 3.0 * t77 * t83;
        let t91 = -16.0 / 27.0 * t27 * t30 * t65 - 2.0 / 9.0 * t27 * t30 * t71 * t86;
        let t96 = piecewise3::<f64>(t2, 0.0, -t6 * t58 * t52 / 8.0 - 3.0 / 8.0 * t6 * t19 * t91);
        let tvrho0 = 2.0 * rho[ip] * t96 + 2.0 * t56;
        vrho[ip] += tvrho0;
        let t99 = t21 * t24;
        let t100 = t25 * t29;
        let t104 = t35 / t36;
        let t106 = t29 * t34;
        let t107 = t106 * t82;
        let t110 = t104 * t44 / 2.0 + t35 * t107 / 2.0;
        let t115 = -2.0 / 9.0 * t27 * t30 * t71 * t110 + 2.0 / 9.0 * t99 * t100 * t48;
        let t119 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t115);
        let tvsigma0 = 2.0 * rho[ip] * t119;
        vsigma[ip] += tvsigma0;
    }
}
