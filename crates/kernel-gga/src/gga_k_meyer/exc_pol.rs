//! GGA_K_MEYER exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_meyer.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_meyer_exc_pol(
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
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t36 = 1.0 / t35;
        let t37 = t32 * t36;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t46 = 1.0 - t37 * sigma0 * t42 / 864.0;
        let t47 = t32 * t32;
        let t48 = 1.0 / t34;
        let t49 = t47 * t48;
        let t50 = f64::sqrt(sigma0);
        let t51 = t39 * rho0;
        let t52 = 1.0 / t51;
        let t55 = t49 * t50 * t52 / 72.0;
        let t56 = 1.0 + t55;
        let t57 = 1.0 - t55;
        let t58 = f64::abs(t57);
        let t59 = 1.0 / t58;
        let t61 = f64::ln(t56 * t59);
        let t63 = t46 * t61 * t32;
        let t64 = 1.0 / t50;
        let t65 = t34 * t64;
        let t68 = 3.0 * t63 * t65 * t51;
        let t69 = 1.0 / 2.0 - t68;
        let t70 = 1.0 / 2.0 + t68;
        let t71 = 1.0 / t70;
        let t74 = 20.0 * t69 * t71 + 1.0;
        let t78 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t74);
        let t79 = rho1 <= dens_threshold;
        let t80 = -t17;
        let t82 = piecewise5(t15, t12, t11, t16, t80 * t8);
        let t83 = 1.0 + t82;
        let t84 = t83 <= zeta_threshold;
        let t85 = pow_1_3(t83);
        let t86 = t85 * t85;
        let t88 = piecewise3(t84, t24, t86 * t83);
        let t89 = t88 * t30;
        let t90 = rho1 * rho1;
        let t91 = pow_1_3(rho1);
        let t92 = t91 * t91;
        let t94 = 1.0 / t92 / t90;
        let t98 = 1.0 - t37 * sigma2 * t94 / 864.0;
        let t99 = f64::sqrt(sigma2);
        let t100 = t91 * rho1;
        let t101 = 1.0 / t100;
        let t104 = t49 * t99 * t101 / 72.0;
        let t105 = 1.0 + t104;
        let t106 = 1.0 - t104;
        let t107 = f64::abs(t106);
        let t108 = 1.0 / t107;
        let t110 = f64::ln(t105 * t108);
        let t112 = t98 * t110 * t32;
        let t113 = 1.0 / t99;
        let t114 = t34 * t113;
        let t117 = 3.0 * t112 * t114 * t100;
        let t118 = 1.0 / 2.0 - t117;
        let t119 = 1.0 / 2.0 + t117;
        let t120 = 1.0 / t119;
        let t123 = 20.0 * t118 * t120 + 1.0;
        let t127 = piecewise3(t79, 0.0, 3.0 / 20.0 * t6 * t89 * t123);
        let tzk0 = t78 + t127;
        zk[ip] += tzk0;
    }
}
