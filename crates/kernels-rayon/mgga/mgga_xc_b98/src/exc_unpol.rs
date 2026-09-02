//! MGGA_XC_B98 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_b98.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_b98_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = pow_1_3(zeta_threshold);
        let t8 = t7 * zeta_threshold;
        let t9 = piecewise3(t6, t8, 1.0);
        let t10 = pow_1_3(rho[ip]);
        let t11 = t9 * t10;
        let t12 = M_CBRT2;
        let t13 = t12 * t12;
        let t14 = tau[ip] * t13;
        let t15 = t10 * t10;
        let t17 = 1.0 / t15 / rho[ip];
        let t19 = sigma[ip] * t13;
        let t20 = rho[ip] * rho[ip];
        let t22 = 1.0 / t15 / t20;
        let t25 = lapl[ip] * t13;
        let t29 = M_CBRT6;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = t32 * t32;
        let t34 = 1.0 / t33;
        let t37 = 1.0 - 5.0 / 9.0 * (t14 * t17 - t19 * t22 / 8.0 - t25 * t17 / 4.0) * t29 * t34;
        let t38 = t37 * t37;
        let t40 = 1.0 + 0.0121 * t38;
        let t41 = rmath::sqrt(t40);
        let t42 = 1.0 / t41;
        let t45 = 1.0 / t40;
        let t48 = 0.8085 + 0.073502 * t37 * t42 + 0.0017182 * t38 * t45;
        let t51 = 3.0 / 4.0 * t5 * t11 * t48;
        let t54 = rho[ip] / 2.0 <= dens_threshold || t6;
        let t55 = piecewise3(t6, zeta_threshold, 1.0);
        let t56 = 1.0 / M_PI;
        let t57 = pow_1_3(t56);
        let t58 = t2 * t57;
        let t59 = M_CBRT4;
        let t60 = t59 * t59;
        let t61 = t58 * t60;
        let t62 = 1.0 / t10;
        let t65 = piecewise3(t6, 1.0 / t7, 1.0);
        let t67 = t61 * t62 * t12 * t65;
        let t69 = 1.0 + 0.053425 * t67;
        let t70 = rmath::sqrt(t67);
        let t73 = pow_3_2(t67);
        let t75 = t2 * t2;
        let t76 = t57 * t57;
        let t77 = t75 * t76;
        let t78 = t77 * t59;
        let t79 = 1.0 / t15;
        let t81 = t65 * t65;
        let t83 = t78 * t79 * t13 * t81;
        let t85 = 3.79785 * t70 + 0.8969 * t67 + 0.204775 * t73 + 0.123235 * t83;
        let t88 = 1.0 + 16.081824322151103 / t85;
        let t89 = rmath::ln(t88);
        let t91 = 0.062182 * t69 * t89;
        let t94 = piecewise3(2.0 <= zeta_threshold, t8, 2.0 * t12);
        let t96 = piecewise3(0.0 <= zeta_threshold, t8, 0.0);
        let t100 = 1.0 / (2.0 * t12 - 2.0);
        let t101 = (t94 + t96 - 2.0) * t100;
        let t103 = 1.0 + 0.05137 * t67;
        let t108 = 7.05945 * t70 + 1.549425 * t67 + 0.420775 * t73 + 0.1562925 * t83;
        let t111 = 1.0 + 32.1646831778707 / t108;
        let t112 = rmath::ln(t111);
        let t116 = 1.0 + 0.0278125 * t67;
        let t121 = 5.1785 * t70 + 0.905775 * t67 + 0.1100325 * t73 + 0.1241775 * t83;
        let t124 = 1.0 + 29.608574643216677 / t121;
        let t125 = rmath::ln(t124);
        let t126 = t116 * t125;
        let t135 = piecewise3(t54, 0.0, t55 * (-t91 + t101 * (-0.03109 * t103 * t112 + t91 - 0.019751789702565206 * t126) + 0.019751789702565206 * t101 * t126) / 2.0);
        let t137 = 1.0 + 2.56 * t38;
        let t138 = rmath::sqrt(t137);
        let t139 = 1.0 / t138;
        let t142 = 1.0 / t137;
        let t145 = 0.2606 - 1.53728 * t37 * t139 + 2.309888 * t38 * t142;
        let t146 = t135 * t145;
        let t147 = 1.0 / rho[ip];
        let t148 = sigma[ip] * t147;
        let t149 = 1.0 / tau[ip];
        let t152 = 1.0 - t148 * t149 / 8.0;
        let t154 = 2.0 * t146 * t152;
        let t156 = t58 * t60 * t62;
        let t158 = 1.0 + 0.053425 * t156;
        let t159 = rmath::sqrt(t156);
        let t162 = pow_3_2(t156);
        let t165 = t77 * t59 * t79;
        let t167 = 3.79785 * t159 + 0.8969 * t156 + 0.204775 * t162 + 0.123235 * t165;
        let t170 = 1.0 + 16.081824322151103 / t167;
        let t171 = rmath::ln(t170);
        let t176 = (2.0 * t9 - 2.0) * t100;
        let t178 = 1.0 + 0.0278125 * t156;
        let t183 = 5.1785 * t159 + 0.905775 * t156 + 0.1100325 * t162 + 0.1241775 * t165;
        let t186 = 1.0 + 29.608574643216677 / t183;
        let t187 = rmath::ln(t186);
        let t192 = -0.062182 * t158 * t171 + 0.019751789702565206 * t176 * t178 * t187 - 2.0 * t135;
        let t194 = 1.0 + 0.0196 * t38;
        let t195 = rmath::sqrt(t194);
        let t196 = 1.0 / t195;
        let t199 = 1.0 / t194;
        let t202 = 1.2033 - 0.318038 * t37 * t196 + 0.01880816 * t38 * t199;
        let t203 = t192 * t202;
        let tzk0 = -t51 + t154 + t203;
        zk[ip] += tzk0;
    }
}
