//! GGA_X_EV93 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ev93.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ev93_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_b1: f64,
    param_b2: f64,
    param_b3: f64,
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
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = M_CBRT6;
        let t29 = param_a1 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = t34 * t39;
        let t43 = t28 * t28;
        let t44 = param_a2 * t43;
        let t46 = 1.0 / t31 / t30;
        let t47 = sigma0 * sigma0;
        let t48 = t46 * t47;
        let t49 = t35 * t35;
        let t50 = t49 * rho0;
        let t52 = 1.0 / t36 / t50;
        let t53 = t48 * t52;
        let t56 = t30 * t30;
        let t57 = 1.0 / t56;
        let t58 = param_a3 * t57;
        let t59 = t47 * sigma0;
        let t60 = t49 * t49;
        let t61 = 1.0 / t60;
        let t62 = t59 * t61;
        let t65 = 1.0 + t29 * t40 / 24.0 + t44 * t53 / 576.0 + t58 * t62 / 2304.0;
        let t66 = t27 * t65;
        let t67 = param_b1 * t28;
        let t70 = param_b2 * t43;
        let t73 = param_b3 * t57;
        let t76 = 1.0 + t67 * t40 / 24.0 + t70 * t53 / 576.0 + t73 * t62 / 2304.0;
        let t77 = 1.0 / t76;
        let t78 = t66 * t77;
        let t81 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t78);
        let t82 = rho1 <= dens_threshold;
        let t83 = -t16;
        let t85 = piecewise5(t14, t11, t10, t15, t83 * t7);
        let t86 = 1.0 + t85;
        let t87 = t86 <= zeta_threshold;
        let t88 = pow_1_3(t86);
        let t90 = piecewise3(t87, t22, t88 * t86);
        let t91 = t5 * t90;
        let t92 = t33 * sigma2;
        let t93 = rho1 * rho1;
        let t94 = pow_1_3(rho1);
        let t95 = t94 * t94;
        let t97 = 1.0 / t95 / t93;
        let t98 = t92 * t97;
        let t101 = sigma2 * sigma2;
        let t102 = t46 * t101;
        let t103 = t93 * t93;
        let t104 = t103 * rho1;
        let t106 = 1.0 / t94 / t104;
        let t107 = t102 * t106;
        let t110 = t101 * sigma2;
        let t111 = t103 * t103;
        let t112 = 1.0 / t111;
        let t113 = t110 * t112;
        let t116 = 1.0 + t29 * t98 / 24.0 + t44 * t107 / 576.0 + t58 * t113 / 2304.0;
        let t117 = t27 * t116;
        let t124 = 1.0 + t67 * t98 / 24.0 + t70 * t107 / 576.0 + t73 * t113 / 2304.0;
        let t125 = 1.0 / t124;
        let t126 = t117 * t125;
        let t129 = piecewise3(t82, 0.0, -3.0 / 8.0 * t91 * t126);
        let tzk0 = t81 + t129;
        zk[ip] += tzk0;
    }
}
