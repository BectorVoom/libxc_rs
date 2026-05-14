//! GGA_C_REGTPSS exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_regtpss.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_regtpss_exc_unpol(
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
        let t58 = f64::ln(2.0);
        let t59 = 1.0 - t58;
        let t60 = M_PI * M_PI;
        let t61 = 1.0 / t60;
        let t62 = t59 * t61;
        let t63 = t34 * t34;
        let t64 = piecewise3(t33, t63, 1.0);
        let t65 = t64 * t64;
        let t66 = t65 * t64;
        let t68 = 1.0 + 0.25e-1 * t10;
        let t70 = 1.0 + 0.4445e-1 * t10;
        let t71 = 1.0 / t70;
        let t72 = t68 * t71;
        let t73 = rho[ip] * rho[ip];
        let t75 = 1.0 / t7 / t73;
        let t78 = 1.0 / t65;
        let t80 = 1.0 / t3;
        let t81 = t80 * t5;
        let t82 = t78 * t18 * t81;
        let t85 = 1.0 / t59;
        let t88 = 1.0 / t66;
        let t89 = t60 * t88;
        let t91 = f64::exp(-(-t32 + t57) * t85 * t89);
        let t92 = t91 - 1.0;
        let t93 = 1.0 / t92;
        let t94 = t85 * t93;
        let t95 = sigma[ip] * sigma[ip];
        let t96 = t94 * t95;
        let t97 = t72 * t96;
        let t98 = t73 * t73;
        let t100 = 1.0 / t21 / t98;
        let t101 = t39 * t39;
        let t102 = t100 * t101;
        let t103 = t65 * t65;
        let t104 = 1.0 / t103;
        let t105 = t102 * t104;
        let t106 = 1.0 / t19;
        let t107 = t1 * t106;
        let t108 = t107 * t6;
        let t109 = t105 * t108;
        let t112 = sigma[ip] * t75 * t39 * t82 / 96.0 + 0.21437009059034868486e-3 * t97 * t109;
        let t113 = t112 * t85;
        let t114 = t94 * t112;
        let t117 = 1.0 + 0.65854491829355115987e0 * t72 * t114;
        let t118 = 1.0 / t117;
        let t119 = t113 * t118;
        let t122 = 1.0 + 0.65854491829355115987e0 * t72 * t119;
        let t123 = f64::ln(t122);
        let t125 = t62 * t66 * t123;
        let tzk0 = -t32 + t57 + t125;
        zk[ip] += tzk0;
    }
}
