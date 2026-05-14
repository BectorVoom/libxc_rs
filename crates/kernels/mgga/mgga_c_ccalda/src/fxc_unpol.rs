//! MGGA_C_CCALDA fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ccalda.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_ccalda_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = 1.0 + param_c;
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / rho[ip];
        let t8 = rho[ip] * rho[ip];
        let t10 = 1.0 / t4 / t8;
        let t13 = tau[ip] * t6 - sigma[ip] * t10 / 8.0;
        let t14 = t2 * t13;
        let t15 = M_CBRT6;
        let t16 = t14 * t15;
        let t17 = M_PI * M_PI;
        let t18 = pow_1_3(t17);
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t26 = t15 * t20 * t22;
        let t29 = 1.0 + 5.0 / 9.0 * param_c * t13 * t26;
        let t30 = 1.0 / t29;
        let t31 = M_CBRT3;
        let t32 = 1.0 / M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t31 * t33;
        let t35 = M_CBRT4;
        let t36 = t35 * t35;
        let t39 = t34 * t36 / t3;
        let t41 = 1.0 + 0.53425e-1 * t39;
        let t42 = f64::sqrt(t39);
        let t45 = pow_3_2(t39);
        let t47 = t31 * t31;
        let t48 = t33 * t33;
        let t49 = t47 * t48;
        let t52 = t49 * t35 / t4;
        let t54 = 0.379785e1 * t42 + 0.8969e0 * t39 + 0.204775e0 * t45 + 0.123235e0 * t52;
        let t57 = 1.0 + 0.16081979498692535067e2 / t54;
        let t58 = f64::ln(t57);
        let t62 = pow_1_3(zeta_threshold);
        let t64 = piecewise3(1.0 <= zeta_threshold, t62 * zeta_threshold, 1.0);
        let t70 = (2.0 * t64 - 2.0) / (2.0 * t21 - 2.0);
        let t72 = 1.0 + 0.278125e-1 * t39;
        let t77 = 0.51785e1 * t42 + 0.905775e0 * t39 + 0.1100325e0 * t45 + 0.1241775e0 * t52;
        let t80 = 1.0 + 0.29608749977793437516e2 / t77;
        let t81 = f64::ln(t80);
        let t85 = -0.621814e-1 * t41 * t58 + 0.19751673498613801407e-1 * t70 * t72 * t81;
        let t87 = t23 * t30 * t85;
        let t89 = 5.0 / 9.0 * t16 * t87;
        let t90 = t23 * t30;
        let t93 = 1.0 - 5.0 / 9.0 * t16 * t90;
        let t94 = t93 * t85;
        let tzk0 = t89 + t94;
        zk[ip] += tzk0;
        let t97 = t8 * rho[ip];
        let t99 = 1.0 / t4 / t97;
        let t102 = -5.0 / 3.0 * tau[ip] * t10 + sigma[ip] * t99 / 3.0;
        let t103 = t2 * t102;
        let t104 = t103 * t15;
        let t105 = t104 * t87;
        let t107 = t15 * t15;
        let t109 = 1.0 / t18 / t17;
        let t110 = t107 * t109;
        let t111 = t14 * t110;
        let t112 = t29 * t29;
        let t113 = 1.0 / t112;
        let t114 = t21 * t113;
        let t115 = t85 * param_c;
        let t117 = t114 * t115 * t102;
        let t118 = t111 * t117;
        let t121 = 1.0 / t3 / rho[ip];
        let t122 = t36 * t121;
        let t126 = t54 * t54;
        let t127 = 1.0 / t126;
        let t128 = t41 * t127;
        let t130 = 1.0 / t42 * t31;
        let t131 = t33 * t36;
        let t132 = t131 * t121;
        let t133 = t130 * t132;
        let t135 = t34 * t122;
        let t137 = f64::sqrt(t39);
        let t138 = t137 * t31;
        let t139 = t138 * t132;
        let t142 = t49 * t35 * t6;
        let t144 = -0.632975e0 * t133 - 0.29896666666666666667e0 * t135 - 0.1023875e0 * t139 - 0.82156666666666666667e-1 * t142;
        let t145 = 1.0 / t57;
        let t146 = t144 * t145;
        let t149 = t70 * t31;
        let t154 = t70 * t72;
        let t155 = t77 * t77;
        let t156 = 1.0 / t155;
        let t161 = -0.86308333333333333334e0 * t133 - 0.301925e0 * t135 - 0.5501625e-1 * t139 - 0.82785e-1 * t142;
        let t163 = 1.0 / t80;
        let t164 = t156 * t161 * t163;
        let t167 = 0.11073470983333333333e-2 * t34 * t122 * t58 + 1.0 * t128 * t146 - 0.18311447306006545054e-3 * t149 * t131 * t121 * t81 - 0.5848223622634646207e0 * t154 * t164;
        let t169 = t23 * t30 * t167;
        let t170 = t16 * t169;
        let t175 = t114 * param_c * t102;
        let t178 = -5.0 / 9.0 * t104 * t90 + 50.0 / 81.0 * t111 * t175;
        let t179 = t178 * t85;
        let t180 = t93 * t167;
        let tvrho0 = t89 + t94 + rho[ip] * (5.0 / 9.0 * t105 - 50.0 / 81.0 * t118 + 5.0 / 9.0 * t170 + t179 + t180);
        vrho[ip] += tvrho0;
        let t183 = t2 * t10;
        let t184 = t183 * t15;
        let t185 = t184 * t87;
        let t186 = 5.0 / 72.0 * t185;
        let t189 = t111 * t114 * t115 * t10;
        let t190 = 25.0 / 324.0 * t189;
        let t191 = t184 * t90;
        let t195 = t111 * t114 * param_c * t10;
        let t197 = 5.0 / 72.0 * t191 - 25.0 / 324.0 * t195;
        let t198 = t197 * t85;
        let tvsigma0 = rho[ip] * (-t186 + t190 + t198);
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t200 = t2 * t6;
        let t201 = t200 * t15;
        let t203 = 5.0 / 9.0 * t201 * t87;
        let t207 = 50.0 / 81.0 * t111 * t114 * t115 * t6;
        let t214 = -5.0 / 9.0 * t201 * t90 + 50.0 / 81.0 * t111 * t114 * param_c * t6;
        let t215 = t214 * t85;
        let tvtau0 = rho[ip] * (t203 - t207 + t215);
        vtau[ip] += tvtau0;
        let t224 = t8 * t8;
        let t226 = 1.0 / t4 / t224;
        let t229 = 40.0 / 9.0 * tau[ip] * t99 - 11.0 / 9.0 * sigma[ip] * t226;
        let t230 = t2 * t229;
        let t231 = t230 * t15;
        let t232 = t231 * t87;
        let t234 = t102 * t102;
        let t235 = t2 * t234;
        let t236 = t235 * t110;
        let t237 = t114 * t115;
        let t238 = t236 * t237;
        let t240 = t104 * t169;
        let t242 = t17 * t17;
        let t243 = 1.0 / t242;
        let t244 = t14 * t243;
        let t246 = 1.0 / t112 / t29;
        let t247 = t246 * t85;
        let t248 = param_c * param_c;
        let t249 = t248 * t234;
        let t250 = t247 * t249;
        let t251 = t244 * t250;
        let t253 = t167 * param_c;
        let t255 = t114 * t253 * t102;
        let t256 = t111 * t255;
        let t259 = t114 * t115 * t229;
        let t260 = t111 * t259;
        let t263 = 1.0 / t3 / t8;
        let t264 = t36 * t263;
        let t268 = t34 * t36;
        let t269 = t121 * t127;
        let t273 = t126 * t54;
        let t274 = 1.0 / t273;
        let t275 = t41 * t274;
        let t276 = t144 * t144;
        let t277 = t276 * t145;
        let t282 = 1.0 / t42 / t39 * t47;
        let t283 = t48 * t35;
        let t284 = t283 * t10;
        let t285 = t282 * t284;
        let t287 = t131 * t263;
        let t288 = t130 * t287;
        let t290 = t34 * t264;
        let t292 = 1.0/f64::sqrt(t39);
        let t293 = t292 * t47;
        let t294 = t293 * t284;
        let t296 = t138 * t287;
        let t299 = t49 * t35 * t10;
        let t301 = -0.42198333333333333333e0 * t285 + 0.84396666666666666666e0 * t288 + 0.39862222222222222223e0 * t290 + 0.68258333333333333333e-1 * t294 + 0.13651666666666666667e0 * t296 + 0.13692777777777777778e0 * t299;
        let t302 = t301 * t145;
        let t305 = t126 * t126;
        let t306 = 1.0 / t305;
        let t307 = t41 * t306;
        let t308 = t57 * t57;
        let t309 = 1.0 / t308;
        let t310 = t276 * t309;
        let t317 = t70 * t34;
        let t321 = t155 * t77;
        let t322 = 1.0 / t321;
        let t323 = t161 * t161;
        let t325 = t322 * t323 * t163;
        let t334 = -0.57538888888888888889e0 * t285 + 0.11507777777777777778e1 * t288 + 0.40256666666666666667e0 * t290 + 0.366775e-1 * t294 + 0.73355e-1 * t296 + 0.137975e0 * t299;
        let t336 = t156 * t334 * t163;
        let t339 = t155 * t155;
        let t340 = 1.0 / t339;
        let t341 = t340 * t323;
        let t342 = t80 * t80;
        let t343 = 1.0 / t342;
        let t344 = t341 * t343;
        let t347 = -0.14764627977777777777e-2 * t34 * t264 * t58 - 0.35616666666666666666e-1 * t268 * t269 * t146 - 2.0 * t275 * t277 + 1.0 * t128 * t302 + 0.16081979498692535067e2 * t307 * t310 + 0.24415263074675393405e-3 * t149 * t131 * t263 * t81 + 0.10843581300301739842e-1 * t317 * t122 * t164 + 0.11696447245269292414e1 * t154 * t325 - 0.5848223622634646207e0 * t154 * t336 - 0.17315859105681463759e2 * t154 * t344;
        let t349 = t23 * t30 * t347;
        let t350 = t16 * t349;
        let t355 = t109 * t21;
        let t357 = t355 * t113 * param_c;
        let t360 = t246 * t248;
        let t361 = t360 * t234;
        let t365 = t114 * param_c * t229;
        let t368 = -5.0 / 9.0 * t231 * t90 + 100.0 / 81.0 * t235 * t107 * t357 - 2000.0 / 243.0 * t244 * t361 + 50.0 / 81.0 * t111 * t365;
        let t369 = t368 * t85;
        let t370 = t178 * t167;
        let t372 = t93 * t347;
        let tv2rho20 = 10.0 / 9.0 * t105 - 100.0 / 81.0 * t118 + 10.0 / 9.0 * t170 + 2.0 * t179 + 2.0 * t180 + rho[ip] * (5.0 / 9.0 * t232 - 100.0 / 81.0 * t238 + 10.0 / 9.0 * t240 + 2000.0 / 243.0 * t251 - 100.0 / 81.0 * t256 - 50.0 / 81.0 * t260 + 5.0 / 9.0 * t350 + t369 + 2.0 * t370 + t372);
        v2rho2[ip] += tv2rho20;
        let t375 = t2 * t99;
        let t376 = t375 * t15;
        let t377 = t376 * t87;
        let t379 = t183 * t110;
        let t380 = t379 * t117;
        let t382 = t184 * t169;
        let t384 = t243 * t246;
        let t385 = t14 * t384;
        let t386 = t85 * t248;
        let t387 = t10 * t102;
        let t389 = t385 * t386 * t387;
        let t393 = t111 * t114 * t253 * t10;
        let t397 = t111 * t114 * t115 * t99;
        let t399 = t376 * t90;
        let t401 = t379 * t175;
        let t404 = t244 * t360 * t387;
        let t408 = t111 * t114 * param_c * t99;
        let t410 = -5.0 / 27.0 * t399 - 25.0 / 162.0 * t401 + 250.0 / 243.0 * t404 + 50.0 / 243.0 * t408;
        let t411 = t410 * t85;
        let t412 = t197 * t167;
        let tv2rhosigma0 = -t186 + t190 + t198 + rho[ip] * (5.0 / 27.0 * t377 + 25.0 / 162.0 * t380 - 5.0 / 72.0 * t382 - 250.0 / 243.0 * t389 + 25.0 / 324.0 * t393 - 50.0 / 243.0 * t397 + t411 + t412);
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t416 = t200 * t110;
        let t417 = t416 * t117;
        let t419 = t201 * t169;
        let t421 = t6 * t102;
        let t423 = t385 * t386 * t421;
        let t427 = t111 * t114 * t253 * t6;
        let t437 = 25.0 / 27.0 * t191 + 100.0 / 81.0 * t416 * t175 - 2000.0 / 243.0 * t244 * t360 * t421 - 250.0 / 243.0 * t195;
        let t438 = t437 * t85;
        let t439 = t214 * t167;
        let tv2rhotau0 = t203 - t207 + t215 + rho[ip] * (-25.0 / 27.0 * t185 - 100.0 / 81.0 * t417 + 5.0 / 9.0 * t419 + 2000.0 / 243.0 * t423 - 50.0 / 81.0 * t427 + 250.0 / 243.0 * t189 + t438 + t439);
        v2rhotau[ip] += tv2rhotau0;
        let t442 = t224 * rho[ip];
        let t444 = 1.0 / t3 / t442;
        let t445 = t2 * t444;
        let t446 = t445 * t110;
        let t447 = t446 * t237;
        let t448 = 25.0 / 1296.0 * t447;
        let t449 = t248 * t444;
        let t451 = t244 * t247 * t449;
        let t452 = 125.0 / 972.0 * t451;
        let t454 = t445 * t107 * t357;
        let t457 = t244 * t360 * t444;
        let t459 = 25.0 / 1296.0 * t454 - 125.0 / 972.0 * t457;
        let t460 = t459 * t85;
        let tv2sigma20 = rho[ip] * (-t448 + t452 + t460);
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t463 = 1.0 / t3 / t224;
        let t464 = t2 * t463;
        let t465 = t464 * t110;
        let t466 = t465 * t237;
        let t467 = 25.0 / 162.0 * t466;
        let t468 = t248 * t463;
        let t470 = t244 * t247 * t468;
        let t471 = 250.0 / 243.0 * t470;
        let t473 = t464 * t107 * t357;
        let t476 = t244 * t360 * t463;
        let t478 = -25.0 / 162.0 * t473 + 250.0 / 243.0 * t476;
        let t479 = t478 * t85;
        let tv2sigmatau0 = rho[ip] * (t467 - t471 + t479);
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t482 = 1.0 / t3 / t97;
        let t483 = t2 * t482;
        let t484 = t483 * t110;
        let t486 = 100.0 / 81.0 * t484 * t237;
        let t487 = t248 * t482;
        let t490 = 2000.0 / 243.0 * t244 * t247 * t487;
        let t497 = 100.0 / 81.0 * t483 * t107 * t357 - 2000.0 / 243.0 * t244 * t360 * t482;
        let t498 = t497 * t85;
        let tv2tau20 = rho[ip] * (-t486 + t490 + t498);
        v2tau2[ip] += tv2tau20;
    }
}
