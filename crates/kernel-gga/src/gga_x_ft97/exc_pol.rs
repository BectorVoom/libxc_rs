//! GGA_X_FT97 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 98 shared lines across all orders.
//! Delta: 98 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ft97_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_beta0: f64,
    param_beta1: f64,
    param_beta2: f64,
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
        // --- shared preamble (98 lines) ---
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
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = param_beta1 * sigma0;
        let t29 = rho0 * rho0;
        let t30 = pow_1_3(rho0);
        let t31 = t30 * t30;
        let t33 = 1.0 / t31 / t29;
        let t34 = M_CBRT2;
        let t35 = t33 * t34;
        let t36 = t28 * t35;
        let t37 = t19 * t19;
        let t38 = t6 * t6;
        let t39 = t37 * t38;
        let t40 = t19 * t6;
        let t41 = pow_1_3(t40);
        let t42 = t41 * t41;
        let t43 = sigma0 * t33;
        let t44 = t43 * t34;
        let t45 = t39 * t42;
        let t48 = param_beta2 + t44 * t45 / 8.0;
        let t49 = 1.0 / t48;
        let t50 = t42 * t49;
        let t51 = t39 * t50;
        let t54 = param_beta0 + t36 * t51 / 8.0;
        let t55 = t54 * sigma0;
        let t57 = t2 * t2;
        let t59 = pow_1_3(1.0 / M_PI);
        let t60 = 1.0 / t59;
        let t61 = t57 * t60;
        let t62 = M_CBRT4;
        let t63 = t54 * t54;
        let t64 = f64::ln(t43 + f64::sqrt(t43 * t43 + 1.0));
        let t65 = t64 * t64;
        let t66 = t63 * t65;
        let t69 = 9.0 * t43 * t66 + 1.0;
        let t70 = f64::sqrt(t69);
        let t71 = 1.0 / t70;
        let t73 = t61 * t62 * t71;
        let t76 = 1.0 + 2.0 / 9.0 * t55 * t33 * t73;
        let t80 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t76);
        let t81 = rho1 <= dens_threshold;
        let t82 = -t16;
        let t84 = piecewise5(t14, t11, t10, t15, t82 * t7);
        let t85 = 1.0 + t84;
        let t86 = t85 <= zeta_threshold;
        let t87 = pow_1_3(t85);
        let t89 = piecewise3(t86, t22, t87 * t85);
        let t90 = t89 * t26;
        let t91 = param_beta1 * sigma2;
        let t92 = rho1 * rho1;
        let t93 = pow_1_3(rho1);
        let t94 = t93 * t93;
        let t96 = 1.0 / t94 / t92;
        let t97 = t96 * t34;
        let t98 = t91 * t97;
        let t99 = t85 * t85;
        let t100 = t99 * t38;
        let t101 = t85 * t6;
        let t102 = pow_1_3(t101);
        let t103 = t102 * t102;
        let t104 = sigma2 * t96;
        let t105 = t104 * t34;
        let t106 = t100 * t103;
        let t109 = param_beta2 + t105 * t106 / 8.0;
        let t110 = 1.0 / t109;
        let t111 = t103 * t110;
        let t112 = t100 * t111;
        let t115 = param_beta0 + t98 * t112 / 8.0;
        let t116 = t115 * sigma2;
        let t118 = t115 * t115;
        let t119 = f64::ln(t104 + f64::sqrt(t104 * t104 + 1.0));
        let t120 = t119 * t119;
        let t121 = t118 * t120;
        let t124 = 9.0 * t104 * t121 + 1.0;
        let t125 = f64::sqrt(t124);
        let t126 = 1.0 / t125;
        let t128 = t61 * t62 * t126;
        let t131 = 1.0 + 2.0 / 9.0 * t116 * t96 * t128;
        let t135 = piecewise3(t81, 0.0, -3.0 / 8.0 * t5 * t90 * t131);
        let tzk0 = t80 + t135;
        zk[ip] += tzk0;
    }
}
