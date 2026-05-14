//! GGA_X_SSB_SW vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ssb_sw.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ssb_sw_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
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
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = param_B * t20 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t39 = 1.0 + param_C * t20 * t25 * t29 * t33 / 24.0;
        let t40 = 1.0 / t39;
        let t46 = param_D * t20 * t25;
        let t47 = t20 * t20;
        let t50 = 1.0 / t23 / t22;
        let t52 = sigma[ip] * sigma[ip];
        let t54 = t30 * t30;
        let t55 = t54 * rho[ip];
        let t57 = 1.0 / t18 / t55;
        let t61 = 1.0 + param_E * t47 * t50 * t52 * t27 * t57 / 288.0;
        let t62 = 1.0 / t61;
        let t67 = param_A + t26 * t29 * t33 * t40 / 24.0 - t46 * t29 * t33 * t62 / 24.0;
        let t71 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        let t73 = t17 / t31;
        let t77 = t30 * rho[ip];
        let t79 = 1.0 / t31 / t77;
        let t84 = param_B * t47;
        let t86 = t84 * t50 * t52;
        let t87 = t54 * t30;
        let t89 = 1.0 / t18 / t87;
        let t91 = t39 * t39;
        let t92 = 1.0 / t91;
        let t93 = t92 * param_C;
        let t94 = t27 * t89 * t93;
        let t101 = t22 * t22;
        let t102 = 1.0 / t101;
        let t103 = param_D * t102;
        let t104 = t52 * sigma[ip];
        let t105 = t103 * t104;
        let t106 = t54 * t54;
        let t107 = t106 * rho[ip];
        let t108 = 1.0 / t107;
        let t109 = t61 * t61;
        let t110 = 1.0 / t109;
        let t112 = t108 * t110 * param_E;
        let t115 = -t26 * t29 * t79 * t40 / 9.0 + t86 * t94 / 108.0 + t46 * t29 * t79 * t62 / 9.0 - t105 * t112 / 108.0;
        let t120 = piecewise3(t2, 0.0, -t6 * t73 * t67 / 8.0 - 3.0 / 8.0 * t6 * t19 * t115);
        let tvrho0 = 2.0 * rho[ip] * t120 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t123 = t28 * t33;
        let t130 = t27 * t57 * t93;
        let t137 = 1.0 / t106;
        let t139 = t137 * t110 * param_E;
        let t142 = t26 * t123 * t40 / 24.0 - t84 * t50 * sigma[ip] * t130 / 288.0 - t46 * t123 * t62 / 24.0 + t103 * t52 * t139 / 288.0;
        let t146 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t142);
        let tvsigma0 = 2.0 * rho[ip] * t146;
        vsigma[ip] += tvsigma0;
    }
}
