//! MGGA_C_RSCAN exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rscan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_rscan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = rho0 + rho1;
        let t9 = pow_1_3(t8);
        let t12 = t5 * t7 / t9;
        let t14 = 1.0 + 0.053425 * t12;
        let t15 = rmath::sqrt(t12);
        let t18 = pow_3_2(t12);
        let t20 = t2 * t2;
        let t21 = t4 * t4;
        let t22 = t20 * t21;
        let t23 = t9 * t9;
        let t24 = 1.0 / t23;
        let t26 = t22 * t6 * t24;
        let t28 = 3.79785 * t15 + 0.8969 * t12 + 0.204775 * t18 + 0.123235 * t26;
        let t31 = 1.0 + 16.081979498692537 / t28;
        let t32 = rmath::ln(t31);
        let t34 = 0.0621814 * t14 * t32;
        let t35 = rho0 - rho1;
        let t36 = t35 * t35;
        let t37 = t36 * t36;
        let t38 = t8 * t8;
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t41 = t37 * t40;
        let t42 = 1.0 / t8;
        let t43 = t35 * t42;
        let t44 = 1.0 + t43;
        let t45 = t44 <= zeta_threshold;
        let t46 = pow_1_3(zeta_threshold);
        let t47 = t46 * zeta_threshold;
        let t48 = pow_1_3(t44);
        let t49 = t48 * t44;
        let t50 = piecewise3(t45, t47, t49);
        let t51 = 1.0 - t43;
        let t52 = t51 <= zeta_threshold;
        let t53 = pow_1_3(t51);
        let t54 = t53 * t51;
        let t55 = piecewise3(t52, t47, t54);
        let t56 = t50 + t55 - 2.0;
        let t57 = M_CBRT2;
        let t58 = t57 - 1.0;
        let t60 = 1.0 / t58 / 2.0;
        let t61 = t56 * t60;
        let t63 = 1.0 + 0.05137 * t12;
        let t68 = 7.05945 * t15 + 1.549425 * t12 + 0.420775 * t18 + 0.1562925 * t26;
        let t71 = 1.0 + 32.16395899738507 / t68;
        let t72 = rmath::ln(t71);
        let t76 = 1.0 + 0.0278125 * t12;
        let t81 = 5.1785 * t15 + 0.905775 * t12 + 0.1100325 * t18 + 0.1241775 * t26;
        let t84 = 1.0 + 29.608749977793437 / t81;
        let t85 = rmath::ln(t84);
        let t86 = t76 * t85;
        let t88 = -0.0310907 * t63 * t72 + t34 - 0.0197516734986138 * t86;
        let t89 = t61 * t88;
        let t90 = t41 * t89;
        let t92 = 0.0197516734986138 * t61 * t86;
        let t93 = rmath::ln(2.0);
        let t94 = 1.0 - t93;
        let t95 = M_PI * M_PI;
        let t97 = t94 / t95;
        let t98 = t46 * t46;
        let t99 = t48 * t48;
        let t100 = piecewise3(t45, t98, t99);
        let t101 = t53 * t53;
        let t102 = piecewise3(t52, t98, t101);
        let t104 = t100 / 2.0 + t102 / 2.0;
        let t105 = t104 * t104;
        let t106 = t105 * t104;
        let t108 = 1.0 + 0.025 * t12;
        let t110 = 1.0 + 0.04445 * t12;
        let t111 = 1.0 / t110;
        let t112 = t108 * t111;
        let t113 = 1.0 / t94;
        let t115 = (-t34 + t90 + t92) * t113;
        let t116 = 1.0 / t106;
        let t117 = t95 * t116;
        let t119 = rmath::exp(-t115 * t117);
        let t120 = t119 - 1.0;
        let t121 = 1.0 / t120;
        let t122 = t113 * t121;
        let t124 = sigma0 + 2.0 * sigma1 + sigma2;
        let t125 = t122 * t124;
        let t126 = t112 * t125;
        let t127 = t9 * t38;
        let t128 = 1.0 / t127;
        let t129 = t128 * t57;
        let t130 = 1.0 / t105;
        let t132 = 1.0 / t4;
        let t133 = t20 * t132;
        let t134 = t133 * t6;
        let t138 = 1.0 + 0.027439371595564633 * t126 * t129 * t130 * t134;
        let t139 = pow_1_4(t138);
        let t141 = 1.0 - 1.0 / t139;
        let t144 = 1.0 + 1.0 * t141 * t120;
        let t145 = rmath::ln(t144);
        let t147 = t97 * t106 * t145;
        let t148 = t39 * t8;
        let t149 = pow_1_3(rho0);
        let t150 = t149 * t149;
        let t152 = 1.0 / t150 / rho0;
        let t153 = tau0 * t152;
        let t154 = t44 / 2.0;
        let t155 = pow_1_3(t154);
        let t156 = t155 * t155;
        let t157 = t156 * t154;
        let t159 = pow_1_3(rho1);
        let t160 = t159 * t159;
        let t162 = 1.0 / t160 / rho1;
        let t163 = tau1 * t162;
        let t164 = t51 / 2.0;
        let t165 = pow_1_3(t164);
        let t166 = t165 * t165;
        let t167 = t166 * t164;
        let t169 = t23 * t38;
        let t170 = 1.0 / t169;
        let t173 = t153 * t157 + t163 * t167 - t124 * t170 / 8.0;
        let t174 = 0.0 < t173;
        let t175 = piecewise3(t174, t173, 0.0);
        let t176 = t175 * t175;
        let t177 = t176 * t175;
        let t178 = t148 * t177;
        let t179 = M_CBRT6;
        let t180 = t179 * t179;
        let t181 = pow_1_3(t95);
        let t182 = t181 * t181;
        let t183 = t180 * t182;
        let t184 = t23 * t8;
        let t187 = t57 * t57;
        let t189 = 3.0 / 10.0 * t183 * t184 + 0.0001 * t187;
        let t190 = t189 * t189;
        let t191 = t190 * t189;
        let t192 = 1.0 / t191;
        let t193 = t157 + t167;
        let t194 = t193 * t193;
        let t195 = t194 * t193;
        let t196 = 1.0 / t195;
        let t197 = t192 * t196;
        let t198 = t38 * t8;
        let t199 = t9 * t198;
        let t200 = t199 * t176;
        let t201 = 1.0 / t190;
        let t202 = 1.0 / t194;
        let t203 = t201 * t202;
        let t205 = t200 * t203 + 0.001;
        let t206 = 1.0 / t205;
        let t207 = t197 * t206;
        let t208 = t178 * t207;
        let t209 = t208 <= 2.5;
        let t210 = 2.5 < t208;
        let t211 = piecewise3(t210, 2.5, t208);
        let t213 = t211 * t211;
        let t215 = t213 * t211;
        let t217 = t213 * t213;
        let t219 = t217 * t211;
        let t221 = t217 * t213;
        let t226 = piecewise3(t210, t208, 2.5);
        let t227 = 1.0 - t226;
        let t230 = rmath::exp(1.5 / t227);
        let t232 = piecewise3(t209, 1.0 - 0.64 * t211 - 0.4352 * t213 - 1.535685604549 * t215 + 3.061560252175 * t217 - 1.915710236206 * t219 + 0.516884468372 * t221 - 0.051848879792 * t217 * t215, -0.7 * t230);
        let t235 = 1.0 + 0.04445 * t15 + 0.03138525 * t12;
        let t236 = 1.0 / t235;
        let t239 = rmath::exp(1.0 * t236);
        let t240 = t239 - 1.0;
        let t241 = 1.0 / t182;
        let t242 = t179 * t241;
        let t243 = t187 * t124;
        let t247 = 1.0 + 0.02133764210437636 * t242 * t243 * t170;
        let t248 = pow_1_4(t247);
        let t250 = 1.0 - 1.0 / t248;
        let t252 = t240 * t250 + 1.0;
        let t253 = rmath::ln(t252);
        let t255 = -0.0285764 * t236 + 0.0285764 * t253;
        let t259 = 1.0 - 2.363 * t58 * t56 * t60;
        let t260 = t255 * t259;
        let t261 = t37 * t37;
        let t262 = t261 * t37;
        let t263 = t39 * t39;
        let t264 = t263 * t39;
        let t265 = 1.0 / t264;
        let t267 = -t262 * t265 + 1.0;
        let t269 = t260 * t267 - t147 + t34 - t90 - t92;
        let t270 = t232 * t269;
        let tzk0 = -t34 + t90 + t92 + t147 + t270;
        zk[ip] += tzk0;
    }
}
