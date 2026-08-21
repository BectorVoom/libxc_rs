//! GGA_C_SG4 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sg4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_sg4_exc_unpol(
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
        let t58 = t34 * t34;
        let t59 = piecewise3(t33, t58, 1.0);
        let t60 = rmath::sqrt(sigma[ip]);
        let t61 = t60 * sigma[ip];
        let t62 = rho[ip] * rho[ip];
        let t63 = t62 * t62;
        let t64 = 1.0 / t63;
        let t66 = t59 * t59;
        let t67 = t66 * t59;
        let t68 = 1.0 / t67;
        let t70 = 1.0 / t13 / t10;
        let t71 = t68 * t70;
        let t74 = rmath::pow(t59, 0.05 * t61 * t64 * t71);
        let t75 = rmath::ln(2.0);
        let t76 = 1.0 - t75;
        let t77 = t74 * t76;
        let t78 = M_PI * M_PI;
        let t79 = 1.0 / t78;
        let t80 = t79 * t67;
        let t82 = 1.0 / t7 / rho[ip];
        let t84 = t39 * t39;
        let t86 = 1.0 / t59;
        let t87 = 1.0 / t13;
        let t88 = t86 * t87;
        let t90 = rmath::exp(-t24 / 4.0);
        let t91 = 1.0 - t90;
        let t92 = t88 * t91;
        let t95 = 0.07963845034287749 + 0.0175 * t60 * t82 * t84 * t92;
        let t97 = 1.0 / t7 / t62;
        let t100 = 1.0 / t66;
        let t102 = 1.0 / t3;
        let t104 = t100 * t18 * t102 * t5;
        let t107 = 1.0 / t76;
        let t108 = t95 * t107;
        let t113 = rmath::exp(-(-t32 + t57) * t107 * t78 * t68);
        let t114 = t113 - 1.0;
        let t115 = 1.0 / t114;
        let t116 = t78 * t115;
        let t117 = sigma[ip] * sigma[ip];
        let t118 = t116 * t117;
        let t119 = t108 * t118;
        let t121 = 1.0 / t21 / t63;
        let t122 = t121 * t84;
        let t123 = t66 * t66;
        let t124 = 1.0 / t123;
        let t126 = 1.0 / t19;
        let t127 = t1 * t126;
        let t128 = t127 * t6;
        let t129 = t122 * t124 * t128;
        let t132 = sigma[ip] * t97 * t39 * t104 / 96.0 + t119 * t129 / 3072.0;
        let t133 = t95 * t132;
        let t134 = t107 * t78;
        let t135 = t116 * t132;
        let t137 = t108 * t135 + 1.0;
        let t138 = 1.0 / t137;
        let t139 = t134 * t138;
        let t141 = t133 * t139 + 1.0;
        let t142 = rmath::ln(t141);
        let t144 = t77 * t80 * t142;
        let tzk0 = -t32 + t57 + t144;
        zk[ip] += tzk0;
    }
}
