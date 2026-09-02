//! MGGA_C_BC95 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_bc95.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_bc95_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_copp: f64,
    param_css: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t4 = 1.0 <= zeta_threshold;
        let t5 = rho[ip] / 2.0 <= dens_threshold || t4;
        let t6 = piecewise3(t4, zeta_threshold, 1.0);
        let t7 = M_CBRT3;
        let t8 = 1.0 / M_PI;
        let t9 = pow_1_3(t8);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = t10 * t12;
        let t14 = pow_1_3(rho[ip]);
        let t15 = 1.0 / t14;
        let t16 = M_CBRT2;
        let t18 = pow_1_3(zeta_threshold);
        let t20 = piecewise3(t4, 1.0 / t18, 1.0);
        let t22 = t13 * t15 * t16 * t20;
        let t24 = 1.0 + 0.053425 * t22;
        let t25 = rmath::sqrt(t22);
        let t28 = pow_3_2(t22);
        let t30 = t7 * t7;
        let t31 = t9 * t9;
        let t32 = t30 * t31;
        let t33 = t32 * t11;
        let t34 = t14 * t14;
        let t35 = 1.0 / t34;
        let t36 = t16 * t16;
        let t38 = t20 * t20;
        let t40 = t33 * t35 * t36 * t38;
        let t42 = 3.79785 * t25 + 0.8969 * t22 + 0.204775 * t28 + 0.123235 * t40;
        let t45 = 1.0 + 16.081979498692537 / t42;
        let t46 = rmath::ln(t45);
        let t48 = 0.0621814 * t24 * t46;
        let t50 = t18 * zeta_threshold;
        let t52 = piecewise3(2.0 <= zeta_threshold, t50, 2.0 * t16);
        let t54 = piecewise3(0.0 <= zeta_threshold, t50, 0.0);
        let t58 = 1.0 / (2.0 * t16 - 2.0);
        let t59 = (t52 + t54 - 2.0) * t58;
        let t61 = 1.0 + 0.05137 * t22;
        let t66 = 7.05945 * t25 + 1.549425 * t22 + 0.420775 * t28 + 0.1562925 * t40;
        let t69 = 1.0 + 32.16395899738507 / t66;
        let t70 = rmath::ln(t69);
        let t74 = 1.0 + 0.0278125 * t22;
        let t79 = 5.1785 * t25 + 0.905775 * t22 + 0.1100325 * t28 + 0.1241775 * t40;
        let t82 = 1.0 + 29.608749977793437 / t79;
        let t83 = rmath::ln(t82);
        let t84 = t74 * t83;
        let t93 = piecewise3(t5, 0.0, t6 * (-t48 + t59 * (-0.0310907 * t61 * t70 + t48 - 0.0197516734986138 * t84) + 0.0197516734986138 * t59 * t84) / 2.0);
        let t94 = t93 * tau[ip];
        let t96 = 1.0 / t34 / rho[ip];
        let t97 = t36 * t96;
        let t99 = 1.0 / rho[ip];
        let t101 = 1.0 / tau[ip];
        let t104 = 1.0 - sigma[ip] * t99 * t101 / 8.0;
        let t105 = M_CBRT6;
        let t106 = t104 * t105;
        let t107 = M_PI * M_PI;
        let t108 = pow_1_3(t107);
        let t109 = t108 * t108;
        let t110 = 1.0 / t109;
        let t111 = param_css * sigma[ip];
        let t112 = rho[ip] * rho[ip];
        let t114 = 1.0 / t34 / t112;
        let t115 = t36 * t114;
        let t117 = t111 * t115 + 1.0;
        let t118 = t117 * t117;
        let t119 = 1.0 / t118;
        let t120 = t110 * t119;
        let t121 = t106 * t120;
        let t123 = 10.0 / 9.0 * t94 * t97 * t121;
        let t125 = t10 * t12 * t15;
        let t127 = 1.0 + 0.053425 * t125;
        let t128 = rmath::sqrt(t125);
        let t131 = pow_3_2(t125);
        let t134 = t32 * t11 * t35;
        let t136 = 3.79785 * t128 + 0.8969 * t125 + 0.204775 * t131 + 0.123235 * t134;
        let t139 = 1.0 + 16.081979498692537 / t136;
        let t140 = rmath::ln(t139);
        let t143 = piecewise3(t4, t50, 1.0);
        let t146 = (2.0 * t143 - 2.0) * t58;
        let t148 = 1.0 + 0.0278125 * t125;
        let t153 = 5.1785 * t128 + 0.905775 * t125 + 0.1100325 * t131 + 0.1241775 * t134;
        let t156 = 1.0 + 29.608749977793437 / t153;
        let t157 = rmath::ln(t156);
        let t162 = -0.0621814 * t127 * t140 + 0.0197516734986138 * t146 * t148 * t157 - 2.0 * t93;
        let t166 = 2.0 * param_copp * sigma[ip] * t115 + 1.0;
        let t167 = 1.0 / t166;
        let t168 = t162 * t167;
        let tzk0 = t123 + t168;
        zk[ip] += tzk0;
    }
}
