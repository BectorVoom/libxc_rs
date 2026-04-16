//! MGGA_X_TB09 fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 63 shared lines across all orders.
//! Delta: 172 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_tb09_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    param_alpha: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (63 lines) ---
        let t2 = M_CBRTPI;
        let t3 = param_c * t2;
        let t4 = pow_1_3(rho0);
        let t5 = t4 * t4;
        let t7 = 1.0 / t5 / rho0;
        let t10 = tau0 * t7;
        let t12 = rho0 * rho0;
        let t14 = 1.0 / t5 / t12;
        let t17 = lapl0 * t7 / 6.0 - 0.53333333333333333333e0 * t10 + 0.66666666666666666667e-1 * sigma0 * t14;
        let t18 = f64::abs(t17);
        let t19 = t18 < 0.5e-12;
        let t20 = 0.0 < t17;
        let t21 = piecewise3(t20, 0.5e-12, -0.5e-12);
        let t22 = piecewise3(t19, t21, t17);
        let t23 = xc_mgga_x_br89_get_x(t22);
        let t25 = f64::exp(t23 / 3.0);
        let t26 = f64::exp(-t23);
        let t28 = 1.0 + t23 / 2.0;
        let t29 = t26 * t28;
        let t30 = 1.0 - t29;
        let t31 = t25 * t30;
        let t32 = 1.0 / t23;
        let t33 = t31 * t32;
        let t38 = f64::sqrt(15.0);
        let t39 = (3.0 * param_c - 2.0) * t38;
        let t40 = 1.0 / M_PI;
        let t41 = M_SQRT2;
        let t42 = t40 * t41;
        let t43 = param_alpha * sigma0;
        let t46 = t10 - t43 * t14 / 8.0;
        let t47 = 0.1e-9 < t46;
        let t48 = piecewise3(t47, t46, 0.1e-9);
        let t49 = f64::sqrt(t48);
        let t53 = -2.0 * t3 * t33 + t39 * t42 * t49 / 6.0;
        let tvrho0 = t53 * t4;
        vrho[ip * 2] += tvrho0;
        let t54 = pow_1_3(rho1);
        let t55 = t54 * t54;
        let t57 = 1.0 / t55 / rho1;
        let t60 = tau1 * t57;
        let t62 = rho1 * rho1;
        let t64 = 1.0 / t55 / t62;
        let t67 = lapl1 * t57 / 6.0 - 0.53333333333333333333e0 * t60 + 0.66666666666666666667e-1 * sigma2 * t64;
        let t68 = f64::abs(t67);
        let t69 = t68 < 0.5e-12;
        let t70 = 0.0 < t67;
        let t71 = piecewise3(t70, 0.5e-12, -0.5e-12);
        let t72 = piecewise3(t69, t71, t67);
        let t73 = xc_mgga_x_br89_get_x(t72);
        let t75 = f64::exp(t73 / 3.0);
        let t76 = f64::exp(-t73);
        let t78 = 1.0 + t73 / 2.0;
        let t79 = t76 * t78;
        let t80 = 1.0 - t79;
        let t81 = t75 * t80;
        let t82 = 1.0 / t73;
        let t83 = t81 * t82;
        let t86 = param_alpha * sigma2;
        let t89 = t60 - t86 * t64 / 8.0;
        let t90 = 0.1e-9 < t89;
        let t91 = piecewise3(t90, t89, 0.1e-9);
        let t92 = f64::sqrt(t91);
        let t96 = -2.0 * t3 * t83 + t39 * t42 * t92 / 6.0;
        let tvrho1 = t96 * t54;
        vrho[ip * 2 + 1] += tvrho1;
        // --- fxc delta (this level) (172 lines) ---
        let t97 = param_c * M_PI;
        let t98 = piecewise3(t20, 0.0, 0.0);
        let t101 = tau0 * t14;
        let t105 = 1.0 / t5 / t12 / rho0;
        let t109 = piecewise3(t19, t98, -5.0 / 18.0 * lapl0 * t14 + 0.88888888888888888889e0 * t101 - 0.17777777777777777778e0 * sigma0 * t105);
        let t110 = t22 * t22;
        let t111 = 1.0 / t110;
        let t112 = t109 * t111;
        let t114 = f64::exp(-2.0 / 3.0 * t23);
        let t115 = 1.0 / t114;
        let t116 = t112 * t115;
        let t117 = t97 * t116;
        let t118 = t23 * t23;
        let t120 = t118 - 2.0 * t23 + 3.0;
        let t121 = 1.0 / t120;
        let t122 = t23 - 2.0;
        let t123 = t122 * t122;
        let t124 = t121 * t123;
        let t125 = t124 * t33;
        let t128 = t2 * t2;
        let t129 = t128 * t109;
        let t130 = t111 * t115;
        let t131 = t129 * t130;
        let t132 = t124 * t29;
        let t134 = t129 * t111;
        let t135 = t115 * t121;
        let t136 = t123 * t26;
        let t137 = t135 * t136;
        let t140 = t131 * t132 - t134 * t137 / 2.0;
        let t141 = t25 * t140;
        let t142 = t141 * t32;
        let t145 = 1.0 / t118;
        let t146 = t31 * t145;
        let t147 = t97 * t146;
        let t148 = t135 * t123;
        let t149 = t112 * t148;
        let t152 = t39 * t40;
        let t154 = t41 / t49;
        let t159 = piecewise3(t47, -5.0 / 3.0 * t101 + t43 * t105 / 3.0, 0.0);
        let t163 = -2.0 / 3.0 * t117 * t125 - 2.0 * t3 * t142 + 2.0 * t147 * t149 + t152 * t154 * t159 / 12.0;
        let t165 = 1.0 / t5;
        let tv2rho20 = t163 * t4 + t53 * t165 / 3.0;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = 0.0;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t168 = piecewise3(t70, 0.0, 0.0);
        let t171 = tau1 * t64;
        let t175 = 1.0 / t55 / t62 / rho1;
        let t179 = piecewise3(t69, t168, -5.0 / 18.0 * lapl1 * t64 + 0.88888888888888888889e0 * t171 - 0.17777777777777777778e0 * sigma2 * t175);
        let t180 = t72 * t72;
        let t181 = 1.0 / t180;
        let t182 = t179 * t181;
        let t184 = f64::exp(-2.0 / 3.0 * t73);
        let t185 = 1.0 / t184;
        let t186 = t182 * t185;
        let t187 = t97 * t186;
        let t188 = t73 * t73;
        let t190 = t188 - 2.0 * t73 + 3.0;
        let t191 = 1.0 / t190;
        let t192 = t73 - 2.0;
        let t193 = t192 * t192;
        let t194 = t191 * t193;
        let t195 = t194 * t83;
        let t198 = t128 * t179;
        let t199 = t181 * t185;
        let t200 = t198 * t199;
        let t201 = t194 * t79;
        let t203 = t198 * t181;
        let t204 = t185 * t191;
        let t205 = t193 * t76;
        let t206 = t204 * t205;
        let t209 = t200 * t201 - t203 * t206 / 2.0;
        let t210 = t75 * t209;
        let t211 = t210 * t82;
        let t214 = 1.0 / t188;
        let t215 = t81 * t214;
        let t216 = t97 * t215;
        let t217 = t204 * t193;
        let t218 = t182 * t217;
        let t222 = t41 / t92;
        let t227 = piecewise3(t90, -5.0 / 3.0 * t171 + t86 * t175 / 3.0, 0.0);
        let t231 = -2.0 / 3.0 * t187 * t195 - 2.0 * t3 * t211 + 2.0 * t216 * t218 + t152 * t222 * t227 / 12.0;
        let t233 = 1.0 / t55;
        let tv2rho22 = t231 * t54 + t96 * t233 / 3.0;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t237 = piecewise3(t19, t98, t7 / 6.0);
        let t238 = t237 * t111;
        let t239 = t238 * t115;
        let t240 = t97 * t239;
        let t243 = t128 * t237;
        let t244 = t243 * t130;
        let t246 = t243 * t111;
        let t249 = t244 * t132 - t246 * t137 / 2.0;
        let t250 = t25 * t249;
        let t251 = t250 * t32;
        let t254 = t238 * t148;
        let t257 = -2.0 / 3.0 * t240 * t125 - 2.0 * t3 * t251 + 2.0 * t147 * t254;
        let tv2rholapl0 = t257 * t4;
        v2rholapl[ip * 4] += tv2rholapl0;
        let tv2rholapl1 = 0.0;
        v2rholapl[ip * 4 + 1] += tv2rholapl1;
        let tv2rholapl2 = 0.0;
        v2rholapl[ip * 4 + 2] += tv2rholapl2;
        let t259 = piecewise3(t69, t168, t57 / 6.0);
        let t260 = t259 * t181;
        let t261 = t260 * t185;
        let t262 = t97 * t261;
        let t265 = t128 * t259;
        let t266 = t265 * t199;
        let t268 = t265 * t181;
        let t271 = t266 * t201 - t268 * t206 / 2.0;
        let t272 = t75 * t271;
        let t273 = t272 * t82;
        let t276 = t260 * t217;
        let t279 = -2.0 / 3.0 * t262 * t195 - 2.0 * t3 * t273 + 2.0 * t216 * t276;
        let tv2rholapl3 = t279 * t54;
        v2rholapl[ip * 4 + 3] += tv2rholapl3;
        let t281 = piecewise3(t19, t98, 0.66666666666666666667e-1 * t14);
        let t282 = t281 * t111;
        let t283 = t282 * t115;
        let t284 = t97 * t283;
        let t287 = t128 * t281;
        let t288 = t287 * t130;
        let t290 = t287 * t111;
        let t293 = t288 * t132 - t290 * t137 / 2.0;
        let t294 = t25 * t293;
        let t295 = t294 * t32;
        let t298 = t282 * t148;
        let t303 = piecewise3(t47, -param_alpha * t14 / 8.0, 0.0);
        let t307 = -2.0 / 3.0 * t284 * t125 - 2.0 * t3 * t295 + 2.0 * t147 * t298 + t152 * t154 * t303 / 12.0;
        let tv2rhosigma0 = t307 * t4;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = 0.0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = 0.0;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t309 = piecewise3(t69, t168, 0.66666666666666666667e-1 * t64);
        let t310 = t309 * t181;
        let t311 = t310 * t185;
        let t312 = t97 * t311;
        let t315 = t128 * t309;
        let t316 = t315 * t199;
        let t318 = t315 * t181;
        let t321 = t316 * t201 - t318 * t206 / 2.0;
        let t322 = t75 * t321;
        let t323 = t322 * t82;
        let t326 = t310 * t217;
        let t331 = piecewise3(t90, -param_alpha * t64 / 8.0, 0.0);
        let t335 = -2.0 / 3.0 * t312 * t195 - 2.0 * t3 * t323 + 2.0 * t216 * t326 + t152 * t222 * t331 / 12.0;
        let tv2rhosigma5 = t335 * t54;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t337 = piecewise3(t19, t98, -0.53333333333333333333e0 * t7);
        let t338 = t337 * t111;
        let t339 = t338 * t115;
        let t340 = t97 * t339;
        let t343 = t128 * t337;
        let t344 = t343 * t130;
        let t346 = t343 * t111;
        let t349 = t344 * t132 - t346 * t137 / 2.0;
        let t350 = t25 * t349;
        let t351 = t350 * t32;
        let t354 = t338 * t148;
        let t357 = piecewise3(t47, t7, 0.0);
        let t361 = -2.0 / 3.0 * t340 * t125 - 2.0 * t3 * t351 + 2.0 * t147 * t354 + t152 * t154 * t357 / 12.0;
        let tv2rhotau0 = t361 * t4;
        v2rhotau[ip * 4] += tv2rhotau0;
        let tv2rhotau1 = 0.0;
        v2rhotau[ip * 4 + 1] += tv2rhotau1;
        let tv2rhotau2 = 0.0;
        v2rhotau[ip * 4 + 2] += tv2rhotau2;
        let t363 = piecewise3(t69, t168, -0.53333333333333333333e0 * t57);
        let t364 = t363 * t181;
        let t365 = t364 * t185;
        let t366 = t97 * t365;
        let t369 = t128 * t363;
        let t370 = t369 * t199;
        let t372 = t369 * t181;
        let t375 = t370 * t201 - t372 * t206 / 2.0;
        let t376 = t75 * t375;
        let t377 = t376 * t82;
        let t380 = t364 * t217;
        let t383 = piecewise3(t90, t57, 0.0);
        let t387 = -2.0 / 3.0 * t366 * t195 - 2.0 * t3 * t377 + 2.0 * t216 * t380 + t152 * t222 * t383 / 12.0;
        let tv2rhotau3 = t387 * t54;
        v2rhotau[ip * 4 + 3] += tv2rhotau3;
    }
}
