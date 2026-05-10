//! GGA_K_VT84F fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 81 shared lines across all orders.
//! Delta: 75 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_vt84f_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_alpha: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (81 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = t24 * t24;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t29 = t25 / t27;
        let t30 = f64::sqrt(sigma[ip]);
        let t31 = M_CBRT2;
        let t32 = t30 * t31;
        let t34 = 1.0 / t21 / rho[ip];
        let t37 = t29 * t32 * t34 / 12.0;
        let t38 = f64::sqrt(f64::EPSILON);
        let t39 = t37 <= t38;
        let t41 = (-param_mu + param_alpha + 5.0 / 3.0) * t24;
        let t42 = t27 * t27;
        let t43 = 1.0 / t42;
        let t44 = t41 * t43;
        let t45 = t31 * t31;
        let t46 = sigma[ip] * t45;
        let t47 = rho[ip] * rho[ip];
        let t49 = 1.0 / t22 / t47;
        let t53 = param_mu * param_alpha;
        let t54 = param_mu * param_mu;
        let t56 = (t53 + t54 - param_alpha) * t25;
        let t58 = 1.0 / t27 / t26;
        let t59 = t56 * t58;
        let t60 = sigma[ip] * sigma[ip];
        let t61 = t60 * t31;
        let t62 = t47 * t47;
        let t63 = t62 * rho[ip];
        let t65 = 1.0 / t21 / t63;
        let t69 = param_alpha * param_alpha;
        let t71 = param_mu * t69 / 2.0;
        let t74 = t69 / 2.0;
        let t76 = t26 * t26;
        let t78 = (-t71 - (t53 + t54) * param_mu - t74) / t76;
        let t79 = t60 * sigma[ip];
        let t80 = t62 * t62;
        let t81 = 1.0 / t80;
        let t85 = t69 * param_alpha;
        let t89 = t54 * param_mu;
        let t93 = (param_mu * t85 / 6.0 - (-param_alpha * t54 - t71 - t89) * param_mu + t74) * t24;
        let t95 = 1.0 / t42 / t76;
        let t96 = t93 * t95;
        let t97 = t60 * t60;
        let t98 = t97 * t45;
        let t99 = t80 * t47;
        let t101 = 1.0 / t22 / t99;
        let t106 = t38 < t37;
        let t107 = piecewise3(t106, t37, t38);
        let t108 = t107 * t107;
        let t109 = param_mu * t108;
        let t110 = param_alpha * t108;
        let t111 = f64::exp(-t110);
        let t112 = 1.0 + t109;
        let t113 = 1.0 / t112;
        let t114 = t111 * t113;
        let t116 = t108 * t108;
        let t118 = f64::exp(-param_alpha * t116);
        let t119 = 1.0 - t118;
        let t120 = 1.0 / t108;
        let t121 = t120 - 1.0;
        let t125 = piecewise3(t39, 1.0 + t44 * t46 * t49 / 24.0 + t59 * t61 * t65 / 288.0 + t78 * t79 * t81 / 576.0 + t96 * t98 * t101 / 13824.0, 1.0 - t109 * t114 + t119 * t121 + 5.0 / 3.0 * t108);
        let t129 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t125);
        let tzk0 = 2.0 * t129;
        zk[ip] += tzk0;
        // --- vxc delta (41 lines) ---
        let t131 = t20 / t21;
        let t135 = t47 * rho[ip];
        let t137 = 1.0 / t22 / t135;
        let t141 = t62 * t47;
        let t143 = 1.0 / t21 / t141;
        let t147 = t80 * rho[ip];
        let t148 = 1.0 / t147;
        let t152 = t80 * t135;
        let t154 = 1.0 / t22 / t152;
        let t159 = param_mu * t107;
        let t161 = 1.0 / t21 / t47;
        let t165 = piecewise3(t106, -t29 * t32 * t161 / 9.0, 0.0);
        let t166 = t114 * t165;
        let t169 = t108 * t107;
        let t170 = param_mu * t169;
        let t171 = t170 * param_alpha;
        let t174 = t54 * t169;
        let t175 = t112 * t112;
        let t176 = 1.0 / t175;
        let t177 = t111 * t176;
        let t178 = t177 * t165;
        let t181 = param_alpha * t169;
        let t182 = t165 * t118;
        let t183 = t182 * t121;
        let t187 = t119 / t169;
        let t190 = t107 * t165;
        let t193 = piecewise3(t39, -t44 * t46 * t137 / 9.0 - t59 * t61 * t143 / 54.0 - t78 * t79 * t148 / 72.0 - t96 * t98 * t154 / 1296.0, -2.0 * t159 * t166 + 2.0 * t171 * t166 + 2.0 * t174 * t178 + 4.0 * t181 * t183 - 2.0 * t187 * t165 + 10.0 / 3.0 * t190);
        let t198 = piecewise3(t2, 0.0, t7 * t131 * t125 / 10.0 + 3.0 / 20.0 * t7 * t23 * t193);
        let tvrho0 = 2.0 * rho[ip] * t198 + 2.0 * t129;
        vrho[ip] += tvrho0;
        let t201 = t43 * t45;
        let t205 = sigma[ip] * t31;
        let t212 = t79 * t45;
        let t218 = 1.0 / t30 * t31;
        let t222 = piecewise3(t106, t29 * t218 * t34 / 24.0, 0.0);
        let t223 = t114 * t222;
        let t228 = t177 * t222;
        let t231 = t222 * t118;
        let t232 = t231 * t121;
        let t240 = piecewise3(t39, t41 * t201 * t49 / 24.0 + t59 * t205 * t65 / 144.0 + t78 * t60 * t81 / 192.0 + t96 * t212 * t101 / 3456.0, -2.0 * t159 * t223 + 2.0 * t171 * t223 + 2.0 * t174 * t228 + 4.0 * t181 * t232 - 2.0 * t187 * t222 + 10.0 / 3.0 * t107 * t222);
        let t244 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t240);
        let tvsigma0 = 2.0 * rho[ip] * t244;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (75 lines) ---
        let t247 = t20 * t34;
        let t255 = 1.0 / t22 / t62;
        let t261 = 1.0 / t21 / t62 / t135;
        let t265 = 1.0 / t99;
        let t269 = t80 * t62;
        let t271 = 1.0 / t22 / t269;
        let t276 = t165 * t165;
        let t277 = param_mu * t276;
        let t280 = t109 * param_alpha;
        let t281 = t276 * t111;
        let t282 = t281 * t113;
        let t285 = t54 * t108;
        let t286 = t177 * t276;
        let t290 = 1.0 / t21 / t135;
        let t294 = piecewise3(t106, 7.0 / 27.0 * t29 * t32 * t290, 0.0);
        let t295 = t114 * t294;
        let t300 = param_mu * t116;
        let t301 = t300 * t69;
        let t304 = t54 * t116;
        let t305 = t304 * param_alpha;
        let t308 = t89 * t116;
        let t310 = 1.0 / t175 / t112;
        let t311 = t111 * t310;
        let t319 = t276 * t118 * t121;
        let t322 = t294 * t118;
        let t323 = t322 * t121;
        let t326 = t116 * t108;
        let t327 = t69 * t326;
        let t330 = param_alpha * t276;
        let t334 = t119 / t116;
        let t340 = t107 * t294;
        let t342 = -2.0 * t277 * t114 + 10.0 * t280 * t282 + 10.0 * t285 * t286 - 2.0 * t159 * t295 + 2.0 * t171 * t295 - 4.0 * t301 * t282 - 8.0 * t305 * t286 - 8.0 * t308 * t311 * t276 + 2.0 * t174 * t177 * t294 + 12.0 * t110 * t319 + 4.0 * t181 * t323 - 16.0 * t327 * t319 - 16.0 * t330 * t118 + 6.0 * t334 * t276 - 2.0 * t187 * t294 + 10.0 / 3.0 * t276 + 10.0 / 3.0 * t340;
        let t343 = piecewise3(t39, 11.0 / 27.0 * t44 * t46 * t255 + 19.0 / 162.0 * t59 * t61 * t261 + t78 * t79 * t265 / 8.0 + 35.0 / 3888.0 * t96 * t98 * t271, t342);
        let t348 = piecewise3(t2, 0.0, -t7 * t247 * t125 / 30.0 + t7 * t131 * t193 / 5.0 + 3.0 / 20.0 * t7 * t23 * t343);
        let tv2rho20 = 2.0 * rho[ip] * t348 + 4.0 * t198;
        v2rho2[ip] += tv2rho20;
        let t367 = param_mu * t165;
        let t370 = t165 * t111;
        let t371 = t113 * t222;
        let t372 = t370 * t371;
        let t375 = t285 * t111;
        let t376 = t176 * t222;
        let t383 = piecewise3(t106, -t29 * t218 * t161 / 18.0, 0.0);
        let t384 = t114 * t383;
        let t391 = t222 * t111;
        let t392 = t176 * t165;
        let t393 = t391 * t392;
        let t396 = t308 * t111;
        let t397 = t310 * t222;
        let t404 = t110 * t222;
        let t407 = t383 * t118;
        let t408 = t407 * t121;
        let t411 = t327 * t222;
        let t414 = param_alpha * t222;
        let t417 = t165 * t222;
        let t425 = -2.0 * t367 * t223 + 10.0 * t280 * t372 + 10.0 * t375 * t376 * t165 - 2.0 * t159 * t384 + 2.0 * t171 * t384 - 4.0 * t301 * t372 - 8.0 * t305 * t393 - 8.0 * t396 * t397 * t165 + 2.0 * t174 * t177 * t383 + 12.0 * t404 * t183 + 4.0 * t181 * t408 - 16.0 * t411 * t183 - 16.0 * t414 * t182 + 6.0 * t334 * t417 - 2.0 * t187 * t383 + 10.0 / 3.0 * t417 + 10.0 / 3.0 * t107 * t383;
        let t426 = piecewise3(t39, -t41 * t201 * t137 / 9.0 - t59 * t205 * t143 / 27.0 - t78 * t60 * t148 / 24.0 - t96 * t212 * t154 / 324.0, t425);
        let t431 = piecewise3(t2, 0.0, t7 * t131 * t240 / 10.0 + 3.0 / 20.0 * t7 * t23 * t426);
        let tv2rhosigma0 = 2.0 * rho[ip] * t431 + 2.0 * t244;
        v2rhosigma[ip] += tv2rhosigma0;
        let t434 = t58 * t31;
        let t441 = t60 * t45;
        let t446 = t222 * t222;
        let t447 = param_mu * t446;
        let t450 = t446 * t111;
        let t451 = t450 * t113;
        let t454 = t177 * t446;
        let t459 = 1.0 / t30 / sigma[ip] * t31;
        let t463 = piecewise3(t106, -t29 * t459 * t34 / 48.0, 0.0);
        let t464 = t114 * t463;
        let t479 = t446 * t118;
        let t480 = t479 * t121;
        let t484 = t463 * t118 * t121;
        let t499 = -2.0 * t447 * t114 + 10.0 * t280 * t451 + 10.0 * t285 * t454 - 2.0 * t159 * t464 + 2.0 * t171 * t464 - 4.0 * t301 * t451 - 8.0 * t305 * t454 - 8.0 * t308 * t311 * t446 + 2.0 * t174 * t177 * t463 + 12.0 * t110 * t480 + 4.0 * t181 * t484 - 16.0 * t327 * t480 - 16.0 * param_alpha * t446 * t118 + 6.0 * t334 * t446 - 2.0 * t187 * t463 + 10.0 / 3.0 * t446 + 10.0 / 3.0 * t107 * t463;
        let t500 = piecewise3(t39, t56 * t434 * t65 / 144.0 + t78 * sigma[ip] * t81 / 96.0 + t96 * t441 * t101 / 1152.0, t499);
        let t504 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t500);
        let tv2sigma20 = 2.0 * rho[ip] * t504;
        v2sigma2[ip] += tv2sigma20;
    }
}
