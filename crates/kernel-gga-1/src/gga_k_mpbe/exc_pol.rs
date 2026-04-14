//! GGA_K_MPBE exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 89 shared lines across all orders.
//! Delta: 89 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_mpbe_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
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
        // --- shared preamble (89 lines) ---
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
        let t33 = param_c1 * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t45 = param_a * t32;
        let t46 = t37 * sigma0;
        let t50 = 1.0 + t45 * t46 * t43 / 24.0;
        let t51 = 1.0 / t50;
        let t55 = t32 * t32;
        let t56 = param_c2 * t55;
        let t58 = 1.0 / t35 / t34;
        let t59 = t56 * t58;
        let t60 = sigma0 * sigma0;
        let t61 = t39 * t39;
        let t62 = t61 * rho0;
        let t64 = 1.0 / t40 / t62;
        let t66 = t50 * t50;
        let t67 = 1.0 / t66;
        let t71 = t34 * t34;
        let t72 = 1.0 / t71;
        let t73 = param_c3 * t72;
        let t74 = t60 * sigma0;
        let t75 = t61 * t61;
        let t76 = 1.0 / t75;
        let t78 = t66 * t50;
        let t79 = 1.0 / t78;
        let t83 = 1.0 + t38 * sigma0 * t43 * t51 / 24.0 + t59 * t60 * t64 * t67 / 576.0 + t73 * t74 * t76 * t79 / 2304.0;
        let t87 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t83);
        let t88 = rho1 <= dens_threshold;
        let t89 = -t17;
        let t91 = piecewise5(t15, t12, t11, t16, t89 * t8);
        let t92 = 1.0 + t91;
        let t93 = t92 <= zeta_threshold;
        let t94 = pow_1_3(t92);
        let t95 = t94 * t94;
        let t97 = piecewise3(t93, t24, t95 * t92);
        let t98 = t97 * t30;
        let t99 = rho1 * rho1;
        let t100 = pow_1_3(rho1);
        let t101 = t100 * t100;
        let t103 = 1.0 / t101 / t99;
        let t105 = t37 * sigma2;
        let t109 = 1.0 + t45 * t105 * t103 / 24.0;
        let t110 = 1.0 / t109;
        let t114 = sigma2 * sigma2;
        let t115 = t99 * t99;
        let t116 = t115 * rho1;
        let t118 = 1.0 / t100 / t116;
        let t120 = t109 * t109;
        let t121 = 1.0 / t120;
        let t125 = t114 * sigma2;
        let t126 = t115 * t115;
        let t127 = 1.0 / t126;
        let t129 = t120 * t109;
        let t130 = 1.0 / t129;
        let t134 = 1.0 + t38 * sigma2 * t103 * t110 / 24.0 + t59 * t114 * t118 * t121 / 576.0 + t73 * t125 * t127 * t130 / 2304.0;
        let t138 = piecewise3(t88, 0.0, 3.0 / 20.0 * t6 * t98 * t134);
        let tzk0 = t87 + t138;
        zk[ip] += tzk0;
    }
}
