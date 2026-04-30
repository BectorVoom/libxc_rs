//! GGA_C_AM05 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 82 shared lines across all orders.
//! Delta: 82 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_am05_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha: f64,
    param_gamma: f64,
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
        // --- shared preamble (82 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t11 = t4 * t6 / t8;
        let t13 = 1.0 + 0.53425e-1 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t1 * t1;
        let t20 = t3 * t3;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t5 / t22;
        let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
        let t30 = 1.0 + 0.16081979498692535067e2 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.621814e-1 * t13 * t31;
        let t34 = rho0 - rho1;
        let t35 = t34 * t34;
        let t36 = t35 * t35;
        let t37 = t7 * t7;
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t36 * t39;
        let t41 = 1.0 / t7;
        let t42 = t34 * t41;
        let t43 = 1.0 + t42;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(zeta_threshold);
        let t46 = t45 * zeta_threshold;
        let t47 = pow_1_3(t43);
        let t49 = piecewise3(t44, t46, t47 * t43);
        let t50 = 1.0 - t42;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t54 = piecewise3(t51, t46, t52 * t50);
        let t55 = t49 + t54 - 2.0;
        let t56 = M_CBRT2;
        let t59 = 1.0 / (2.0 * t56 - 2.0);
        let t60 = t55 * t59;
        let t62 = 1.0 + 0.5137e-1 * t11;
        let t67 = 0.705945e1 * t14 + 0.1549425e1 * t11 + 0.420775e0 * t17 + 0.1562925e0 * t25;
        let t70 = 1.0 + 0.32163958997385070134e2 / t67;
        let t71 = f64::ln(t70);
        let t75 = 1.0 + 0.278125e-1 * t11;
        let t80 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
        let t83 = 1.0 + 0.29608749977793437516e2 / t80;
        let t84 = f64::ln(t83);
        let t85 = t75 * t84;
        let t87 = -0.310907e-1 * t62 * t71 + t33 - 0.19751673498613801407e-1 * t85;
        let t88 = t60 * t87;
        let t92 = -t33 + t40 * t88 + 0.19751673498613801407e-1 * t60 * t85;
        let t93 = piecewise3(t44, zeta_threshold, t43);
        let t94 = M_CBRT6;
        let t95 = param_alpha * t94;
        let t96 = M_PI * M_PI;
        let t97 = pow_1_3(t96);
        let t98 = t97 * t97;
        let t99 = 1.0 / t98;
        let t100 = t99 * sigma0;
        let t101 = rho0 * rho0;
        let t102 = pow_1_3(rho0);
        let t103 = t102 * t102;
        let t105 = 1.0 / t103 / t101;
        let t109 = 1.0 + t95 * t100 * t105 / 24.0;
        let t110 = 1.0 / t109;
        let t113 = t110 + param_gamma * (1.0 - t110);
        let t115 = piecewise3(t51, zeta_threshold, t50);
        let t116 = t99 * sigma2;
        let t117 = rho1 * rho1;
        let t118 = pow_1_3(rho1);
        let t119 = t118 * t118;
        let t121 = 1.0 / t119 / t117;
        let t125 = 1.0 + t95 * t116 * t121 / 24.0;
        let t126 = 1.0 / t125;
        let t129 = t126 + param_gamma * (1.0 - t126);
        let t132 = t93 * t113 / 2.0 + t115 * t129 / 2.0;
        let tzk0 = t92 * t132;
        zk[ip] += tzk0;
    }
}
