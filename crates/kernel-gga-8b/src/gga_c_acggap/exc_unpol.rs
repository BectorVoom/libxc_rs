//! GGA_C_ACGGAP exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_acggap.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_acggap_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
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
        let t58 = f64::ln(2.0);
        let t59 = 1.0 - t58;
        let t60 = M_PI * M_PI;
        let t61 = 1.0 / t60;
        let t62 = t59 * t61;
        let t63 = t34 * t34;
        let t64 = piecewise3(t33, t63, 1.0);
        let t65 = t64 * t64;
        let t66 = t65 * t64;
        let t68 = 1.0 + 0.416675e-1 * t10;
        let t72 = 1.0 + 0.125e0 * t4 * t9 * t68;
        let t74 = 1.0 + 0.740825e-1 * t10;
        let t78 = 1.0 + 0.125e0 * t4 * t9 * t74;
        let t79 = 1.0 / t78;
        let t80 = t72 * t79;
        let t81 = rho[ip] * rho[ip];
        let t83 = 1.0 / t7 / t81;
        let t84 = sigma[ip] * t83;
        let t85 = 1.0 / t65;
        let t86 = t39 * t85;
        let t87 = t84 * t86;
        let t88 = 1.0 / t3;
        let t89 = t18 * t88;
        let t90 = f64::sqrt(sigma[ip]);
        let t92 = 1.0 / t7 / rho[ip];
        let t94 = t39 * t39;
        let t95 = 1.0 / t64;
        let t96 = t94 * t95;
        let t97 = 1.0 / t13;
        let t98 = t96 * t97;
        let t99 = t90 * t92 * t98;
        let t101 = 0.45e1 + t99 / 4.0;
        let t102 = t5 * t101;
        let t104 = 0.45e1 + 0.36675e0 * t99;
        let t105 = 1.0 / t104;
        let t107 = t89 * t102 * t105;
        let t110 = 1.0 / t59;
        let t111 = t80 * t110;
        let t114 = 1.0 / t66;
        let t115 = t60 * t114;
        let t117 = f64::exp(-(-t32 + t57) * t110 * t115);
        let t118 = t117 - 1.0;
        let t119 = 1.0 / t118;
        let t120 = sigma[ip] * sigma[ip];
        let t121 = t119 * t120;
        let t122 = t81 * t81;
        let t124 = 1.0 / t21 / t122;
        let t125 = t121 * t124;
        let t126 = t111 * t125;
        let t127 = t65 * t65;
        let t128 = 1.0 / t127;
        let t129 = t94 * t128;
        let t130 = t129 * t1;
        let t131 = 1.0 / t19;
        let t132 = t131 * t6;
        let t133 = t101 * t101;
        let t134 = t104 * t104;
        let t135 = 1.0 / t134;
        let t136 = t133 * t135;
        let t137 = t132 * t136;
        let t138 = t130 * t137;
        let t141 = t87 * t107 / 96.0 + 0.21437009059034868486e-3 * t126 * t138;
        let t142 = t141 * t110;
        let t143 = t110 * t119;
        let t144 = t143 * t141;
        let t147 = 1.0 + 0.65854491829355115987e0 * t80 * t144;
        let t148 = 1.0 / t147;
        let t149 = t142 * t148;
        let t152 = 1.0 + 0.65854491829355115987e0 * t80 * t149;
        let t153 = f64::ln(t152);
        let t155 = t62 * t66 * t153;
        let tzk0 = -t32 + t57 + t155;
        zk[ip] += tzk0;
    }
}
