//! MGGA_C_BC95 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_bc95.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_bc95_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_copp: f64,
    param_css: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t4 = 1.0 <= zeta_threshold;
        let t5 = rho[ip] / 2.0 <= dens_threshold || t4;
        let t6 = piecewise3::<f64>(t4, zeta_threshold, 1.0);
        let t7 = M_CBRT3;
        let t8 = 1.0 / M_PI;
        let t9 = pow_1_3::<f64>(t8);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = t10 * t12;
        let t14 = pow_1_3::<f64>(rho[ip]);
        let t15 = 1.0 / t14;
        let t16 = M_CBRT2;
        let t18 = pow_1_3::<f64>(zeta_threshold);
        let t20 = piecewise3::<f64>(t4, 1.0 / t18, 1.0);
        let t22 = t13 * t15 * t16 * t20;
        let t24 = 1.0 + 0.53425e-1 * t22;
        let t25 = f64::sqrt(t22);
        let t28 = pow_3_2::<f64>(t22);
        let t30 = t7 * t7;
        let t31 = t9 * t9;
        let t32 = t30 * t31;
        let t33 = t32 * t11;
        let t34 = t14 * t14;
        let t35 = 1.0 / t34;
        let t36 = t16 * t16;
        let t38 = t20 * t20;
        let t40 = t33 * t35 * t36 * t38;
        let t42 = 0.379785e1 * t25 + 0.8969e0 * t22 + 0.204775e0 * t28 + 0.123235e0 * t40;
        let t45 = 1.0 + 0.16081979498692535067e2 / t42;
        let t46 = f64::ln(t45);
        let t48 = 0.621814e-1 * t24 * t46;
        let t50 = t18 * zeta_threshold;
        let t52 = piecewise3::<f64>(2.0 <= zeta_threshold, t50, 2.0 * t16);
        let t54 = piecewise3::<f64>(0.0 <= zeta_threshold, t50, 0.0);
        let t58 = 1.0 / (2.0 * t16 - 2.0);
        let t59 = (t52 + t54 - 2.0) * t58;
        let t61 = 1.0 + 0.5137e-1 * t22;
        let t66 = 0.705945e1 * t25 + 0.1549425e1 * t22 + 0.420775e0 * t28 + 0.1562925e0 * t40;
        let t69 = 1.0 + 0.32163958997385070134e2 / t66;
        let t70 = f64::ln(t69);
        let t74 = 1.0 + 0.278125e-1 * t22;
        let t79 = 0.51785e1 * t25 + 0.905775e0 * t22 + 0.1100325e0 * t28 + 0.1241775e0 * t40;
        let t82 = 1.0 + 0.29608749977793437516e2 / t79;
        let t83 = f64::ln(t82);
        let t84 = t74 * t83;
        let t93 = piecewise3::<f64>(t5, 0.0, t6 * (-t48 + t59 * (-0.310907e-1 * t61 * t70 + t48 - 0.19751673498613801407e-1 * t84) + 0.19751673498613801407e-1 * t59 * t84) / 2.0);
        let t94 = t93 * tau[ip];
        let t96 = 1.0 / t34 / rho[ip];
        let t97 = t36 * t96;
        let t99 = 1.0 / rho[ip];
        let t101 = 1.0 / tau[ip];
        let t104 = 1.0 - sigma[ip] * t99 * t101 / 8.0;
        let t105 = M_CBRT6;
        let t106 = t104 * t105;
        let t107 = M_PI * M_PI;
        let t108 = pow_1_3::<f64>(t107);
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
        let t127 = 1.0 + 0.53425e-1 * t125;
        let t128 = f64::sqrt(t125);
        let t131 = pow_3_2::<f64>(t125);
        let t134 = t32 * t11 * t35;
        let t136 = 0.379785e1 * t128 + 0.8969e0 * t125 + 0.204775e0 * t131 + 0.123235e0 * t134;
        let t139 = 1.0 + 0.16081979498692535067e2 / t136;
        let t140 = f64::ln(t139);
        let t143 = piecewise3::<f64>(t4, t50, 1.0);
        let t146 = (2.0 * t143 - 2.0) * t58;
        let t148 = 1.0 + 0.278125e-1 * t125;
        let t153 = 0.51785e1 * t128 + 0.905775e0 * t125 + 0.1100325e0 * t131 + 0.1241775e0 * t134;
        let t156 = 1.0 + 0.29608749977793437516e2 / t153;
        let t157 = f64::ln(t156);
        let t162 = -0.621814e-1 * t127 * t140 + 0.19751673498613801407e-1 * t146 * t148 * t157 - 2.0 * t93;
        let t166 = 2.0 * param_copp * sigma[ip] * t115 + 1.0;
        let t167 = 1.0 / t166;
        let t168 = t162 * t167;
        let tzk0 = t123 + t168;
        zk[ip] += tzk0;
        let t170 = 1.0 / t14 / rho[ip];
        let t171 = t170 * t16;
        let t172 = t20 * t46;
        let t175 = 0.11073470983333333333e-2 * t13 * t171 * t172;
        let t176 = t42 * t42;
        let t177 = 1.0 / t176;
        let t178 = t24 * t177;
        let t181 = 1.0 / t25 * t7 * t9;
        let t182 = t12 * t170;
        let t183 = t16 * t20;
        let t184 = t182 * t183;
        let t185 = t181 * t184;
        let t187 = t171 * t20;
        let t188 = t13 * t187;
        let t190 = f64::sqrt(t22);
        let t192 = t190 * t7 * t9;
        let t193 = t192 * t184;
        let t196 = t33 * t97 * t38;
        let t198 = -0.632975e0 * t185 - 0.29896666666666666667e0 * t188 - 0.1023875e0 * t193 - 0.82156666666666666667e-1 * t196;
        let t199 = 1.0 / t45;
        let t200 = t198 * t199;
        let t202 = 1.0 * t178 * t200;
        let t203 = t20 * t70;
        let t207 = t66 * t66;
        let t208 = 1.0 / t207;
        let t209 = t61 * t208;
        let t214 = -0.1176575e1 * t185 - 0.516475e0 * t188 - 0.2103875e0 * t193 - 0.104195e0 * t196;
        let t215 = 1.0 / t69;
        let t216 = t214 * t215;
        let t219 = t20 * t83;
        let t223 = t79 * t79;
        let t224 = 1.0 / t223;
        let t225 = t74 * t224;
        let t230 = -0.86308333333333333334e0 * t185 - 0.301925e0 * t188 - 0.5501625e-1 * t193 - 0.82785e-1 * t196;
        let t231 = 1.0 / t82;
        let t232 = t230 * t231;
        let t237 = t59 * t10;
        let t238 = t183 * t83;
        let t242 = t59 * t74;
        let t244 = t224 * t230 * t231;
        let t250 = piecewise3::<f64>(t5, 0.0, t6 * (t175 + t202 + t59 * (0.53237641966666666666e-3 * t13 * t171 * t203 + 1.0 * t209 * t216 - t175 - t202 + 0.18311447306006545054e-3 * t13 * t171 * t219 + 0.5848223622634646207e0 * t225 * t232) - 0.18311447306006545054e-3 * t237 * t182 * t238 - 0.5848223622634646207e0 * t242 * t244) / 2.0);
        let t251 = t250 * tau[ip];
        let t253 = t251 * t97 * t121;
        let t256 = t94 * t115 * t121;
        let t258 = t93 * t36;
        let t259 = t112 * rho[ip];
        let t261 = 1.0 / t34 / t259;
        let t262 = t258 * t261;
        let t264 = sigma[ip] * t105 * t120;
        let t265 = t262 * t264;
        let t267 = t112 * t112;
        let t268 = t267 * rho[ip];
        let t270 = 1.0 / t14 / t268;
        let t271 = t16 * t270;
        let t272 = t271 * t104;
        let t274 = t105 * t110;
        let t276 = 1.0 / t118 / t117;
        let t277 = t276 * param_css;
        let t279 = t274 * t277 * sigma[ip];
        let t280 = t94 * t272 * t279;
        let t285 = t136 * t136;
        let t286 = 1.0 / t285;
        let t287 = t127 * t286;
        let t289 = 1.0 / t128 * t7;
        let t290 = t9 * t12;
        let t291 = t290 * t170;
        let t292 = t289 * t291;
        let t294 = t10 * t182;
        let t296 = f64::sqrt(t125);
        let t297 = t296 * t7;
        let t298 = t297 * t291;
        let t301 = t32 * t11 * t96;
        let t303 = -0.632975e0 * t292 - 0.29896666666666666667e0 * t294 - 0.1023875e0 * t298 - 0.82156666666666666667e-1 * t301;
        let t304 = 1.0 / t139;
        let t305 = t303 * t304;
        let t308 = t146 * t7;
        let t313 = t146 * t148;
        let t314 = t153 * t153;
        let t315 = 1.0 / t314;
        let t320 = -0.86308333333333333334e0 * t292 - 0.301925e0 * t294 - 0.5501625e-1 * t298 - 0.82785e-1 * t301;
        let t322 = 1.0 / t156;
        let t323 = t315 * t320 * t322;
        let t327 = 0.11073470983333333333e-2 * t10 * t182 * t140 + 1.0 * t287 * t305 - 0.18311447306006545054e-3 * t308 * t290 * t170 * t157 - 0.5848223622634646207e0 * t313 * t323 - 2.0 * t250;
        let t328 = t327 * t167;
        let t329 = t166 * t166;
        let t330 = 1.0 / t329;
        let t331 = t162 * t330;
        let t332 = t331 * param_copp;
        let t333 = sigma[ip] * t36;
        let t334 = t333 * t261;
        let t335 = t332 * t334;
        let tvrho0 = t123 + t168 + rho[ip] * (10.0 / 9.0 * t253 - 50.0 / 27.0 * t256 + 5.0 / 36.0 * t265 + 320.0 / 27.0 * t280 + t328 + 16.0 / 3.0 * t335);
        vrho[ip] += tvrho0;
        let t339 = t258 * t114;
        let t340 = t274 * t119;
        let t342 = 5.0 / 36.0 * t339 * t340;
        let t344 = 1.0 / t14 / t267;
        let t345 = t16 * t344;
        let t347 = t110 * t276;
        let t348 = t347 * param_css;
        let t349 = t106 * t348;
        let t351 = 40.0 / 9.0 * t94 * t345 * t349;
        let t352 = param_copp * t36;
        let t353 = t352 * t114;
        let t355 = 2.0 * t331 * t353;
        let tvsigma0 = rho[ip] * (-t342 - t351 - t355);
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t359 = 10.0 / 9.0 * t258 * t96 * t121;
        let t360 = t93 * t101;
        let t363 = 5.0 / 36.0 * t360 * t115 * t264;
        let tvtau0 = rho[ip] * (t359 + t363);
        vtau[ip] += tvtau0;
    }
}
