//! GGA_C_Q2D exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_q2d.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_q2d_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = sigma[ip] * sigma[ip];
        let t2 = rho[ip] * rho[ip];
        let t3 = t2 * t2;
        let t4 = pow_1_3(rho[ip]);
        let t5 = t4 * t4;
        let t7 = 1.0 / t5 / t3;
        let t8 = t1 * t7;
        let t9 = M_CBRT2;
        let t10 = t9 * t9;
        let t11 = 1.0 <= zeta_threshold;
        let t12 = pow_1_3(zeta_threshold);
        let t13 = t12 * t12;
        let t14 = piecewise3(t11, t13, 1.0);
        let t15 = t14 * t14;
        let t16 = t15 * t15;
        let t17 = 1.0 / t16;
        let t18 = t10 * t17;
        let t20 = M_CBRT3;
        let t21 = 1.0 / M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT4;
        let t27 = t26 * t26;
        let t29 = 1.0 / t4 / t2;
        let t32 = 1.0 / t15;
        let t33 = t20 * t20;
        let t35 = 1.0 / t22;
        let t37 = t32 * t33 * t35 * t26;
        let t39 = sigma[ip] * t29 * t9 * t37 / 96.0;
        let t40 = 1.0 + t39;
        let t41 = t27 * t40;
        let t42 = t1 * sigma[ip];
        let t43 = t2 * rho[ip];
        let t44 = t3 * t43;
        let t45 = 1.0 / t44;
        let t47 = t16 * t15;
        let t48 = 1.0 / t47;
        let t49 = t48 * M_PI;
        let t52 = 1000000.0 + t42 * t45 * t49 / 12288.0;
        let t53 = 1.0 / t52;
        let t55 = t25 * t41 * t53;
        let t58 = 1.0 - t8 * t18 * t55 / 3072.0;
        let t59 = t20 * t22;
        let t60 = 1.0 / t4;
        let t62 = t59 * t27 * t60;
        let t64 = 1.0 + 0.053425 * t62;
        let t65 = rmath::sqrt(t62);
        let t68 = pow_3_2(t62);
        let t70 = t33 * t23;
        let t71 = 1.0 / t5;
        let t73 = t70 * t26 * t71;
        let t75 = 3.79785 * t65 + 0.8969 * t62 + 0.204775 * t68 + 0.123235 * t73;
        let t78 = 1.0 + 16.081979498692537 / t75;
        let t79 = rmath::ln(t78);
        let t81 = 0.0621814 * t64 * t79;
        let t83 = piecewise3(t11, t12 * zeta_threshold, 1.0);
        let t89 = (2.0 * t83 - 2.0) / (2.0 * t9 - 2.0);
        let t91 = 1.0 + 0.0278125 * t62;
        let t96 = 5.1785 * t65 + 0.905775 * t62 + 0.1100325 * t68 + 0.1241775 * t73;
        let t99 = 1.0 + 29.608749977793437 / t96;
        let t100 = rmath::ln(t99);
        let t103 = 0.0197516734986138 * t89 * t91 * t100;
        let t104 = rmath::ln(2.0);
        let t105 = 1.0 - t104;
        let t106 = M_PI * M_PI;
        let t107 = 1.0 / t106;
        let t108 = t105 * t107;
        let t109 = t15 * t14;
        let t110 = 1.0 / t105;
        let t113 = 1.0 / t109;
        let t114 = t106 * t113;
        let t116 = rmath::exp(-(-t81 + t103) * t110 * t114);
        let t117 = t116 - 1.0;
        let t118 = 1.0 / t117;
        let t119 = t110 * t118;
        let t121 = t25 * t27;
        let t122 = t18 * t121;
        let t125 = t39 + 0.0002143700905903487 * t119 * t8 * t122;
        let t126 = t125 * t110;
        let t129 = 1.0 + 0.6585449182935511 * t119 * t125;
        let t130 = 1.0 / t129;
        let t133 = 1.0 + 0.6585449182935511 * t126 * t130;
        let t134 = rmath::ln(t133);
        let t137 = t108 * t109 * t134 + t103 - t81;
        let t138 = t58 * t137;
        let t139 = t18 * t20;
        let t140 = t8 * t139;
        let t141 = t24 * t27;
        let t142 = t40 * t53;
        let t143 = rmath::sqrt(3.0);
        let t145 = M_CBRT6;
        let t146 = t145 * t145;
        let t147 = pow_1_3(t106);
        let t148 = 1.0 / t147;
        let t149 = t146 * t148;
        let t150 = rmath::sqrt(sigma[ip]);
        let t152 = 1.0 / t4 / rho[ip];
        let t154 = t149 * t150 * t152;
        let t155 = rmath::sqrt(t154);
        let t156 = t143 * t60 * t155;
        let t158 = 1.0 / t2;
        let t159 = t158 * t146;
        let t160 = t148 * t150;
        let t161 = t159 * t160;
        let t163 = 1.0 / rho[ip];
        let t164 = t143 * t163;
        let t165 = t155 * t154;
        let t166 = t164 * t165;
        let t168 = 0.0245130624 * t156 + 0.0138498611712 * t161 + 0.0002310999830832 * t166;
        let t170 = pow_3_2(t156);
        let t174 = 0.2846248 * t156 - 0.0031313960595450714 * t170 + 0.08226186096 * t161 + 0.00120051939264 * t166;
        let t176 = 1.0 + 1.0 / t174;
        let t177 = rmath::ln(t176);
        let t180 = rmath::exp(-0.3801624 * t156);
        let t182 = M_SQRT2;
        let t183 = (t180 - 1.0) * t182;
        let t184 = t183 * t143;
        let t185 = 1.0 / t155;
        let t187 = rmath::sqrt(zeta_threshold);
        let t189 = piecewise3(t11, t187 * zeta_threshold, 1.0);
        let t190 = t189 - 1.0;
        let t194 = -0.1925 + t168 * t177 - 0.4981375370638352 * t184 * t4 * t185 * t190;
        let t196 = t141 * t142 * t194;
        let t198 = t140 * t196 / 3072.0;
        let tzk0 = t138 + t198;
        zk[ip] += tzk0;
    }
}
