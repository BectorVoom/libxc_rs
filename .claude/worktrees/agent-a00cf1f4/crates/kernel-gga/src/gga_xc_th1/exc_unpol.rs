//! GGA_XC_TH1 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th1.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_xc_th1_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_20: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = f64::powf(2.0, 1.0 / 6.0);
        let t3 = t2 * t2;
        let t4 = t3 * t3;
        let t6 = param_omega_0 * t4 * t2;
        let t7 = f64::powf(rho[ip], 1.0 / 6.0);
        let t8 = t7 * rho[ip];
        let t12 = M_CBRT2;
        let t13 = t12 * t12;
        let t14 = param_omega_1 * t13;
        let t15 = pow_1_3(rho[ip]);
        let t16 = t15 * rho[ip];
        let t20 = M_SQRT2;
        let t21 = param_omega_2 * t20;
        let t22 = f64::sqrt(rho[ip]);
        let t23 = t22 * rho[ip];
        let t27 = param_omega_3 * t12;
        let t28 = t15 * t15;
        let t29 = t28 * rho[ip];
        let t33 = param_omega_4 * t13;
        let t34 = f64::sqrt(sigma[ip]);
        let t36 = pow_1_3(zeta_threshold);
        let t38 = piecewise3(1.0 <= zeta_threshold, t36 * zeta_threshold, 1.0);
        let t43 = param_omega_5 * t20;
        let t49 = param_omega_6 * t12;
        let t55 = param_omega_7 * t2;
        let t61 = param_omega_8 * t20;
        let t62 = 1.0 / t8;
        let t64 = t38 * t38;
        let t69 = param_omega_9 * t12;
        let t70 = 1.0 / rho[ip];
        let t76 = param_omega_10 * t2;
        let t77 = t7 * t7;
        let t78 = t77 * t77;
        let t79 = t78 * t7;
        let t80 = 1.0 / t79;
        let t85 = param_omega_11;
        let t86 = 1.0 / t28;
        let t87 = t85 * t86;
        let t88 = sigma[ip] * t64;
        let t92 = param_omega_12 * t20;
        let t93 = rho[ip] * rho[ip];
        let t95 = 1.0 / t28 / t93;
        let t96 = sigma[ip] * t95;
        let t98 = t96 * t64 - t96;
        let t103 = param_omega_13 * t12;
        let t108 = param_omega_14 * t2;
        let t109 = t79 * rho[ip];
        let t113 = param_omega_15;
        let t114 = t113 * t93;
        let t117 = param_omega_20;
        let t119 = t6 * t8 / 2.0 + t14 * t16 / 2.0 + t21 * t23 / 2.0 + t27 * t29 / 2.0 + t33 * t34 * t38 / 4.0 + t43 * t7 * t34 * t38 / 4.0 + t49 * t15 * t34 * t38 / 4.0 + t55 * t22 * t34 * t38 / 4.0 + t61 * t62 * sigma[ip] * t64 / 8.0 + t69 * t70 * sigma[ip] * t64 / 8.0 + t76 * t80 * sigma[ip] * t64 / 8.0 + t87 * t88 / 8.0 + t92 * t23 * t98 / 2.0 + t103 * t29 * t98 / 2.0 + t108 * t109 * t98 / 2.0 + t114 * t98 / 2.0 + t117 * rho[ip];
        let tzk0 = t119 * t70;
        zk[ip] += tzk0;
    }
}
