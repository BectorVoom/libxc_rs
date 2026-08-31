//! GGA_C_ACGGA exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_acgga.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(
    unused_imports,
    unused_variables,
    non_snake_case,
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::needless_return
)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::piecewise3;
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_acgga_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.053425 * t10;
        let t13 = rmath::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 3.79785 * t13 + 0.8969 * t10 + 0.204775 * t16 + 0.123235 * t24;
        let t29 = 1.0 + 16.081979498692537 / t26;
        let t30 = rmath::ln(t29);
        let t32 = 0.0621814 * t12 * t30;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.0278125 * t10;
        let t50 = 5.1785 * t13 + 0.905775 * t10 + 0.1100325 * t16 + 0.1241775 * t24;
        let t53 = 1.0 + 29.608749977793437 / t50;
        let t54 = rmath::ln(t53);
        let t57 = 0.0197516734986138 * t43 * t45 * t54;
        let t58 = rmath::ln(2.0);
        let t59 = 1.0 - t58;
        let t60 = M_PI * M_PI;
        let t61 = 1.0 / t60;
        let t62 = t59 * t61;
        let t63 = t34 * t34;
        let t64 = piecewise3(t33, t63, 1.0);
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
        let t76 = rmath::sqrt(sigma[ip]);
        let t78 = 1.0 / t7 / rho[ip];
        let t80 = t39 * t39;
        let t81 = 1.0 / t64;
        let t82 = t80 * t81;
        let t83 = 1.0 / t13;
        let t84 = t82 * t83;
        let t85 = t76 * t78 * t84;
        let t87 = 4.5 + t85 / 4.0;
        let t88 = t5 * t87;
        let t90 = 4.5 + 0.36675 * t85;
        let t91 = 1.0 / t90;
        let t93 = t75 * t88 * t91;
        let t96 = 1.0 / t59;
        let t99 = 1.0 / t66;
        let t100 = t60 * t99;
        let t102 = rmath::exp(-(-t32 + t57) * t96 * t100);
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
        let t126 = t73 * t93 / 96.0 + 0.0002143700905903487 * t112 * t123;
        let t127 = t126 * t96;
        let t130 = 1.0 + 0.6585449182935511 * t105 * t126;
        let t131 = 1.0 / t130;
        let t134 = 1.0 + 0.6585449182935511 * t127 * t131;
        let t135 = rmath::ln(t134);
        let t137 = t62 * t66 * t135;
        let tzk0 = -t32 + t57 + t137;
        zk[ip] += tzk0;
    }
}
