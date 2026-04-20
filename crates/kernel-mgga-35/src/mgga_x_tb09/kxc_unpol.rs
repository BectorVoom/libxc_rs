//! MGGA_X_TB09 kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 41 shared lines across all orders.
//! Delta: 362 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::br89::xc_mgga_x_br89_get_x;

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_tb09_kxc_unpol(
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
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    param_alpha: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        // --- shared preamble (41 lines) ---
        let t2 = M_CBRTPI;
        let t3 = param_c * t2;
        let t4 = M_CBRT2;
        let t5 = t4 * t4;
        let t6 = pow_1_3(rho[ip]);
        let t7 = t6 * t6;
        let t9 = 1.0 / t7 / rho[ip];
        let t14 = rho[ip] * rho[ip];
        let t16 = 1.0 / t7 / t14;
        let t20 = f64::abs(lapl[ip] * t9 / 6.0 - 0.53333333333333333332e0 * tau[ip] * t9 + 0.66666666666666666668e-1 * sigma[ip] * t16);
        let t22 = t5 * t20 < 0.5e-12;
        let t23 = lapl[ip] * t5;
        let t26 = tau[ip] * t5;
        let t27 = t26 * t9;
        let t29 = sigma[ip] * t5;
        let t32 = t23 * t9 / 6.0 - 0.53333333333333333333e0 * t27 + 0.66666666666666666667e-1 * t29 * t16;
        let t33 = 0.0 < t32;
        let t34 = piecewise3(t33, 0.5e-12, -0.5e-12);
        let t35 = piecewise3(t22, t34, t32);
        let t36 = xc_mgga_x_br89_get_x(t35);
        let t38 = f64::exp(t36 / 3.0);
        let t39 = f64::exp(-t36);
        let t41 = 1.0 + t36 / 2.0;
        let t42 = t39 * t41;
        let t43 = 1.0 - t42;
        let t44 = t38 * t43;
        let t45 = 1.0 / t36;
        let t46 = t44 * t45;
        let t51 = f64::sqrt(15.0);
        let t52 = (3.0 * param_c - 2.0) * t51;
        let t53 = 1.0 / M_PI;
        let t54 = M_SQRT2;
        let t55 = t53 * t54;
        let t56 = param_alpha * sigma[ip];
        let t57 = t5 * t16;
        let t60 = t27 - t56 * t57 / 8.0;
        let t61 = 0.1e-9 < t60;
        let t62 = piecewise3(t61, t60, 0.1e-9);
        let t63 = f64::sqrt(t62);
        let t68 = (-2.0 * t3 * t46 + t52 * t55 * t63 / 6.0) * t5;
        let tvrho0 = t68 * t6 / 2.0;
        vrho[ip] += tvrho0;
        // --- fxc delta (86 lines) ---
        let t70 = param_c * M_PI;
        let t71 = piecewise3(t33, 0.0, 0.0);
        let t74 = t26 * t16;
        let t78 = 1.0 / t7 / t14 / rho[ip];
        let t82 = piecewise3(t22, t71, -5.0 / 18.0 * t23 * t16 + 0.88888888888888888889e0 * t74 - 0.17777777777777777778e0 * t29 * t78);
        let t83 = t35 * t35;
        let t84 = 1.0 / t83;
        let t85 = t82 * t84;
        let t87 = f64::exp(-2.0 / 3.0 * t36);
        let t88 = 1.0 / t87;
        let t89 = t85 * t88;
        let t90 = t70 * t89;
        let t91 = t36 * t36;
        let t93 = t91 - 2.0 * t36 + 3.0;
        let t94 = 1.0 / t93;
        let t95 = t36 - 2.0;
        let t96 = t95 * t95;
        let t97 = t94 * t96;
        let t98 = t97 * t46;
        let t101 = t2 * t2;
        let t102 = t101 * t82;
        let t103 = t84 * t88;
        let t104 = t102 * t103;
        let t105 = t97 * t42;
        let t107 = t102 * t84;
        let t108 = t88 * t94;
        let t109 = t96 * t39;
        let t110 = t108 * t109;
        let t113 = t104 * t105 - t107 * t110 / 2.0;
        let t114 = t38 * t113;
        let t115 = t114 * t45;
        let t118 = 1.0 / t91;
        let t119 = t44 * t118;
        let t120 = t70 * t119;
        let t121 = t108 * t96;
        let t122 = t85 * t121;
        let t125 = t52 * t53;
        let t127 = t54 / t63;
        let t129 = t5 * t78;
        let t133 = piecewise3(t61, -5.0 / 3.0 * t74 + t56 * t129 / 3.0, 0.0);
        let t138 = (-2.0 / 3.0 * t90 * t98 - 2.0 * t3 * t115 + 2.0 * t120 * t122 + t125 * t127 * t133 / 12.0) * t5;
        let t141 = 1.0 / t7;
        let tv2rho20 = t138 * t6 / 2.0 + t68 * t141 / 6.0;
        v2rho2[ip] += tv2rho20;
        let t145 = piecewise3(t22, t71, 0.66666666666666666667e-1 * t57);
        let t146 = t145 * t84;
        let t147 = t146 * t88;
        let t148 = t70 * t147;
        let t151 = t101 * t145;
        let t152 = t151 * t103;
        let t154 = t151 * t84;
        let t157 = t152 * t105 - t154 * t110 / 2.0;
        let t158 = t38 * t157;
        let t159 = t158 * t45;
        let t162 = t146 * t121;
        let t165 = param_alpha * t5;
        let t168 = piecewise3(t61, -t165 * t16 / 8.0, 0.0);
        let t173 = (-2.0 / 3.0 * t148 * t98 - 2.0 * t3 * t159 + 2.0 * t120 * t162 + t125 * t127 * t168 / 12.0) * t5;
        let tv2rhosigma0 = t173 * t6 / 2.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t175 = t5 * t9;
        let t177 = piecewise3(t22, t71, t175 / 6.0);
        let t178 = t177 * t84;
        let t179 = t178 * t88;
        let t180 = t70 * t179;
        let t183 = t101 * t177;
        let t184 = t183 * t103;
        let t186 = t183 * t84;
        let t189 = t184 * t105 - t186 * t110 / 2.0;
        let t190 = t38 * t189;
        let t191 = t190 * t45;
        let t194 = t178 * t121;
        let t198 = (-2.0 / 3.0 * t180 * t98 - 2.0 * t3 * t191 + 2.0 * t120 * t194) * t5;
        let tv2rholapl0 = t198 * t6 / 2.0;
        v2rholapl[ip] += tv2rholapl0;
        let t201 = piecewise3(t22, t71, -0.53333333333333333333e0 * t175);
        let t202 = t201 * t84;
        let t203 = t202 * t88;
        let t204 = t70 * t203;
        let t207 = t101 * t201;
        let t208 = t207 * t103;
        let t210 = t207 * t84;
        let t213 = t208 * t105 - t210 * t110 / 2.0;
        let t214 = t38 * t213;
        let t215 = t214 * t45;
        let t218 = t202 * t121;
        let t221 = piecewise3(t61, t175, 0.0);
        let t226 = (-2.0 / 3.0 * t204 * t98 - 2.0 * t3 * t215 + 2.0 * t120 * t218 + t125 * t127 * t221 / 12.0) * t5;
        let tv2rhotau0 = t226 * t6 / 2.0;
        v2rhotau[ip] += tv2rhotau0;
        // --- kxc delta (this level) (362 lines) ---
        let t230 = t26 * t78;
        let t232 = t14 * t14;
        let t234 = 1.0 / t7 / t232;
        let t238 = piecewise3(t22, t71, 20.0 / 27.0 * t23 * t78 - 0.23703703703703703704e1 * t230 + 0.65185185185185185186e0 * t29 * t234);
        let t239 = t238 * t84;
        let t240 = t239 * t88;
        let t241 = t70 * t240;
        let t244 = t82 * t82;
        let t246 = 1.0 / t83 / t35;
        let t247 = t244 * t246;
        let t248 = t247 * t88;
        let t249 = t70 * t248;
        let t253 = param_c * t101 * M_PI;
        let t254 = t83 * t83;
        let t255 = 1.0 / t254;
        let t256 = t244 * t255;
        let t257 = t87 * t87;
        let t258 = 1.0 / t257;
        let t259 = t256 * t258;
        let t260 = t253 * t259;
        let t261 = t93 * t93;
        let t262 = 1.0 / t261;
        let t263 = t96 * t96;
        let t264 = t262 * t263;
        let t265 = t264 * t46;
        let t268 = t262 * t96;
        let t269 = t268 * t38;
        let t270 = t43 * t45;
        let t271 = t36 * t101;
        let t273 = t103 * t97;
        let t277 = 2.0 * t271 * t82 * t273 - 2.0 * t107 * t121;
        let t278 = t270 * t277;
        let t279 = t269 * t278;
        let t282 = t96 * t95;
        let t283 = t262 * t282;
        let t284 = t283 * t46;
        let t287 = t97 * t115;
        let t290 = t264 * t119;
        let t293 = t101 * t238;
        let t294 = t293 * t103;
        let t296 = t101 * t244;
        let t297 = t246 * t88;
        let t298 = t296 * t297;
        let t301 = t2 * M_PI;
        let t302 = t301 * t244;
        let t303 = t255 * t258;
        let t304 = t302 * t303;
        let t305 = t264 * t42;
        let t308 = t42 * t277;
        let t309 = t268 * t308;
        let t311 = t283 * t42;
        let t314 = t302 * t255;
        let t315 = t258 * t262;
        let t316 = t263 * t39;
        let t317 = t315 * t316;
        let t320 = t293 * t84;
        let t323 = t296 * t246;
        let t325 = t39 * t277;
        let t326 = t268 * t325;
        let t329 = t282 * t39;
        let t330 = t315 * t329;
        let t333 = t38 * (t294 * t105 - 2.0 * t298 * t105 - t304 * t305 / 3.0 - t104 * t309 + 2.0 * t304 * t311 + 2.0 / 3.0 * t314 * t317 - t320 * t110 / 2.0 + t323 * t110 + t104 * t326 / 2.0 - t314 * t330);
        let t334 = t333 * t45;
        let t337 = t114 * t118;
        let t338 = t70 * t337;
        let t342 = 1.0 / t91 / t36;
        let t343 = t44 * t342;
        let t344 = t253 * t343;
        let t345 = t315 * t263;
        let t346 = t256 * t345;
        let t349 = t239 * t121;
        let t352 = t247 * t121;
        let t355 = t268 * t277;
        let t356 = t89 * t355;
        let t359 = t253 * t119;
        let t360 = t315 * t282;
        let t361 = t256 * t360;
        let t366 = t54 / t63 / t62;
        let t367 = t133 * t133;
        let t372 = t5 * t234;
        let t376 = piecewise3(t61, 40.0 / 9.0 * t230 - 11.0 / 9.0 * t56 * t372, 0.0);
        let t380 = -2.0 / 3.0 * t241 * t98 + 4.0 / 3.0 * t249 * t98 - 2.0 / 3.0 * t260 * t265 + 2.0 / 3.0 * t90 * t279 - 4.0 / 3.0 * t260 * t284 - 4.0 / 3.0 * t90 * t287 + 8.0 / 3.0 * t260 * t290 - 2.0 * t3 * t334 + 4.0 * t338 * t122 - 4.0 * t344 * t346 + 2.0 * t120 * t349 - 4.0 * t120 * t352 - 2.0 * t120 * t356 + 4.0 * t359 * t361 - t125 * t366 * t367 / 24.0 + t125 * t127 * t376 / 12.0;
        let t381 = t380 * t5;
        let tv3rho30 = t381 * t6 / 2.0 + t138 * t141 / 3.0 - t68 * t9 / 9.0;
        v3rho3[ip] += tv3rho30;
        let t389 = piecewise3(t22, t71, -0.17777777777777777778e0 * t129);
        let t390 = t389 * t84;
        let t391 = t390 * t88;
        let t392 = t70 * t391;
        let t395 = t145 * t246;
        let t396 = t395 * t88;
        let t397 = t70 * t396;
        let t398 = t97 * t38;
        let t399 = t270 * t82;
        let t400 = t398 * t399;
        let t403 = t145 * t255;
        let t404 = t403 * t258;
        let t405 = t253 * t404;
        let t406 = t264 * t38;
        let t407 = t406 * t399;
        let t412 = t283 * t38;
        let t413 = t412 * t399;
        let t418 = t43 * t118;
        let t419 = t418 * t82;
        let t420 = t406 * t419;
        let t423 = t97 * t159;
        let t426 = t101 * t389;
        let t427 = t426 * t103;
        let t429 = t151 * t297;
        let t430 = t42 * t82;
        let t431 = t97 * t430;
        let t434 = t301 * t145;
        let t435 = t434 * t303;
        let t436 = t264 * t430;
        let t440 = t283 * t430;
        let t443 = t39 * t82;
        let t444 = t264 * t443;
        let t447 = t426 * t84;
        let t450 = t97 * t443;
        let t454 = t283 * t443;
        let t457 = t38 * (t427 * t105 - 2.0 * t429 * t431 - t435 * t436 / 3.0 - t152 * t309 + 2.0 * t435 * t440 + 2.0 / 3.0 * t435 * t444 - t447 * t110 / 2.0 + t429 * t450 + t152 * t326 / 2.0 - t435 * t454);
        let t458 = t457 * t45;
        let t461 = t158 * t118;
        let t462 = t70 * t461;
        let t467 = t264 * t82;
        let t468 = t404 * t467;
        let t471 = t390 * t121;
        let t474 = t97 * t82;
        let t475 = t396 * t474;
        let t478 = t147 * t355;
        let t481 = t283 * t82;
        let t482 = t404 * t481;
        let t485 = t168 * t133;
        let t491 = piecewise3(t61, t165 * t78 / 3.0, 0.0);
        let t495 = -2.0 / 3.0 * t392 * t98 + 4.0 / 3.0 * t397 * t400 - 2.0 / 3.0 * t405 * t407 + 2.0 / 3.0 * t148 * t279 - 4.0 / 3.0 * t405 * t413 - 2.0 / 3.0 * t148 * t287 + 8.0 / 3.0 * t405 * t420 - 2.0 / 3.0 * t90 * t423 - 2.0 * t3 * t458 + 2.0 * t462 * t122 + 2.0 * t338 * t162 - 4.0 * t344 * t468 + 2.0 * t120 * t471 - 4.0 * t120 * t475 - 2.0 * t120 * t478 + 4.0 * t359 * t482 - t125 * t366 * t485 / 24.0 + t125 * t127 * t491 / 12.0;
        let t496 = t495 * t5;
        let tv3rho2sigma0 = t496 * t6 / 2.0 + t173 * t141 / 6.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t502 = piecewise3(t22, t71, -5.0 / 18.0 * t57);
        let t503 = t502 * t84;
        let t504 = t503 * t88;
        let t505 = t70 * t504;
        let t508 = t177 * t246;
        let t509 = t508 * t88;
        let t510 = t70 * t509;
        let t513 = t177 * t255;
        let t514 = t513 * t258;
        let t515 = t253 * t514;
        let t526 = t97 * t191;
        let t529 = t101 * t502;
        let t530 = t529 * t103;
        let t532 = t183 * t297;
        let t535 = t301 * t177;
        let t536 = t535 * t303;
        let t544 = t529 * t84;
        let t552 = t38 * (t530 * t105 - 2.0 * t532 * t431 - t536 * t436 / 3.0 - t184 * t309 + 2.0 * t536 * t440 + 2.0 / 3.0 * t536 * t444 - t544 * t110 / 2.0 + t532 * t450 + t184 * t326 / 2.0 - t536 * t454);
        let t553 = t552 * t45;
        let t556 = t190 * t118;
        let t557 = t70 * t556;
        let t562 = t514 * t467;
        let t565 = t503 * t121;
        let t568 = t509 * t474;
        let t571 = t179 * t355;
        let t574 = t514 * t481;
        let t577 = -2.0 / 3.0 * t505 * t98 + 4.0 / 3.0 * t510 * t400 - 2.0 / 3.0 * t515 * t407 + 2.0 / 3.0 * t180 * t279 - 4.0 / 3.0 * t515 * t413 - 2.0 / 3.0 * t180 * t287 + 8.0 / 3.0 * t515 * t420 - 2.0 / 3.0 * t90 * t526 - 2.0 * t3 * t553 + 2.0 * t557 * t122 + 2.0 * t338 * t194 - 4.0 * t344 * t562 + 2.0 * t120 * t565 - 4.0 * t120 * t568 - 2.0 * t120 * t571 + 4.0 * t359 * t574;
        let t578 = t577 * t5;
        let tv3rho2lapl0 = t578 * t6 / 2.0 + t198 * t141 / 6.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t584 = piecewise3(t22, t71, 0.88888888888888888889e0 * t57);
        let t585 = t584 * t84;
        let t586 = t585 * t88;
        let t587 = t70 * t586;
        let t590 = t201 * t246;
        let t591 = t590 * t88;
        let t592 = t70 * t591;
        let t595 = t201 * t255;
        let t596 = t595 * t258;
        let t597 = t253 * t596;
        let t608 = t97 * t215;
        let t611 = t101 * t584;
        let t612 = t611 * t103;
        let t614 = t207 * t297;
        let t617 = t301 * t201;
        let t618 = t617 * t303;
        let t626 = t611 * t84;
        let t634 = t38 * (t612 * t105 - 2.0 * t614 * t431 - t618 * t436 / 3.0 - t208 * t309 + 2.0 * t618 * t440 + 2.0 / 3.0 * t618 * t444 - t626 * t110 / 2.0 + t614 * t450 + t208 * t326 / 2.0 - t618 * t454);
        let t635 = t634 * t45;
        let t638 = t214 * t118;
        let t639 = t70 * t638;
        let t644 = t596 * t467;
        let t647 = t585 * t121;
        let t650 = t591 * t474;
        let t653 = t203 * t355;
        let t656 = t596 * t481;
        let t664 = piecewise3(t61, -5.0 / 3.0 * t57, 0.0);
        let t668 = -2.0 / 3.0 * t587 * t98 + 4.0 / 3.0 * t592 * t400 - 2.0 / 3.0 * t597 * t407 + 2.0 / 3.0 * t204 * t279 - 4.0 / 3.0 * t597 * t413 - 2.0 / 3.0 * t204 * t287 + 8.0 / 3.0 * t597 * t420 - 2.0 / 3.0 * t90 * t608 - 2.0 * t3 * t635 + 2.0 * t639 * t122 + 2.0 * t338 * t218 - 4.0 * t344 * t644 + 2.0 * t120 * t647 - 4.0 * t120 * t650 - 2.0 * t120 * t653 + 4.0 * t359 * t656 - t125 * t366 * t221 * t133 / 24.0 + t125 * t127 * t664 / 12.0;
        let t669 = t668 * t5;
        let tv3rho2tau0 = t669 * t6 / 2.0 + t226 * t141 / 6.0;
        v3rho2tau[ip] += tv3rho2tau0;
        let t674 = piecewise3(t22, t71, 0.0);
        let t675 = t674 * t84;
        let t676 = t675 * t88;
        let t677 = t70 * t676;
        let t679 = 2.0 / 3.0 * t677 * t98;
        let t680 = t145 * t145;
        let t681 = t680 * t246;
        let t682 = t681 * t88;
        let t683 = t70 * t682;
        let t686 = t680 * t255;
        let t687 = t686 * t258;
        let t688 = t253 * t687;
        let t695 = 2.0 * t271 * t145 * t273 - 2.0 * t154 * t121;
        let t696 = t270 * t695;
        let t697 = t269 * t696;
        let t706 = t101 * t674;
        let t707 = t706 * t103;
        let t708 = t707 * t105;
        let t709 = t101 * t680;
        let t710 = t709 * t297;
        let t713 = t301 * t680;
        let t714 = t713 * t303;
        let t717 = t42 * t695;
        let t718 = t268 * t717;
        let t722 = t713 * t255;
        let t725 = t706 * t84;
        let t727 = t725 * t110 / 2.0;
        let t728 = t709 * t246;
        let t730 = t39 * t695;
        let t731 = t268 * t730;
        let t736 = t38 * (t708 - 2.0 * t710 * t105 - t714 * t305 / 3.0 - t152 * t718 + 2.0 * t714 * t311 + 2.0 / 3.0 * t722 * t317 - t727 + t728 * t110 + t152 * t731 / 2.0 - t722 * t330);
        let t737 = t736 * t45;
        let t742 = t686 * t345;
        let t745 = t675 * t121;
        let t747 = 2.0 * t120 * t745;
        let t748 = t681 * t121;
        let t751 = t268 * t695;
        let t752 = t147 * t751;
        let t755 = t686 * t360;
        let t758 = t168 * t168;
        let t762 = piecewise3(t61, 0.0, 0.0);
        let t765 = t125 * t127 * t762 / 12.0;
        let t766 = -t679 + 4.0 / 3.0 * t683 * t98 - 2.0 / 3.0 * t688 * t265 + 2.0 / 3.0 * t148 * t697 - 4.0 / 3.0 * t688 * t284 - 4.0 / 3.0 * t148 * t423 + 8.0 / 3.0 * t688 * t290 - 2.0 * t3 * t737 + 4.0 * t462 * t162 - 4.0 * t344 * t742 + t747 - 4.0 * t120 * t748 - 2.0 * t120 * t752 + 4.0 * t359 * t755 - t125 * t366 * t758 / 24.0 + t765;
        let t767 = t766 * t5;
        let tv3rhosigma20 = t767 * t6 / 2.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t769 = t270 * t145;
        let t770 = t398 * t769;
        let t773 = t406 * t769;
        let t778 = t412 * t769;
        let t783 = t418 * t145;
        let t784 = t406 * t783;
        let t789 = t42 * t145;
        let t790 = t97 * t789;
        let t793 = t264 * t789;
        let t797 = t283 * t789;
        let t800 = t39 * t145;
        let t801 = t264 * t800;
        let t804 = t97 * t800;
        let t808 = t283 * t800;
        let t811 = t38 * (t708 - 2.0 * t532 * t790 - t536 * t793 / 3.0 - t184 * t718 + 2.0 * t536 * t797 + 2.0 / 3.0 * t536 * t801 - t727 + t532 * t804 + t184 * t731 / 2.0 - t536 * t808);
        let t812 = t811 * t45;
        let t819 = t264 * t145;
        let t820 = t514 * t819;
        let t823 = t97 * t145;
        let t824 = t509 * t823;
        let t827 = t179 * t751;
        let t830 = t283 * t145;
        let t831 = t514 * t830;
        let t834 = -t679 + 4.0 / 3.0 * t510 * t770 - 2.0 / 3.0 * t515 * t773 + 2.0 / 3.0 * t180 * t697 - 4.0 / 3.0 * t515 * t778 - 2.0 / 3.0 * t180 * t423 + 8.0 / 3.0 * t515 * t784 - 2.0 / 3.0 * t148 * t526 - 2.0 * t3 * t812 + 2.0 * t557 * t162 + 2.0 * t462 * t194 - 4.0 * t344 * t820 + t747 - 4.0 * t120 * t824 - 2.0 * t120 * t827 + 4.0 * t359 * t831;
        let t835 = t834 * t5;
        let tv3rhosigmalapl0 = t835 * t6 / 2.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t865 = t38 * (t708 - 2.0 * t614 * t790 - t618 * t793 / 3.0 - t208 * t718 + 2.0 * t618 * t797 + 2.0 / 3.0 * t618 * t801 - t727 + t614 * t804 + t208 * t731 / 2.0 - t618 * t808);
        let t866 = t865 * t45;
        let t873 = t596 * t819;
        let t876 = t591 * t823;
        let t879 = t203 * t751;
        let t882 = t596 * t830;
        let t889 = -t679 + 4.0 / 3.0 * t592 * t770 - 2.0 / 3.0 * t597 * t773 + 2.0 / 3.0 * t204 * t697 - 4.0 / 3.0 * t597 * t778 - 2.0 / 3.0 * t204 * t423 + 8.0 / 3.0 * t597 * t784 - 2.0 / 3.0 * t148 * t608 - 2.0 * t3 * t866 + 2.0 * t639 * t162 + 2.0 * t462 * t218 - 4.0 * t344 * t873 + t747 - 4.0 * t120 * t876 - 2.0 * t120 * t879 + 4.0 * t359 * t882 - t125 * t366 * t221 * t168 / 24.0 + t765;
        let t890 = t889 * t5;
        let tv3rhosigmatau0 = t890 * t6 / 2.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t892 = t177 * t177;
        let t893 = t892 * t246;
        let t894 = t893 * t88;
        let t895 = t70 * t894;
        let t898 = t892 * t255;
        let t899 = t898 * t258;
        let t900 = t253 * t899;
        let t907 = 2.0 * t271 * t177 * t273 - 2.0 * t186 * t121;
        let t908 = t270 * t907;
        let t909 = t269 * t908;
        let t918 = t101 * t892;
        let t919 = t918 * t297;
        let t922 = t301 * t892;
        let t923 = t922 * t303;
        let t926 = t42 * t907;
        let t927 = t268 * t926;
        let t931 = t922 * t255;
        let t934 = t918 * t246;
        let t936 = t39 * t907;
        let t937 = t268 * t936;
        let t942 = t38 * (t708 - 2.0 * t919 * t105 - t923 * t305 / 3.0 - t184 * t927 + 2.0 * t923 * t311 + 2.0 / 3.0 * t931 * t317 - t727 + t934 * t110 + t184 * t937 / 2.0 - t931 * t330);
        let t943 = t942 * t45;
        let t948 = t898 * t345;
        let t951 = t893 * t121;
        let t954 = t268 * t907;
        let t955 = t179 * t954;
        let t958 = t898 * t360;
        let t961 = -t679 + 4.0 / 3.0 * t895 * t98 - 2.0 / 3.0 * t900 * t265 + 2.0 / 3.0 * t180 * t909 - 4.0 / 3.0 * t900 * t284 - 4.0 / 3.0 * t180 * t526 + 8.0 / 3.0 * t900 * t290 - 2.0 * t3 * t943 + 4.0 * t557 * t194 - 4.0 * t344 * t948 + t747 - 4.0 * t120 * t951 - 2.0 * t120 * t955 + 4.0 * t359 * t958;
        let t962 = t961 * t5;
        let tv3rholapl20 = t962 * t6 / 2.0;
        v3rholapl2[ip] += tv3rholapl20;
        let t964 = t270 * t177;
        let t965 = t398 * t964;
        let t968 = t406 * t964;
        let t973 = t412 * t964;
        let t978 = t418 * t177;
        let t979 = t406 * t978;
        let t984 = t42 * t177;
        let t985 = t97 * t984;
        let t988 = t264 * t984;
        let t992 = t283 * t984;
        let t995 = t39 * t177;
        let t996 = t264 * t995;
        let t999 = t97 * t995;
        let t1003 = t283 * t995;
        let t1006 = t38 * (t708 - 2.0 * t614 * t985 - t618 * t988 / 3.0 - t208 * t927 + 2.0 * t618 * t992 + 2.0 / 3.0 * t618 * t996 - t727 + t614 * t999 + t208 * t937 / 2.0 - t618 * t1003);
        let t1007 = t1006 * t45;
        let t1014 = t264 * t177;
        let t1015 = t596 * t1014;
        let t1018 = t97 * t177;
        let t1019 = t591 * t1018;
        let t1022 = t203 * t954;
        let t1025 = t283 * t177;
        let t1026 = t596 * t1025;
        let t1029 = -t679 + 4.0 / 3.0 * t592 * t965 - 2.0 / 3.0 * t597 * t968 + 2.0 / 3.0 * t204 * t909 - 4.0 / 3.0 * t597 * t973 - 2.0 / 3.0 * t204 * t526 + 8.0 / 3.0 * t597 * t979 - 2.0 / 3.0 * t180 * t608 - 2.0 * t3 * t1007 + 2.0 * t639 * t194 + 2.0 * t557 * t218 - 4.0 * t344 * t1015 + t747 - 4.0 * t120 * t1019 - 2.0 * t120 * t1022 + 4.0 * t359 * t1026;
        let t1030 = t1029 * t5;
        let tv3rholapltau0 = t1030 * t6 / 2.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t1032 = t201 * t201;
        let t1033 = t1032 * t246;
        let t1034 = t1033 * t88;
        let t1035 = t70 * t1034;
        let t1038 = t1032 * t255;
        let t1039 = t1038 * t258;
        let t1040 = t253 * t1039;
        let t1047 = 2.0 * t271 * t201 * t273 - 2.0 * t210 * t121;
        let t1048 = t270 * t1047;
        let t1049 = t269 * t1048;
        let t1058 = t101 * t1032;
        let t1059 = t1058 * t297;
        let t1062 = t301 * t1032;
        let t1063 = t1062 * t303;
        let t1066 = t42 * t1047;
        let t1067 = t268 * t1066;
        let t1071 = t1062 * t255;
        let t1074 = t1058 * t246;
        let t1076 = t39 * t1047;
        let t1077 = t268 * t1076;
        let t1082 = t38 * (t708 - 2.0 * t1059 * t105 - t1063 * t305 / 3.0 - t208 * t1067 + 2.0 * t1063 * t311 + 2.0 / 3.0 * t1071 * t317 - t727 + t1074 * t110 + t208 * t1077 / 2.0 - t1071 * t330);
        let t1083 = t1082 * t45;
        let t1088 = t1038 * t345;
        let t1091 = t1033 * t121;
        let t1094 = t268 * t1047;
        let t1095 = t203 * t1094;
        let t1098 = t1038 * t360;
        let t1101 = t221 * t221;
        let t1105 = -t679 + 4.0 / 3.0 * t1035 * t98 - 2.0 / 3.0 * t1040 * t265 + 2.0 / 3.0 * t204 * t1049 - 4.0 / 3.0 * t1040 * t284 - 4.0 / 3.0 * t204 * t608 + 8.0 / 3.0 * t1040 * t290 - 2.0 * t3 * t1083 + 4.0 * t639 * t218 - 4.0 * t344 * t1088 + t747 - 4.0 * t120 * t1091 - 2.0 * t120 * t1095 + 4.0 * t359 * t1098 - t125 * t366 * t1101 / 24.0 + t765;
        let t1106 = t1105 * t5;
        let tv3rhotau20 = t1106 * t6 / 2.0;
        v3rhotau2[ip] += tv3rhotau20;
    }
}
