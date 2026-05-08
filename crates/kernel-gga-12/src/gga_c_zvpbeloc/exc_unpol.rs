//! GGA_C_ZVPBELOC exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 94 shared lines across all orders.
//! Delta: 94 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_zvpbeloc_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (94 lines) ---
        let t1 = f64::powf(4.0, 1.0 / 6.0);
        let t2 = t1 * t1;
        let t3 = t2 * t2;
        let t4 = t3 * t1;
        let t5 = f64::powf(3.0, 1.0 / 6.0);
        let t6 = t4 * t5;
        let t7 = M_PI * M_PI;
        let t8 = 1.0 / t7;
        let t9 = f64::powf(t8, 1.0 / 6.0);
        let t10 = t6 * t9;
        let t11 = 1.0 / M_PI;
        let t12 = pow_1_3(t11);
        let t13 = pow_1_3(rho[ip]);
        let t14 = 1.0 / t13;
        let t17 = piecewise3(0.1e-19 < 0.0, 0.0, 0.1e-19);
        let t19 = t10 * t12 * t14 * t17;
        let t21 = f64::exp(-0.99999999999999999999e0 * t19);
        let t22 = M_CBRT3;
        let t23 = t22 * t12;
        let t24 = M_CBRT4;
        let t25 = t24 * t24;
        let t27 = t23 * t25 * t14;
        let t29 = 1.0 + 0.53425e-1 * t27;
        let t30 = f64::sqrt(t27);
        let t33 = pow_3_2(t27);
        let t35 = t22 * t22;
        let t36 = t12 * t12;
        let t37 = t35 * t36;
        let t38 = t13 * t13;
        let t41 = t37 * t24 / t38;
        let t43 = 0.379785e1 * t30 + 0.8969e0 * t27 + 0.204775e0 * t33 + 0.123235e0 * t41;
        let t46 = 1.0 + 0.16081979498692535067e2 / t43;
        let t47 = f64::ln(t46);
        let t49 = 0.621814e-1 * t29 * t47;
        let t50 = 1.0 <= zeta_threshold;
        let t51 = pow_1_3(zeta_threshold);
        let t53 = piecewise3(t50, t51 * zeta_threshold, 1.0);
        let t56 = M_CBRT2;
        let t60 = (2.0 * t53 - 2.0) / (2.0 * t56 - 2.0);
        let t62 = 1.0 + 0.278125e-1 * t27;
        let t67 = 0.51785e1 * t30 + 0.905775e0 * t27 + 0.1100325e0 * t33 + 0.1241775e0 * t41;
        let t70 = 1.0 + 0.29608749977793437516e2 / t67;
        let t71 = f64::ln(t70);
        let t74 = 0.19751673498613801407e-1 * t60 * t62 * t71;
        let t75 = f64::ln(2.0);
        let t76 = 1.0 - t75;
        let t77 = t76 * t8;
        let t78 = t51 * t51;
        let t79 = piecewise3(t50, t78, 1.0);
        let t80 = t79 * t79;
        let t81 = t80 * t79;
        let t82 = rho[ip] * rho[ip];
        let t84 = 1.0 / t13 / t82;
        let t85 = sigma[ip] * t84;
        let t86 = 1.0 / t80;
        let t87 = t56 * t86;
        let t89 = 1.0 / t12;
        let t90 = t35 * t89;
        let t92 = f64::exp(-t41 / 4.0);
        let t93 = 1.0 - t92;
        let t94 = t24 * t93;
        let t95 = t90 * t94;
        let t98 = 0.375e-1 + 0.83333333333333333332e-3 * t85 * t87 * t95;
        let t100 = t86 * t35;
        let t102 = t100 * t89 * t24;
        let t105 = 1.0 / t76;
        let t106 = t98 * t105;
        let t109 = 1.0 / t81;
        let t112 = f64::exp(-(-t49 + t74) * t105 * t7 * t109);
        let t113 = t112 - 1.0;
        let t114 = 1.0 / t113;
        let t115 = t7 * t114;
        let t116 = sigma[ip] * sigma[ip];
        let t117 = t115 * t116;
        let t118 = t106 * t117;
        let t119 = t82 * t82;
        let t121 = 1.0 / t38 / t119;
        let t122 = t56 * t56;
        let t123 = t121 * t122;
        let t124 = t80 * t80;
        let t125 = 1.0 / t124;
        let t127 = 1.0 / t36;
        let t129 = t22 * t127 * t25;
        let t130 = t123 * t125 * t129;
        let t133 = t85 * t56 * t102 / 96.0 + t118 * t130 / 3072.0;
        let t134 = t98 * t133;
        let t135 = t105 * t7;
        let t136 = t115 * t133;
        let t138 = t106 * t136 + 1.0;
        let t139 = 1.0 / t138;
        let t140 = t135 * t139;
        let t142 = t134 * t140 + 1.0;
        let t143 = f64::ln(t142);
        let tzk0 = t21 * (t77 * t81 * t143 - t49 + t74);
        zk[ip] += tzk0;
    }
}
