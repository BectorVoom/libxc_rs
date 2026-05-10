//! GGA_C_AM05 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 82 shared lines across all orders.
//! Delta: 100 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_am05_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alpha: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (82 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t11 = t4 * t6 / t8;
        let t13 = 1.0 + 0.53425e-1 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t1 * t1;
        let t20 = t3 * t3;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t5 / t22;
        let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
        let t30 = 1.0 + 0.16081979498692535067e2 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.621814e-1 * t13 * t31;
        let t34 = rho0 - rho1;
        let t35 = t34 * t34;
        let t36 = t35 * t35;
        let t37 = t7 * t7;
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t36 * t39;
        let t41 = 1.0 / t7;
        let t42 = t34 * t41;
        let t43 = 1.0 + t42;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(zeta_threshold);
        let t46 = t45 * zeta_threshold;
        let t47 = pow_1_3(t43);
        let t49 = piecewise3(t44, t46, t47 * t43);
        let t50 = 1.0 - t42;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t54 = piecewise3(t51, t46, t52 * t50);
        let t55 = t49 + t54 - 2.0;
        let t56 = M_CBRT2;
        let t59 = 1.0 / (2.0 * t56 - 2.0);
        let t60 = t55 * t59;
        let t62 = 1.0 + 0.5137e-1 * t11;
        let t67 = 0.705945e1 * t14 + 0.1549425e1 * t11 + 0.420775e0 * t17 + 0.1562925e0 * t25;
        let t70 = 1.0 + 0.32163958997385070134e2 / t67;
        let t71 = f64::ln(t70);
        let t75 = 1.0 + 0.278125e-1 * t11;
        let t80 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
        let t83 = 1.0 + 0.29608749977793437516e2 / t80;
        let t84 = f64::ln(t83);
        let t85 = t75 * t84;
        let t87 = -0.310907e-1 * t62 * t71 + t33 - 0.19751673498613801407e-1 * t85;
        let t88 = t60 * t87;
        let t92 = -t33 + t40 * t88 + 0.19751673498613801407e-1 * t60 * t85;
        let t93 = piecewise3(t44, zeta_threshold, t43);
        let t94 = M_CBRT6;
        let t95 = param_alpha * t94;
        let t96 = M_PI * M_PI;
        let t97 = pow_1_3(t96);
        let t98 = t97 * t97;
        let t99 = 1.0 / t98;
        let t100 = t99 * sigma0;
        let t101 = rho0 * rho0;
        let t102 = pow_1_3(rho0);
        let t103 = t102 * t102;
        let t105 = 1.0 / t103 / t101;
        let t109 = 1.0 + t95 * t100 * t105 / 24.0;
        let t110 = 1.0 / t109;
        let t113 = t110 + param_gamma * (1.0 - t110);
        let t115 = piecewise3(t51, zeta_threshold, t50);
        let t116 = t99 * sigma2;
        let t117 = rho1 * rho1;
        let t118 = pow_1_3(rho1);
        let t119 = t118 * t118;
        let t121 = 1.0 / t119 / t117;
        let t125 = 1.0 + t95 * t116 * t121 / 24.0;
        let t126 = 1.0 / t125;
        let t129 = t126 + param_gamma * (1.0 - t126);
        let t132 = t93 * t113 / 2.0 + t115 * t129 / 2.0;
        let tzk0 = t92 * t132;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (100 lines) ---
        let t134 = 1.0 / t8 / t7;
        let t135 = t6 * t134;
        let t138 = 0.11073470983333333333e-2 * t4 * t135 * t31;
        let t139 = t27 * t27;
        let t140 = 1.0 / t139;
        let t141 = t13 * t140;
        let t143 = 1.0 / t14 * t1;
        let t144 = t3 * t6;
        let t145 = t144 * t134;
        let t146 = t143 * t145;
        let t148 = t4 * t135;
        let t150 = f64::sqrt(t11);
        let t151 = t150 * t1;
        let t152 = t151 * t145;
        let t157 = t21 * t5 / t22 / t7;
        let t159 = -0.632975e0 * t146 - 0.29896666666666666667e0 * t148 - 0.1023875e0 * t152 - 0.82156666666666666667e-1 * t157;
        let t160 = 1.0 / t30;
        let t161 = t159 * t160;
        let t163 = 1.0 * t141 * t161;
        let t164 = t35 * t34;
        let t165 = t164 * t39;
        let t167 = 4.0 * t165 * t88;
        let t168 = t38 * t7;
        let t169 = 1.0 / t168;
        let t170 = t36 * t169;
        let t172 = 4.0 * t170 * t88;
        let t173 = 1.0 / t37;
        let t174 = t34 * t173;
        let t175 = t41 - t174;
        let t178 = piecewise3(t44, 0.0, 4.0 / 3.0 * t47 * t175);
        let t179 = -t175;
        let t182 = piecewise3(t51, 0.0, 4.0 / 3.0 * t52 * t179);
        let t184 = (t178 + t182) * t59;
        let t185 = t184 * t87;
        let t190 = t67 * t67;
        let t191 = 1.0 / t190;
        let t192 = t62 * t191;
        let t197 = -0.1176575e1 * t146 - 0.516475e0 * t148 - 0.2103875e0 * t152 - 0.104195e0 * t157;
        let t198 = 1.0 / t70;
        let t199 = t197 * t198;
        let t205 = t80 * t80;
        let t206 = 1.0 / t205;
        let t207 = t75 * t206;
        let t212 = -0.86308333333333333334e0 * t146 - 0.301925e0 * t148 - 0.5501625e-1 * t152 - 0.82785e-1 * t157;
        let t213 = 1.0 / t83;
        let t214 = t212 * t213;
        let t217 = 0.53237641966666666666e-3 * t4 * t135 * t71 + 1.0 * t192 * t199 - t138 - t163 + 0.18311447306006545054e-3 * t4 * t135 * t84 + 0.5848223622634646207e0 * t207 * t214;
        let t218 = t60 * t217;
        let t219 = t40 * t218;
        let t222 = t60 * t1;
        let t224 = t144 * t134 * t84;
        let t226 = 0.18311447306006545054e-3 * t222 * t224;
        let t227 = t60 * t75;
        let t229 = t206 * t212 * t213;
        let t231 = 0.5848223622634646207e0 * t227 * t229;
        let t232 = t138 + t163 + t167 - t172 + t40 * t185 + t219 + 0.19751673498613801407e-1 * t184 * t85 - t226 - t231;
        let t233 = t7 * t232;
        let t235 = t7 * t92;
        let t236 = piecewise3(t44, 0.0, t175);
        let t238 = t109 * t109;
        let t239 = 1.0 / t238;
        let t240 = t239 * param_alpha;
        let t241 = t240 * t94;
        let t242 = t101 * rho0;
        let t244 = 1.0 / t103 / t242;
        let t248 = param_gamma * t239 * param_alpha;
        let t249 = t94 * t99;
        let t254 = -t248 * t249 * sigma0 * t244 / 9.0 + t241 * t100 * t244 / 9.0;
        let t256 = piecewise3(t51, 0.0, t179);
        let t259 = t236 * t113 / 2.0 + t256 * t129 / 2.0 + t93 * t254 / 2.0;
        let tvrho0 = t132 * t233 + t235 * t259 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t261 = -t41 - t174;
        let t264 = piecewise3(t44, 0.0, 4.0 / 3.0 * t47 * t261);
        let t265 = -t261;
        let t268 = piecewise3(t51, 0.0, 4.0 / 3.0 * t52 * t265);
        let t270 = (t264 + t268) * t59;
        let t271 = t270 * t87;
        let t275 = t138 + t163 - t167 - t172 + t40 * t271 + t219 + 0.19751673498613801407e-1 * t270 * t85 - t226 - t231;
        let t276 = t7 * t275;
        let t278 = piecewise3(t44, 0.0, t261);
        let t280 = piecewise3(t51, 0.0, t265);
        let t282 = t125 * t125;
        let t283 = 1.0 / t282;
        let t284 = t283 * param_alpha;
        let t285 = t284 * t94;
        let t286 = t117 * rho1;
        let t288 = 1.0 / t119 / t286;
        let t292 = param_gamma * t283 * param_alpha;
        let t297 = -t292 * t249 * sigma2 * t288 / 9.0 + t285 * t116 * t288 / 9.0;
        let t300 = t278 * t113 / 2.0 + t115 * t297 / 2.0 + t280 * t129 / 2.0;
        let tvrho1 = t132 * t276 + t235 * t300 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t302 = t249 * t105;
        let t306 = -t240 * t302 / 24.0 + t248 * t302 / 24.0;
        let t307 = t93 * t306;
        let tvsigma0 = t235 * t307 / 2.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t309 = t249 * t121;
        let t313 = -t284 * t309 / 24.0 + t292 * t309 / 24.0;
        let t314 = t115 * t313;
        let tvsigma2 = t235 * t314 / 2.0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
