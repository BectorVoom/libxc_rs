//! GGA_C_ZVPBEINT exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 89 shared lines across all orders.
//! Delta: 89 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_zvpbeint_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha: f64,
    param_beta: f64,
    param_omega: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (89 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 0.379785e1 * t13 + 0.8969e0 * t10 + 0.204775e0 * t16 + 0.123235e0 * t24;
        let t29 = 1.0 + 0.16081979498692535067e2 / t26;
        let t30 = f64::ln(t29);
        let t32 = 0.621814e-1 * t12 * t30;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608749977793437516e2 / t50;
        let t54 = f64::ln(t53);
        let t57 = 0.19751673498613801407e-1 * t43 * t45 * t54;
        let t58 = f64::sqrt(sigma[ip]);
        let t59 = t58 * sigma[ip];
        let t60 = param_alpha * t59;
        let t61 = rho[ip] * rho[ip];
        let t62 = t61 * t61;
        let t63 = 1.0 / t62;
        let t66 = 1.0 / t13 / t10;
        let t67 = 1.0 / t3;
        let t68 = t18 * t67;
        let t70 = t68 * t5 * t7;
        let t71 = f64::sqrt(t70);
        let t72 = t66 * t71;
        let t74 = piecewise3(0.1e-19 < 0.0, 0.0, 0.1e-19);
        let t76 = f64::powf(t74, param_omega / 2.0);
        let t77 = t72 * t76;
        let t80 = f64::exp(-t60 * t63 * t77 / 16.0);
        let t81 = f64::ln(2.0);
        let t82 = 1.0 - t81;
        let t83 = t80 * t82;
        let t84 = M_PI * M_PI;
        let t85 = 1.0 / t84;
        let t86 = t34 * t34;
        let t87 = piecewise3(t33, t86, 1.0);
        let t88 = t87 * t87;
        let t89 = t88 * t87;
        let t90 = t85 * t89;
        let t92 = 1.0 / t7 / t61;
        let t95 = 1.0 / t88;
        let t97 = t67 * t5;
        let t98 = t95 * t18 * t97;
        let t101 = 1.0 / t82;
        let t102 = param_beta * t101;
        let t105 = 1.0 / t89;
        let t108 = f64::exp(-(-t32 + t57) * t101 * t84 * t105);
        let t109 = t108 - 1.0;
        let t110 = 1.0 / t109;
        let t111 = t84 * t110;
        let t112 = sigma[ip] * sigma[ip];
        let t114 = t102 * t111 * t112;
        let t116 = 1.0 / t21 / t62;
        let t117 = t39 * t39;
        let t118 = t116 * t117;
        let t119 = t88 * t88;
        let t120 = 1.0 / t119;
        let t121 = t118 * t120;
        let t122 = 1.0 / t19;
        let t123 = t1 * t122;
        let t124 = t123 * t6;
        let t125 = t121 * t124;
        let t128 = sigma[ip] * t92 * t39 * t98 / 96.0 + t114 * t125 / 3072.0;
        let t129 = param_beta * t128;
        let t133 = t102 * t111 * t128 + 1.0;
        let t134 = 1.0 / t133;
        let t135 = t101 * t84 * t134;
        let t137 = t129 * t135 + 1.0;
        let t138 = f64::ln(t137);
        let t139 = t90 * t138;
        let t140 = t83 * t139;
        let tzk0 = -t32 + t57 + t140;
        zk[ip] += tzk0;
    }
}
