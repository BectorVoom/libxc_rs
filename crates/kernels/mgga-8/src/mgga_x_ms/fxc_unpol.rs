//! MGGA_X_MS fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 52 shared lines across all orders.
//! Delta: 155 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_ms_fxc_unpol(
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
    param_b: f64,
    param_c: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (52 lines) ---
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
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t36 = 5.0 / 972.0 * t26 * t34;
        let t37 = param_kappa + t36;
        let t41 = param_kappa * (1.0 - param_kappa / t37);
        let t42 = tau[ip] * t28;
        let t44 = 1.0 / t31 / rho[ip];
        let t47 = t42 * t44 - t34 / 8.0;
        let t48 = t47 * t47;
        let t49 = t21 * t21;
        let t52 = 1.0 / t23 / t22;
        let t55 = 1.0 - 25.0 / 81.0 * t48 * t49 * t52;
        let t56 = t55 * t55;
        let t57 = t56 * t55;
        let t58 = t48 * t47;
        let t59 = t22 * t22;
        let t60 = 1.0 / t59;
        let t63 = t48 * t48;
        let t66 = t59 * t59;
        let t67 = 1.0 / t66;
        let t70 = 1.0 + 250.0 / 243.0 * t58 * t60 + 62500.0 / 59049.0 * param_b * t63 * t48 * t67;
        let t71 = 1.0 / t70;
        let t72 = t57 * t71;
        let t73 = param_kappa + t36 + param_c;
        let t78 = param_kappa * (1.0 - param_kappa / t73) - t41;
        let t80 = t72 * t78 + t41 + 1.0;
        let t84 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t80);
        let tzk0 = 2.0 * t84;
        zk[ip] += tzk0;
        // --- vxc delta (56 lines) ---
        let t86 = t18 / t31;
        let t90 = param_kappa * param_kappa;
        let t91 = t37 * t37;
        let t94 = t90 / t91 * t21;
        let t95 = t25 * sigma[ip];
        let t96 = t30 * rho[ip];
        let t98 = 1.0 / t31 / t96;
        let t99 = t28 * t98;
        let t100 = t95 * t99;
        let t101 = t94 * t100;
        let t103 = t56 * t71;
        let t104 = t103 * t78;
        let t105 = t47 * t49;
        let t110 = -5.0 / 3.0 * t42 * t33 + t29 * t98 / 3.0;
        let t111 = t52 * t110;
        let t112 = t105 * t111;
        let t115 = t70 * t70;
        let t116 = 1.0 / t115;
        let t117 = t57 * t116;
        let t118 = t48 * t60;
        let t122 = param_b * t63 * t47;
        let t123 = t67 * t110;
        let t126 = 250.0 / 81.0 * t118 * t110 + 125000.0 / 19683.0 * t122 * t123;
        let t127 = t78 * t126;
        let t129 = t73 * t73;
        let t132 = t90 / t129 * t21;
        let t135 = -10.0 / 729.0 * t132 * t100 + 10.0 / 729.0 * t101;
        let t137 = -10.0 / 729.0 * t101 - 50.0 / 27.0 * t104 * t112 - t117 * t127 + t72 * t135;
        let t142 = piecewise3(t3, 0.0, -t7 * t86 * t80 / 8.0 - 3.0 / 8.0 * t7 * t20 * t137);
        let tvrho0 = 2.0 * rho[ip] * t142 + 2.0 * t84;
        vrho[ip] += tvrho0;
        let t145 = t25 * t28;
        let t146 = t145 * t33;
        let t147 = t94 * t146;
        let t149 = t78 * t47;
        let t150 = t103 * t149;
        let t151 = t49 * t52;
        let t152 = t28 * t33;
        let t153 = t151 * t152;
        let t154 = t150 * t153;
        let t156 = t118 * t152;
        let t158 = t67 * t28;
        let t160 = t122 * t158 * t33;
        let t162 = -125.0 / 324.0 * t156 - 15625.0 / 19683.0 * t160;
        let t163 = t78 * t162;
        let t167 = 5.0 / 972.0 * t132 * t146 - 5.0 / 972.0 * t147;
        let t169 = 5.0 / 972.0 * t147 + 25.0 / 108.0 * t154 - t117 * t163 + t72 * t167;
        let t173 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t169);
        let tvsigma0 = 2.0 * rho[ip] * t173;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t175 = t28 * t44;
        let t176 = t151 * t175;
        let t184 = 250.0 / 81.0 * t118 * t175 + 125000.0 / 19683.0 * t122 * t158 * t44;
        let t185 = t78 * t184;
        let t187 = -50.0 / 27.0 * t150 * t176 - t117 * t185;
        let t191 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t187);
        let tvtau0 = 2.0 * rho[ip] * t191;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (155 lines) ---
        let t194 = t18 * t44;
        let t204 = t90 / t91 / t37 * t49;
        let t205 = sigma[ip] * sigma[ip];
        let t206 = t52 * t205;
        let t207 = t30 * t30;
        let t208 = t207 * t96;
        let t210 = 1.0 / t19 / t208;
        let t211 = t27 * t210;
        let t212 = t206 * t211;
        let t214 = 400.0 / 531441.0 * t204 * t212;
        let t216 = 1.0 / t31 / t207;
        let t217 = t28 * t216;
        let t218 = t95 * t217;
        let t220 = 110.0 / 2187.0 * t94 * t218;
        let t221 = t55 * t71;
        let t222 = t221 * t78;
        let t223 = t48 * t21;
        let t225 = 1.0 / t24 / t59;
        let t226 = t110 * t110;
        let t227 = t225 * t226;
        let t228 = t223 * t227;
        let t231 = t56 * t116;
        let t232 = t231 * t149;
        let t233 = t110 * t126;
        let t234 = t151 * t233;
        let t237 = t103 * t135;
        let t240 = t226 * t49;
        let t241 = t240 * t52;
        let t248 = 40.0 / 9.0 * t42 * t98 - 11.0 / 9.0 * t29 * t216;
        let t249 = t52 * t248;
        let t250 = t105 * t249;
        let t254 = 1.0 / t115 / t70;
        let t255 = t57 * t254;
        let t256 = t126 * t126;
        let t257 = t78 * t256;
        let t260 = t135 * t126;
        let t263 = t47 * t60;
        let t268 = param_b * t63;
        let t269 = t67 * t226;
        let t275 = 500.0 / 81.0 * t263 * t226 + 250.0 / 81.0 * t118 * t248 + 625000.0 / 19683.0 * t268 * t269 + 125000.0 / 19683.0 * t122 * t67 * t248;
        let t281 = t90 / t129 / t73 * t49;
        let t286 = -400.0 / 531441.0 * t281 * t212 + 110.0 / 2187.0 * t132 * t218 + t214 - t220;
        let t288 = -t214 + t220 + 10000.0 / 729.0 * t222 * t228 + 100.0 / 27.0 * t232 * t234 - 100.0 / 27.0 * t237 * t112 - 50.0 / 27.0 * t104 * t241 - 50.0 / 27.0 * t104 * t250 + 2.0 * t255 * t257 - 2.0 * t117 * t260 - t117 * t78 * t275 + t72 * t286;
        let t293 = piecewise3(t3, 0.0, t7 * t194 * t80 / 12.0 - t7 * t86 * t137 / 4.0 - 3.0 / 8.0 * t7 * t20 * t288);
        let tv2rho20 = 2.0 * rho[ip] * t293 + 4.0 * t142;
        v2rho2[ip] += tv2rho20;
        let t299 = t52 * t27;
        let t300 = t207 * t30;
        let t302 = 1.0 / t19 / t300;
        let t304 = t299 * t302 * sigma[ip];
        let t306 = 50.0 / 177147.0 * t204 * t304;
        let t307 = t145 * t98;
        let t309 = 10.0 / 729.0 * t94 * t307;
        let t310 = t78 * t48;
        let t311 = t221 * t310;
        let t312 = t21 * t225;
        let t313 = t152 * t110;
        let t314 = t312 * t313;
        let t315 = t311 * t314;
        let t318 = t151 * t152 * t126;
        let t319 = t232 * t318;
        let t321 = t135 * t47;
        let t322 = t103 * t321;
        let t323 = t322 * t153;
        let t325 = t78 * t110;
        let t326 = t103 * t325;
        let t327 = t326 * t153;
        let t329 = t151 * t99;
        let t330 = t150 * t329;
        let t332 = t231 * t163;
        let t338 = t135 * t162;
        let t340 = t263 * t313;
        let t342 = t118 * t99;
        let t344 = t268 * t67;
        let t345 = t344 * t313;
        let t348 = t122 * t158 * t98;
        let t350 = -125.0 / 162.0 * t340 + 250.0 / 243.0 * t342 - 78125.0 / 19683.0 * t345 + 125000.0 / 59049.0 * t348;
        let t351 = t78 * t350;
        let t353 = t103 * t167;
        let t356 = t167 * t126;
        let t362 = 50.0 / 177147.0 * t281 * t304 - 10.0 / 729.0 * t132 * t307 - t306 + t309;
        let t364 = t306 - t309 - 1250.0 / 729.0 * t315 - 25.0 / 108.0 * t319 + 25.0 / 108.0 * t323 + 25.0 / 108.0 * t327 - 50.0 / 81.0 * t330 + 50.0 / 27.0 * t332 * t112 + 2.0 * t255 * t163 * t126 - t117 * t338 - t117 * t351 - 50.0 / 27.0 * t353 * t112 - t117 * t356 + t72 * t362;
        let t369 = piecewise3(t3, 0.0, -t7 * t86 * t169 / 8.0 - 3.0 / 8.0 * t7 * t20 * t364);
        let tv2rhosigma0 = 2.0 * rho[ip] * t369 + 2.0 * t173;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t375 = t175 * t110;
        let t376 = t312 * t375;
        let t380 = t151 * t175 * t126;
        let t388 = t231 * t185;
        let t394 = t135 * t184;
        let t402 = 500.0 / 81.0 * t263 * t375 - 1250.0 / 243.0 * t156 + 625000.0 / 19683.0 * t344 * t375 - 625000.0 / 59049.0 * t160;
        let t403 = t78 * t402;
        let t405 = 10000.0 / 729.0 * t311 * t376 + 50.0 / 27.0 * t232 * t380 - 50.0 / 27.0 * t322 * t176 - 50.0 / 27.0 * t326 * t176 + 250.0 / 81.0 * t154 + 50.0 / 27.0 * t388 * t112 + 2.0 * t255 * t185 * t126 - t117 * t394 - t117 * t403;
        let t410 = piecewise3(t3, 0.0, -t7 * t86 * t187 / 8.0 - 3.0 / 8.0 * t7 * t20 * t405);
        let tv2rhotau0 = 2.0 * rho[ip] * t410 + 2.0 * t191;
        v2rhotau[ip] += tv2rhotau0;
        let t413 = t207 * rho[ip];
        let t415 = 1.0 / t19 / t413;
        let t416 = t299 * t415;
        let t417 = t204 * t416;
        let t419 = t27 * t415;
        let t420 = t312 * t419;
        let t421 = t311 * t420;
        let t424 = t151 * t152 * t162;
        let t425 = t232 * t424;
        let t427 = t167 * t47;
        let t428 = t103 * t427;
        let t429 = t428 * t153;
        let t431 = t419 * t151;
        let t432 = t104 * t431;
        let t434 = t162 * t162;
        let t435 = t78 * t434;
        let t438 = t167 * t162;
        let t441 = t263 * t419;
        let t443 = t67 * t27;
        let t445 = t268 * t443 * t415;
        let t447 = 125.0 / 648.0 * t441 + 78125.0 / 78732.0 * t445;
        let t448 = t78 * t447;
        let t452 = -25.0 / 236196.0 * t281 * t416 + 25.0 / 236196.0 * t417;
        let t454 = -25.0 / 236196.0 * t417 + 625.0 / 1458.0 * t421 - 25.0 / 54.0 * t425 + 25.0 / 54.0 * t429 - 25.0 / 432.0 * t432 + 2.0 * t255 * t435 - 2.0 * t117 * t438 - t117 * t448 + t72 * t452;
        let t458 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t454);
        let tv2sigma20 = 2.0 * rho[ip] * t458;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t461 = 1.0 / t19 / t207;
        let t462 = t27 * t461;
        let t463 = t312 * t462;
        let t464 = t311 * t463;
        let t467 = t151 * t175 * t162;
        let t472 = t462 * t151;
        let t473 = t104 * t472;
        let t475 = t52 * t28;
        let t476 = t475 * t33;
        let t477 = t105 * t476;
        let t478 = t388 * t477;
        let t480 = t185 * t162;
        let t483 = t167 * t184;
        let t485 = t263 * t462;
        let t488 = t268 * t443 * t461;
        let t490 = -125.0 / 81.0 * t485 - 156250.0 / 19683.0 * t488;
        let t491 = t78 * t490;
        let t493 = -2500.0 / 729.0 * t464 + 50.0 / 27.0 * t232 * t467 - 50.0 / 27.0 * t428 * t176 + 25.0 / 54.0 * t473 - 25.0 / 108.0 * t478 + 2.0 * t255 * t480 - t117 * t483 - t117 * t491;
        let t497 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t493);
        let tv2sigmatau0 = 2.0 * rho[ip] * t497;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t500 = 1.0 / t19 / t96;
        let t501 = t27 * t500;
        let t502 = t312 * t501;
        let t506 = t151 * t175 * t184;
        let t509 = t501 * t151;
        let t512 = t184 * t184;
        let t513 = t78 * t512;
        let t521 = 1000.0 / 81.0 * t263 * t501 + 1250000.0 / 19683.0 * t268 * t443 * t500;
        let t522 = t78 * t521;
        let t524 = 20000.0 / 729.0 * t311 * t502 + 100.0 / 27.0 * t232 * t506 - 100.0 / 27.0 * t104 * t509 + 2.0 * t255 * t513 - t117 * t522;
        let t528 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t524);
        let tv2tau20 = 2.0 * rho[ip] * t528;
        v2tau2[ip] += tv2tau20;
    }
}
