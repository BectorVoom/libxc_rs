//! MGGA_X_GX fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gx.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{Heaviside, piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gx_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_c0: f64,
    param_c1: f64,
    param_alphainf: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT2;
        let t22 = t4 * t4;
        let t24 = M_CBRT4;
        let t26 = 8.0 / 27.0 * t21 * t22 * t24;
        let t27 = t21 * t21;
        let t28 = tau[ip] * t27;
        let t29 = t19 * t19;
        let t31 = 1.0 / t29 / rho[ip];
        let t33 = sigma[ip] * t27;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t29 / t34;
        let t39 = t28 * t31 - t33 * t36 / 8.0;
        let t40 = M_CBRT6;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t39 * t40 * t45;
        let t48 = t40 * t45;
        let t51 = param_c0 + 5.0 / 9.0 * param_c1 * t39 * t48;
        let t52 = param_c0 + param_c1 - 1.0;
        let t56 = 1.0 + 5.0 / 9.0 * t52 * t39 * t48;
        let t57 = 1.0 / t56;
        let t59 = 1.0 - t26;
        let t60 = t51 * t57 * t59;
        let t63 = t26 + 5.0 / 9.0 * t46 * t60;
        let t64 = 5.0 / 9.0 * t46;
        let t65 = 1.0 - t64;
        let t66 = Heaviside(t65);
        let t68 = 1.0 - param_alphainf;
        let t69 = t68 * t65;
        let t70 = 1.0 + t64;
        let t71 = 1.0 / t70;
        let t73 = t69 * t71 + 1.0;
        let t74 = -t65;
        let t75 = Heaviside(t74);
        let t77 = t63 * t66 + t73 * t75;
        let t81 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t77);
        let tzk0 = 2.0 * t81;
        zk[ip] += tzk0;
        let t83 = t18 / t29;
        let t89 = t34 * rho[ip];
        let t91 = 1.0 / t29 / t89;
        let t94 = -5.0 / 3.0 * t28 * t36 + t33 * t91 / 3.0;
        let t96 = t94 * t40 * t45;
        let t99 = t40 * t40;
        let t100 = t39 * t99;
        let t102 = 1.0 / t43 / t42;
        let t103 = t100 * t102;
        let t105 = t57 * t59;
        let t106 = param_c1 * t94 * t105;
        let t109 = t102 * t51;
        let t110 = t100 * t109;
        let t111 = t56 * t56;
        let t112 = 1.0 / t111;
        let t113 = t112 * t59;
        let t115 = t113 * t52 * t94;
        let t118 = 5.0 / 9.0 * t96 * t60 + 25.0 / 81.0 * t103 * t106 - 25.0 / 81.0 * t110 * t115;
        let t120 = 0.0;
        let t121 = t63 * t120;
        let t125 = t48 * t71;
        let t127 = t70 * t70;
        let t128 = 1.0 / t127;
        let t129 = t69 * t128;
        let t132 = -5.0 / 9.0 * t68 * t94 * t125 - 5.0 / 9.0 * t129 * t96;
        let t134 = t73 * t120;
        let t137 = t118 * t66 - 5.0 / 9.0 * t121 * t96 + t132 * t75 + 5.0 / 9.0 * t134 * t96;
        let t142 = piecewise3(t3, 0.0, -t7 * t83 * t77 / 8.0 - 3.0 / 8.0 * t7 * t20 * t137);
        let tvrho0 = 2.0 * rho[ip] * t142 + 2.0 * t81;
        vrho[ip] += tvrho0;
        let t145 = t27 * t36;
        let t148 = t45 * t51 * t105;
        let t149 = t145 * t40 * t148;
        let t151 = t102 * param_c1;
        let t152 = t100 * t151;
        let t154 = t152 * t145 * t105;
        let t156 = t52 * t27;
        let t159 = t110 * t113 * t156 * t36;
        let t161 = -5.0 / 72.0 * t149 - 25.0 / 648.0 * t154 + 25.0 / 648.0 * t159;
        let t163 = t121 * t27;
        let t165 = t36 * t40 * t45;
        let t166 = t163 * t165;
        let t168 = t68 * t27;
        let t169 = t168 * t36;
        let t170 = t169 * t125;
        let t172 = t129 * t145 * t48;
        let t174 = 5.0 / 72.0 * t170 + 5.0 / 72.0 * t172;
        let t176 = t134 * t27;
        let t177 = t176 * t165;
        let t179 = t161 * t66 + 5.0 / 72.0 * t166 + t174 * t75 - 5.0 / 72.0 * t177;
        let t183 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t179);
        let tvsigma0 = 2.0 * rho[ip] * t183;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t185 = t27 * t31;
        let t196 = 5.0 / 9.0 * t185 * t40 * t148 + 25.0 / 81.0 * t152 * t185 * t105 - 25.0 / 81.0 * t110 * t113 * t156 * t31;
        let t199 = t31 * t40 * t45;
        let t202 = t168 * t31;
        let t207 = -5.0 / 9.0 * t129 * t185 * t48 - 5.0 / 9.0 * t202 * t125;
        let t211 = t196 * t66 - 5.0 / 9.0 * t163 * t199 + t207 * t75 + 5.0 / 9.0 * t176 * t199;
        let t215 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t211);
        let tvtau0 = 2.0 * rho[ip] * t215;
        vtau[ip] += tvtau0;
        let t218 = t18 * t31;
        let t227 = t34 * t34;
        let t229 = 1.0 / t29 / t227;
        let t232 = 40.0 / 9.0 * t28 * t91 - 11.0 / 9.0 * t33 * t229;
        let t234 = t232 * t40 * t45;
        let t237 = t94 * t94;
        let t239 = t237 * t99 * t102;
        let t241 = param_c1 * t57 * t59;
        let t244 = t51 * t112;
        let t245 = t59 * t52;
        let t246 = t244 * t245;
        let t249 = param_c1 * t232;
        let t250 = t249 * t105;
        let t253 = t42 * t42;
        let t254 = 1.0 / t253;
        let t255 = t39 * t254;
        let t256 = t255 * param_c1;
        let t258 = t237 * t112 * t245;
        let t261 = t255 * t51;
        let t263 = 1.0 / t111 / t56;
        let t264 = t263 * t59;
        let t265 = t52 * t52;
        let t267 = t264 * t265 * t237;
        let t271 = t113 * t52 * t232;
        let t274 = 5.0 / 9.0 * t234 * t60 + 50.0 / 81.0 * t239 * t241 - 50.0 / 81.0 * t239 * t246 + 25.0 / 81.0 * t103 * t250 - 500.0 / 243.0 * t256 * t258 + 500.0 / 243.0 * t261 * t267 - 25.0 / 81.0 * t110 * t271;
        let t276 = t118 * t120;
        let t279 = 0.0;
        let t280 = t63 * t279;
        let t285 = t68 * t232;
        let t289 = t99 * t102;
        let t290 = t289 * t128;
        let t294 = 1.0 / t127 / t70;
        let t295 = t69 * t294;
        let t300 = -5.0 / 9.0 * t285 * t125 + 50.0 / 81.0 * t68 * t237 * t290 + 50.0 / 81.0 * t295 * t239 - 5.0 / 9.0 * t129 * t234;
        let t302 = t132 * t120;
        let t305 = t73 * t279;
        let t310 = t274 * t66 - 10.0 / 9.0 * t276 * t96 - 25.0 / 81.0 * t280 * t239 - 5.0 / 9.0 * t121 * t234 + t300 * t75 + 10.0 / 9.0 * t302 * t96 + 25.0 / 81.0 * t305 * t239 + 5.0 / 9.0 * t134 * t234;
        let t315 = piecewise3(t3, 0.0, t7 * t218 * t77 / 12.0 - t7 * t83 * t137 / 4.0 - 3.0 / 8.0 * t7 * t20 * t310);
        let tv2rho20 = 2.0 * rho[ip] * t315 + 4.0 * t142;
        v2rho2[ip] += tv2rho20;
        let t321 = t27 * t91;
        let t323 = t321 * t40 * t148;
        let t325 = t145 * t289;
        let t326 = t325 * t106;
        let t328 = t245 * t94;
        let t329 = t244 * t328;
        let t330 = t325 * t329;
        let t333 = t152 * t321 * t105;
        let t335 = param_c1 * t27;
        let t336 = t255 * t335;
        let t337 = t36 * t112;
        let t339 = t336 * t337 * t328;
        let t341 = t51 * t263;
        let t342 = t255 * t341;
        let t343 = t59 * t265;
        let t346 = t342 * t343 * t145 * t94;
        let t350 = t110 * t113 * t156 * t91;
        let t352 = 5.0 / 27.0 * t323 - 25.0 / 324.0 * t326 + 25.0 / 324.0 * t330 + 25.0 / 243.0 * t333 + 125.0 / 486.0 * t339 - 125.0 / 486.0 * t346 - 25.0 / 243.0 * t350;
        let t354 = t161 * t120;
        let t357 = t276 * t27;
        let t358 = t357 * t165;
        let t360 = t280 * t94;
        let t361 = t360 * t325;
        let t364 = t91 * t40 * t45;
        let t365 = t163 * t364;
        let t367 = t168 * t91;
        let t368 = t367 * t125;
        let t371 = t289 * t128 * t94;
        let t372 = t169 * t371;
        let t375 = t69 * t294 * t27;
        let t376 = t36 * t99;
        let t377 = t102 * t94;
        let t379 = t375 * t376 * t377;
        let t382 = t129 * t321 * t48;
        let t384 = -5.0 / 27.0 * t368 - 25.0 / 324.0 * t372 - 25.0 / 324.0 * t379 - 5.0 / 27.0 * t382;
        let t386 = t174 * t120;
        let t389 = t302 * t27;
        let t390 = t389 * t165;
        let t392 = t305 * t94;
        let t393 = t392 * t325;
        let t395 = t176 * t364;
        let t397 = t352 * t66 - 5.0 / 9.0 * t354 * t96 + 5.0 / 72.0 * t358 + 25.0 / 648.0 * t361 - 5.0 / 27.0 * t365 + t384 * t75 + 5.0 / 9.0 * t386 * t96 - 5.0 / 72.0 * t390 - 25.0 / 648.0 * t393 + 5.0 / 27.0 * t395;
        let t402 = piecewise3(t3, 0.0, -t7 * t83 * t179 / 8.0 - 3.0 / 8.0 * t7 * t20 * t397);
        let tv2rhosigma0 = 2.0 * rho[ip] * t402 + 2.0 * t183;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t409 = t185 * t289;
        let t415 = t31 * t112;
        let t424 = -25.0 / 27.0 * t149 + 50.0 / 81.0 * t409 * t106 - 50.0 / 81.0 * t409 * t329 - 125.0 / 243.0 * t154 - 500.0 / 243.0 * t336 * t415 * t328 + 500.0 / 243.0 * t342 * t343 * t185 * t94 + 125.0 / 243.0 * t159;
        let t426 = t196 * t120;
        let t437 = t31 * t99;
        let t442 = 25.0 / 27.0 * t170 + 50.0 / 81.0 * t202 * t371 + 50.0 / 81.0 * t375 * t437 * t377 + 25.0 / 27.0 * t172;
        let t444 = t207 * t120;
        let t452 = t424 * t66 - 5.0 / 9.0 * t426 * t96 - 5.0 / 9.0 * t357 * t199 - 25.0 / 81.0 * t360 * t409 + 25.0 / 27.0 * t166 + t442 * t75 + 5.0 / 9.0 * t444 * t96 + 5.0 / 9.0 * t389 * t199 + 25.0 / 81.0 * t392 * t409 - 25.0 / 27.0 * t177;
        let t457 = piecewise3(t3, 0.0, -t7 * t83 * t211 / 8.0 - 3.0 / 8.0 * t7 * t20 * t452);
        let tv2rhotau0 = 2.0 * rho[ip] * t457 + 2.0 * t215;
        v2rhotau[ip] += tv2rhotau0;
        let t460 = t227 * rho[ip];
        let t462 = 1.0 / t19 / t460;
        let t463 = t21 * t462;
        let t465 = t151 * t105;
        let t466 = t463 * t99 * t465;
        let t468 = t463 * t289;
        let t469 = t468 * t246;
        let t471 = param_c1 * t21;
        let t472 = t255 * t471;
        let t475 = t472 * t462 * t112 * t245;
        let t478 = t342 * t343 * t463;
        let t480 = 25.0 / 1296.0 * t466 - 25.0 / 1296.0 * t469 - 125.0 / 1944.0 * t475 + 125.0 / 1944.0 * t478;
        let t482 = t354 * t27;
        let t483 = t482 * t165;
        let t485 = t280 * t21;
        let t487 = t462 * t99 * t102;
        let t488 = t485 * t487;
        let t490 = t68 * t21;
        let t491 = t490 * t462;
        let t492 = t491 * t290;
        let t493 = t295 * t468;
        let t495 = 25.0 / 1296.0 * t492 + 25.0 / 1296.0 * t493;
        let t497 = t386 * t27;
        let t498 = t497 * t165;
        let t500 = t305 * t21;
        let t501 = t500 * t487;
        let t503 = t480 * t66 + 5.0 / 36.0 * t483 - 25.0 / 2592.0 * t488 + t495 * t75 - 5.0 / 36.0 * t498 + 25.0 / 2592.0 * t501;
        let t507 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t503);
        let tv2sigma20 = 2.0 * rho[ip] * t507;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t510 = 1.0 / t19 / t227;
        let t511 = t21 * t510;
        let t513 = t511 * t99 * t465;
        let t515 = t511 * t289;
        let t516 = t515 * t246;
        let t520 = t472 * t510 * t112 * t245;
        let t523 = t342 * t343 * t511;
        let t525 = -25.0 / 162.0 * t513 + 25.0 / 162.0 * t516 + 125.0 / 243.0 * t520 - 125.0 / 243.0 * t523;
        let t527 = t426 * t27;
        let t528 = t527 * t165;
        let t533 = t510 * t99 * t102;
        let t534 = t485 * t533;
        let t536 = t490 * t510;
        let t537 = t536 * t290;
        let t538 = t295 * t515;
        let t540 = -25.0 / 162.0 * t537 - 25.0 / 162.0 * t538;
        let t542 = t444 * t27;
        let t543 = t542 * t165;
        let t547 = t500 * t533;
        let t549 = t525 * t66 + 5.0 / 72.0 * t528 - 5.0 / 9.0 * t482 * t199 + 25.0 / 324.0 * t534 + t540 * t75 - 5.0 / 72.0 * t543 + 5.0 / 9.0 * t497 * t199 - 25.0 / 324.0 * t547;
        let t553 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t549);
        let tv2sigmatau0 = 2.0 * rho[ip] * t553;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t556 = 1.0 / t19 / t89;
        let t557 = t21 * t556;
        let t561 = t557 * t289;
        let t571 = 100.0 / 81.0 * t557 * t99 * t465 - 100.0 / 81.0 * t561 * t246 - 1000.0 / 243.0 * t472 * t556 * t112 * t245 + 1000.0 / 243.0 * t342 * t343 * t557;
        let t576 = t556 * t99 * t102;
        let t579 = t490 * t556;
        let t583 = 100.0 / 81.0 * t579 * t290 + 100.0 / 81.0 * t295 * t561;
        let t589 = t571 * t66 - 10.0 / 9.0 * t527 * t199 - 50.0 / 81.0 * t485 * t576 + t583 * t75 + 10.0 / 9.0 * t542 * t199 + 50.0 / 81.0 * t500 * t576;
        let t593 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t589);
        let tv2tau20 = 2.0 * rho[ip] * t593;
        v2tau2[ip] += tv2tau20;
    }
}
