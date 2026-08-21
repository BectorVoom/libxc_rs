//! GGA_C_ZVPBEINT exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeint.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_zvpbeint_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_alpha: f64,
    param_omega: f64,
    param_beta: f64,
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
        let t58 = rmath::sqrt(sigma[ip]);
        let t59 = t58 * sigma[ip];
        let t60 = param_alpha * t59;
        let t61 = rho[ip] * rho[ip];
        let t62 = t61 * t61;
        let t63 = 1.0 / t62;
        let t66 = 1.0 / t13 / t10;
        let t67 = 1.0 / t3;
        let t68 = t18 * t67;
        let t70 = t68 * t5 * t7;
        let t71 = rmath::sqrt(t70);
        let t72 = t66 * t71;
        let t74 = piecewise3(1e-20 < 0.0, 0.0, 1e-20);
        let t76 = rmath::pow(t74, param_omega / 2.0);
        let t77 = t72 * t76;
        let t80 = rmath::exp(-t60 * t63 * t77 / 16.0);
        let t81 = rmath::ln(2.0);
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
        let t108 = rmath::exp(-(-t32 + t57) * t101 * t84 * t105);
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
        let t138 = rmath::ln(t137);
        let t139 = t90 * t138;
        let t140 = t83 * t139;
        let tzk0 = -t32 + t57 + t140;
        zk[ip] += tzk0;
    }
}
