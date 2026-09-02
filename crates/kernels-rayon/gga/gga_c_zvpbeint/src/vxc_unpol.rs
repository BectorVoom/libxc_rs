//! GGA_C_ZVPBEINT vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeint.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_zvpbeint_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t142 = 1.0 / t7 / rho[ip];
        let t143 = t6 * t142;
        let t145 = t4 * t143 * t30;
        let t146 = 0.0011073470983333333 * t145;
        let t147 = t26 * t26;
        let t148 = 1.0 / t147;
        let t149 = t12 * t148;
        let t151 = 1.0 / t13 * t1;
        let t152 = t3 * t6;
        let t153 = t152 * t142;
        let t154 = t151 * t153;
        let t156 = t4 * t143;
        let t158 = rmath::sqrt(t10);
        let t159 = t158 * t1;
        let t160 = t159 * t153;
        let t164 = t5 / t21 / rho[ip];
        let t165 = t20 * t164;
        let t167 = -0.632975 * t154 - 0.29896666666666666 * t156 - 0.1023875 * t160 - 0.08215666666666667 * t165;
        let t168 = 1.0 / t29;
        let t169 = t167 * t168;
        let t170 = t149 * t169;
        let t171 = 1.0 * t170;
        let t172 = t43 * t1;
        let t175 = t172 * t152 * t142 * t54;
        let t176 = 0.00018311447306006544 * t175;
        let t177 = t43 * t45;
        let t178 = t50 * t50;
        let t179 = 1.0 / t178;
        let t184 = -0.8630833333333333 * t154 - 0.301925 * t156 - 0.05501625 * t160 - 0.082785 * t165;
        let t186 = 1.0 / t53;
        let t187 = t179 * t184 * t186;
        let t188 = t177 * t187;
        let t189 = 0.5848223622634646 * t188;
        let t190 = t62 * rho[ip];
        let t191 = 1.0 / t190;
        let t196 = 1.0 / t7 / t190;
        let t199 = 1.0 / t13 / t24 / 4.0;
        let t200 = t196 * t199;
        let t202 = t71 * t76;
        let t203 = t4 * t6;
        let t204 = t202 * t203;
        let t207 = t116 * t66;
        let t209 = 1.0 / t71;
        let t210 = t209 * t76;
        let t211 = t68 * t5;
        let t212 = t210 * t211;
        let t215 = t60 * t191 * t77 / 4.0 - t60 * t200 * t204 / 32.0 - t60 * t207 * t212 / 96.0;
        let t216 = t215 * t80;
        let t217 = t216 * t82;
        let t218 = t217 * t139;
        let t219 = t83 * t85;
        let t220 = t61 * rho[ip];
        let t222 = 1.0 / t7 / t220;
        let t227 = t82 * t82;
        let t228 = 1.0 / t227;
        let t229 = param_beta * t228;
        let t230 = t84 * t84;
        let t231 = t229 * t230;
        let t232 = t109 * t109;
        let t233 = 1.0 / t232;
        let t234 = t233 * t112;
        let t235 = t234 * t116;
        let t236 = t231 * t235;
        let t238 = 1.0 / t119 / t89;
        let t239 = t117 * t238;
        let t240 = t239 * t1;
        let t241 = t122 * t6;
        let t242 = t146 + t171 - t176 - t189;
        let t243 = t242 * t108;
        let t244 = t241 * t243;
        let t245 = t240 * t244;
        let t249 = 1.0 / t21 / t190;
        let t250 = t249 * t117;
        let t251 = t250 * t120;
        let t252 = t251 * t124;
        let t255 = -7.0 / 288.0 * sigma[ip] * t222 * t39 * t98 + t236 * t245 / 3072.0 - 7.0 / 4608.0 * t114 * t252;
        let t256 = param_beta * t255;
        let t258 = t129 * t101;
        let t259 = t133 * t133;
        let t260 = 1.0 / t259;
        let t261 = t84 * t260;
        let t263 = t229 * t230 * t233;
        let t264 = t128 * t242;
        let t265 = t105 * t108;
        let t270 = t102 * t111 * t255 + t263 * t264 * t265;
        let t271 = t261 * t270;
        let t273 = t256 * t135 - t258 * t271;
        let t274 = t89 * t273;
        let t275 = 1.0 / t137;
        let t276 = t274 * t275;
        let t277 = t219 * t276;
        let tvrho0 = -t32 + t57 + t140 + rho[ip] * (t146 + t171 - t176 - t189 + t218 + t277);
        vrho[ip] += tvrho0;
        let t280 = param_alpha * t58;
        let t282 = t63 * t66 * t71;
        let t284 = t76 * t80;
        let t285 = t284 * t82;
        let t286 = t285 * t139;
        let t288 = 3.0 / 32.0 * t280 * t282 * t286;
        let t294 = t102 * t111 * sigma[ip];
        let t297 = t92 * t39 * t95 * t211 / 96.0 + t294 * t125 / 1536.0;
        let t298 = param_beta * t297;
        let t300 = param_beta * param_beta;
        let t301 = t300 * t128;
        let t302 = t301 * t228;
        let t303 = t230 * t260;
        let t305 = t303 * t110 * t297;
        let t307 = t298 * t135 - t302 * t305;
        let t308 = t89 * t307;
        let t309 = t308 * t275;
        let t310 = t219 * t309;
        let tvsigma0 = rho[ip] * (-t288 + t310);
        vsigma[ip] += tvsigma0;
    }
}
