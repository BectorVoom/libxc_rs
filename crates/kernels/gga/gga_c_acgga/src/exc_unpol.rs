//! GGA_C_ACGGA exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_acgga.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_acgga_exc_unpol(
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
        let t3 = pow_1_3::<f64>(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3::<f64>(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2::<f64>(t10);
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
        let t34 = pow_1_3::<f64>(zeta_threshold);
        let t36 = piecewise3::<f64>(t33, t34 * zeta_threshold, 1.0);
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
        let t64 = piecewise3::<f64>(t33, t63, 1.0);
        let t65 = t64 * t64;
        let t66 = t65 * t64;
        let t67 = rho[ip] * rho[ip];
        let t69 = 1.0 / t7 / t67;
        let t70 = sigma[ip] * t69;
        let t71 = 1.0 / t65;
        let t72 = t39 * t71;
        let t73 = t70 * t72;
        let t74 = 1.0 / t3;
        let t75 = t18 * t74;
        let t76 = f64::sqrt(sigma[ip]);
        let t78 = 1.0 / t7 / rho[ip];
        let t80 = t39 * t39;
        let t81 = 1.0 / t64;
        let t82 = t80 * t81;
        let t83 = 1.0 / t13;
        let t84 = t82 * t83;
        let t85 = t76 * t78 * t84;
        let t87 = 0.45e1 + t85 / 4.0;
        let t88 = t5 * t87;
        let t90 = 0.45e1 + 0.36675e0 * t85;
        let t91 = 1.0 / t90;
        let t93 = t75 * t88 * t91;
        let t96 = 1.0 / t59;
        let t99 = 1.0 / t66;
        let t100 = t60 * t99;
        let t102 = f64::exp(-(-t32 + t57) * t96 * t100);
        let t103 = t102 - 1.0;
        let t104 = 1.0 / t103;
        let t105 = t96 * t104;
        let t106 = sigma[ip] * sigma[ip];
        let t107 = t67 * t67;
        let t109 = 1.0 / t21 / t107;
        let t110 = t106 * t109;
        let t112 = t105 * t110 * t80;
        let t113 = t65 * t65;
        let t114 = 1.0 / t113;
        let t115 = t114 * t1;
        let t116 = 1.0 / t19;
        let t117 = t115 * t116;
        let t118 = t87 * t87;
        let t119 = t6 * t118;
        let t120 = t90 * t90;
        let t121 = 1.0 / t120;
        let t122 = t119 * t121;
        let t123 = t117 * t122;
        let t126 = t73 * t93 / 96.0 + 0.21437009059034868486e-3 * t112 * t123;
        let t127 = t126 * t96;
        let t130 = 1.0 + 0.65854491829355115987e0 * t105 * t126;
        let t131 = 1.0 / t130;
        let t134 = 1.0 + 0.65854491829355115987e0 * t127 * t131;
        let t135 = f64::ln(t134);
        let t137 = t62 * t66 * t135;
        let tzk0 = -t32 + t57 + t137;
        zk[ip] += tzk0;
    }
}
