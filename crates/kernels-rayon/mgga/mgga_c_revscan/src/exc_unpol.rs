//! MGGA_C_REVSCAN exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_revscan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_revscan_exc_unpol(
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
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = pow_1_3(rho[ip]);
        let t11 = t5 * t7 / t8;
        let t13 = 1.0 + 0.053425 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t2 * t2;
        let t20 = t4 * t4;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t6 / t22;
        let t27 = 3.79785 * t14 + 0.8969 * t11 + 0.204775 * t17 + 0.123235 * t25;
        let t30 = 1.0 + 16.081979498692537 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.0621814 * t13 * t31;
        let t34 = 1.0 <= zeta_threshold;
        let t35 = pow_1_3(zeta_threshold);
        let t37 = piecewise3(t34, t35 * zeta_threshold, 1.0);
        let t39 = 2.0 * t37 - 2.0;
        let t40 = M_CBRT2;
        let t41 = t40 - 1.0;
        let t43 = 1.0 / t41 / 2.0;
        let t44 = t39 * t43;
        let t46 = 1.0 + 0.0278125 * t11;
        let t51 = 5.1785 * t14 + 0.905775 * t11 + 0.1100325 * t17 + 0.1241775 * t25;
        let t54 = 1.0 + 29.608749977793437 / t51;
        let t55 = f64::ln(t54);
        let t58 = 0.0197516734986138 * t44 * t46 * t55;
        let t59 = f64::ln(2.0);
        let t60 = 1.0 - t59;
        let t61 = M_PI * M_PI;
        let t63 = t60 / t61;
        let t64 = t35 * t35;
        let t65 = piecewise3(t34, t64, 1.0);
        let t66 = t65 * t65;
        let t67 = t66 * t65;
        let t69 = 1.0 + 0.025 * t11;
        let t71 = 1.0 + 0.04445 * t11;
        let t72 = 1.0 / t71;
        let t73 = t69 * t72;
        let t74 = 1.0 / t60;
        let t77 = 1.0 / t67;
        let t78 = t61 * t77;
        let t80 = f64::exp(-(-t33 + t58) * t74 * t78);
        let t81 = t80 - 1.0;
        let t82 = 1.0 / t81;
        let t83 = t74 * t82;
        let t84 = t83 * sigma[ip];
        let t85 = t73 * t84;
        let t86 = rho[ip] * rho[ip];
        let t88 = 1.0 / t8 / t86;
        let t89 = t88 * t40;
        let t90 = 1.0 / t66;
        let t92 = 1.0 / t4;
        let t93 = t19 * t92;
        let t94 = t93 * t6;
        let t95 = t89 * t90 * t94;
        let t98 = 1.0 + 0.054878743191129266 * t85 * t95;
        let t99 = pow_1_4(t98);
        let t102 = t69 * t69;
        let t103 = t71 * t71;
        let t104 = 1.0 / t103;
        let t105 = t102 * t104;
        let t106 = t60 * t60;
        let t107 = 1.0 / t106;
        let t108 = t81 * t81;
        let t109 = 1.0 / t108;
        let t110 = t107 * t109;
        let t111 = sigma[ip] * sigma[ip];
        let t112 = t110 * t111;
        let t113 = t105 * t112;
        let t114 = t86 * t86;
        let t116 = 1.0 / t22 / t114;
        let t117 = t40 * t40;
        let t118 = t116 * t117;
        let t119 = t66 * t66;
        let t120 = 1.0 / t119;
        let t121 = t118 * t120;
        let t122 = 1.0 / t20;
        let t123 = t2 * t122;
        let t124 = t123 * t7;
        let t125 = t121 * t124;
        let t128 = 1.0 + 0.011293786703392187 * t113 * t125;
        let t129 = f64::powf(t128, 1.0 / 8.0);
        let t132 = 1.0 - 1.0 / t99 / 2.0 - 1.0 / t129 / 2.0;
        let t135 = 1.0 + 1.0 * t132 * t81;
        let t136 = f64::ln(t135);
        let t138 = t63 * t67 * t136;
        let t140 = 1.0 / t22 / rho[ip];
        let t143 = 1.0 / t22 / t86;
        let t147 = M_CBRT6;
        let t149 = pow_1_3(t61);
        let t150 = t149 * t149;
        let t151 = 1.0 / t150;
        let t152 = t151 * t117;
        let t154 = 5.0 / 9.0 * (tau[ip] * t140 - sigma[ip] * t143 / 8.0) * t147 * t152;
        let t155 = t154 <= 1.0;
        let t156 = f64::ln(f64::EPSILON);
        let t159 = t156 / (-t156 + 1.131);
        let t160 = -t159 < t154;
        let t161 = t154 < -t159;
        let t162 = piecewise3(t161, t154, -t159);
        let t163 = 1.0 - t162;
        let t164 = 1.0 / t163;
        let t167 = f64::exp(-1.131 * t162 * t164);
        let t168 = piecewise3(t160, 0.0, t167);
        let t170 = f64::ln(0.7299270072992701 * f64::EPSILON);
        let t173 = (-t170 + 1.7) / t170;
        let t174 = t154 < -t173;
        let t175 = piecewise3(t174, -t173, t154);
        let t176 = 1.0 - t175;
        let t179 = f64::exp(1.7 / t176);
        let t181 = piecewise3(t174, 0.0, -1.37 * t179);
        let t182 = piecewise3(t155, t168, t181);
        let t185 = 1.0 + 0.033115 * t14 + 0.04168 * t11;
        let t186 = 1.0 / t185;
        let t189 = f64::exp(1.0 * t186);
        let t190 = t189 - 1.0;
        let t191 = t147 * t151;
        let t192 = t117 * sigma[ip];
        let t196 = 1.0 + 0.04267528420875272 * t191 * t192 * t143;
        let t197 = pow_1_4(t196);
        let t200 = t147 * t147;
        let t202 = 1.0 / t149 / t61;
        let t203 = t200 * t202;
        let t204 = t40 * t111;
        let t205 = t114 * rho[ip];
        let t207 = 1.0 / t8 / t205;
        let t211 = 1.0 + 0.004552949705744548 * t203 * t204 * t207;
        let t212 = f64::powf(t211, 1.0 / 8.0);
        let t215 = 1.0 - 1.0 / t197 / 2.0 - 1.0 / t212 / 2.0;
        let t217 = t190 * t215 + 1.0;
        let t218 = f64::ln(t217);
        let t224 = 1.0 - 2.363 * t41 * t39 * t43;
        let t226 = (-0.030197 * t186 + 0.030197 * t218) * t224 + t33 - t58 - t138;
        let t227 = t182 * t226;
        let tzk0 = -t33 + t58 + t138 + t227;
        zk[ip] += tzk0;
    }
}
