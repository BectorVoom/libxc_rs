//! MGGA_C_M06L vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_m06l.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_m06l_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_Fermi_D_cnst: f64,
    param_alpha_ab: f64,
    param_alpha_ss: f64,
    param_cab_0: f64,
    param_cab_1: f64,
    param_cab_2: f64,
    param_cab_3: f64,
    param_cab_4: f64,
    param_css_0: f64,
    param_css_1: f64,
    param_css_2: f64,
    param_css_3: f64,
    param_css_4: f64,
    param_dab_0: f64,
    param_dab_1: f64,
    param_dab_2: f64,
    param_dab_3: f64,
    param_dab_4: f64,
    param_dab_5: f64,
    param_dss_0: f64,
    param_dss_1: f64,
    param_dss_2: f64,
    param_dss_3: f64,
    param_dss_4: f64,
    param_dss_5: f64,
    param_gamma_ab: f64,
    param_gamma_ss: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t4 = 1.0 <= zeta_threshold;
        let t5 = rho[ip] / 2.0 <= dens_threshold || t4;
        let t6 = piecewise3(t4, zeta_threshold, 1.0);
        let t7 = M_CBRT3;
        let t8 = 1.0 / M_PI;
        let t9 = pow_1_3(t8);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = t10 * t12;
        let t14 = pow_1_3(rho[ip]);
        let t15 = 1.0 / t14;
        let t16 = M_CBRT2;
        let t18 = pow_1_3(zeta_threshold);
        let t20 = piecewise3(t4, 1.0 / t18, 1.0);
        let t22 = t13 * t15 * t16 * t20;
        let t24 = 1.0 + 0.53425e-1 * t22;
        let t25 = f64::sqrt(t22);
        let t28 = pow_3_2(t22);
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
        let t52 = piecewise3(2.0 <= zeta_threshold, t50, 2.0 * t16);
        let t54 = piecewise3(0.0 <= zeta_threshold, t50, 0.0);
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
        let t93 = piecewise3(t5, 0.0, t6 * (-t48 + t59 * (-0.310907e-1 * t61 * t70 + t48 - 0.19751673498613801407e-1 * t84) + 0.19751673498613801407e-1 * t59 * t84) / 2.0);
        let t95 = param_css_1;
        let t96 = t95 * param_gamma_ss;
        let t97 = t96 * sigma[ip];
        let t98 = rho[ip] * rho[ip];
        let t100 = 1.0 / t34 / t98;
        let t101 = t36 * t100;
        let t104 = param_gamma_ss * sigma[ip] * t101 + 1.0;
        let t105 = 1.0 / t104;
        let t106 = t101 * t105;
        let t108 = param_css_2;
        let t109 = param_gamma_ss * param_gamma_ss;
        let t110 = t108 * t109;
        let t111 = sigma[ip] * sigma[ip];
        let t112 = t110 * t111;
        let t113 = t98 * t98;
        let t114 = t113 * rho[ip];
        let t116 = 1.0 / t14 / t114;
        let t117 = t16 * t116;
        let t118 = t104 * t104;
        let t119 = 1.0 / t118;
        let t120 = t117 * t119;
        let t123 = param_css_3;
        let t124 = t109 * param_gamma_ss;
        let t125 = t123 * t124;
        let t126 = t111 * sigma[ip];
        let t127 = t113 * t113;
        let t128 = 1.0 / t127;
        let t129 = t126 * t128;
        let t130 = t118 * t104;
        let t131 = 1.0 / t130;
        let t135 = param_css_4;
        let t136 = t109 * t109;
        let t137 = t135 * t136;
        let t138 = t111 * t111;
        let t139 = t137 * t138;
        let t140 = t127 * t98;
        let t142 = 1.0 / t34 / t140;
        let t143 = t36 * t142;
        let t144 = t118 * t118;
        let t145 = 1.0 / t144;
        let t146 = t143 * t145;
        let t149 = 4.0 * t125 * t129 * t131 + t97 * t106 + 2.0 * t112 * t120 + 4.0 * t139 * t146 + param_css_0;
        let t150 = t93 * t149;
        let t151 = 1.0 / rho[ip];
        let t152 = sigma[ip] * t151;
        let t153 = 1.0 / tau[ip];
        let t156 = 1.0 - t152 * t153 / 8.0;
        let t157 = tau[ip] * tau[ip];
        let t159 = t98 * rho[ip];
        let t161 = 1.0 / t14 / t159;
        let t162 = param_Fermi_D_cnst * param_Fermi_D_cnst;
        let t163 = 1.0 / t162;
        let t167 = f64::exp(-8.0 * t157 * t16 * t161 * t163);
        let t168 = 1.0 - t167;
        let t169 = t156 * t168;
        let t171 = 2.0 * t150 * t169;
        let t173 = t10 * t12 * t15;
        let t175 = 1.0 + 0.53425e-1 * t173;
        let t176 = f64::sqrt(t173);
        let t179 = pow_3_2(t173);
        let t182 = t32 * t11 * t35;
        let t184 = 0.379785e1 * t176 + 0.8969e0 * t173 + 0.204775e0 * t179 + 0.123235e0 * t182;
        let t187 = 1.0 + 0.16081979498692535067e2 / t184;
        let t188 = f64::ln(t187);
        let t191 = piecewise3(t4, t50, 1.0);
        let t194 = (2.0 * t191 - 2.0) * t58;
        let t196 = 1.0 + 0.278125e-1 * t173;
        let t201 = 0.51785e1 * t176 + 0.905775e0 * t173 + 0.1100325e0 * t179 + 0.1241775e0 * t182;
        let t204 = 1.0 + 0.29608749977793437516e2 / t201;
        let t205 = f64::ln(t204);
        let t210 = -0.621814e-1 * t175 * t188 + 0.19751673498613801407e-1 * t194 * t196 * t205 - 2.0 * t93;
        let t212 = param_cab_1;
        let t213 = t212 * param_gamma_ab;
        let t214 = t213 * sigma[ip];
        let t218 = 2.0 * param_gamma_ab * sigma[ip] * t101 + 1.0;
        let t219 = 1.0 / t218;
        let t220 = t101 * t219;
        let t223 = param_cab_2;
        let t224 = param_gamma_ab * param_gamma_ab;
        let t225 = t223 * t224;
        let t226 = t225 * t111;
        let t227 = t218 * t218;
        let t228 = 1.0 / t227;
        let t229 = t117 * t228;
        let t232 = param_cab_3;
        let t233 = t224 * param_gamma_ab;
        let t234 = t232 * t233;
        let t235 = t227 * t218;
        let t236 = 1.0 / t235;
        let t240 = param_cab_4;
        let t241 = t224 * t224;
        let t242 = t240 * t241;
        let t243 = t242 * t138;
        let t244 = t227 * t227;
        let t245 = 1.0 / t244;
        let t246 = t143 * t245;
        let t249 = 32.0 * t234 * t129 * t236 + 2.0 * t214 * t220 + 8.0 * t226 * t229 + 64.0 * t243 * t246 + param_cab_0;
        let t250 = t210 * t249;
        let t251 = param_dss_0;
        let t252 = sigma[ip] * t36;
        let t253 = t252 * t100;
        let t254 = tau[ip] * t36;
        let t256 = 1.0 / t34 / rho[ip];
        let t257 = t254 * t256;
        let t258 = 2.0 * t257;
        let t259 = M_CBRT6;
        let t260 = t259 * t259;
        let t261 = M_PI * M_PI;
        let t262 = pow_1_3(t261);
        let t263 = t262 * t262;
        let t264 = t260 * t263;
        let t265 = 3.0 / 5.0 * t264;
        let t268 = 1.0 + param_alpha_ss * (t253 + t258 - t265);
        let t271 = param_dss_1;
        let t272 = t271 * sigma[ip];
        let t274 = param_dss_2;
        let t275 = t258 - t265;
        let t277 = t272 * t101 + t274 * t275;
        let t278 = t268 * t268;
        let t279 = 1.0 / t278;
        let t281 = param_dss_3;
        let t282 = t281 * t111;
        let t285 = param_dss_4;
        let t286 = t285 * sigma[ip];
        let t289 = param_dss_5;
        let t290 = t275 * t275;
        let t292 = t286 * t101 * t275 + 2.0 * t282 * t117 + t289 * t290;
        let t293 = t278 * t268;
        let t294 = 1.0 / t293;
        let t296 = t251 / t268 + t277 * t279 + t292 * t294;
        let t297 = t93 * t296;
        let t299 = 2.0 * t297 * t156;
        let t300 = param_dab_0;
        let t302 = 4.0 * t257;
        let t303 = 6.0 / 5.0 * t264;
        let t306 = 1.0 + param_alpha_ab * (2.0 * t253 + t302 - t303);
        let t309 = param_dab_1;
        let t310 = t309 * sigma[ip];
        let t313 = param_dab_2;
        let t314 = t302 - t303;
        let t316 = 2.0 * t310 * t101 + t313 * t314;
        let t317 = t306 * t306;
        let t318 = 1.0 / t317;
        let t320 = param_dab_3;
        let t321 = t320 * t111;
        let t324 = param_dab_4;
        let t325 = t324 * sigma[ip];
        let t329 = param_dab_5;
        let t330 = t314 * t314;
        let t332 = 2.0 * t325 * t101 * t314 + 8.0 * t321 * t117 + t329 * t330;
        let t333 = t317 * t306;
        let t334 = 1.0 / t333;
        let t336 = t300 / t306 + t316 * t318 + t332 * t334;
        let t337 = t210 * t336;
        let tzk0 = t171 + t250 + t299 + t337;
        zk[ip] += tzk0;
        let t339 = 1.0 / t14 / rho[ip];
        let t340 = t339 * t16;
        let t341 = t20 * t46;
        let t344 = 0.11073470983333333333e-2 * t13 * t340 * t341;
        let t345 = t42 * t42;
        let t346 = 1.0 / t345;
        let t347 = t24 * t346;
        let t350 = 1.0 / t25 * t7 * t9;
        let t351 = t12 * t339;
        let t352 = t16 * t20;
        let t353 = t351 * t352;
        let t354 = t350 * t353;
        let t356 = t340 * t20;
        let t357 = t13 * t356;
        let t359 = f64::sqrt(t22);
        let t361 = t359 * t7 * t9;
        let t362 = t361 * t353;
        let t364 = t256 * t36;
        let t366 = t33 * t364 * t38;
        let t368 = -0.632975e0 * t354 - 0.29896666666666666667e0 * t357 - 0.1023875e0 * t362 - 0.82156666666666666667e-1 * t366;
        let t369 = 1.0 / t45;
        let t370 = t368 * t369;
        let t372 = 1.0 * t347 * t370;
        let t373 = t20 * t70;
        let t377 = t66 * t66;
        let t378 = 1.0 / t377;
        let t379 = t61 * t378;
        let t384 = -0.1176575e1 * t354 - 0.516475e0 * t357 - 0.2103875e0 * t362 - 0.104195e0 * t366;
        let t385 = 1.0 / t69;
        let t386 = t384 * t385;
        let t389 = t20 * t83;
        let t393 = t79 * t79;
        let t394 = 1.0 / t393;
        let t395 = t74 * t394;
        let t400 = -0.86308333333333333334e0 * t354 - 0.301925e0 * t357 - 0.5501625e-1 * t362 - 0.82785e-1 * t366;
        let t401 = 1.0 / t82;
        let t402 = t400 * t401;
        let t407 = t59 * t10;
        let t408 = t352 * t83;
        let t412 = t59 * t74;
        let t414 = t394 * t400 * t401;
        let t420 = piecewise3(t5, 0.0, t6 * (t344 + t372 + t59 * (0.53237641966666666666e-3 * t13 * t340 * t373 + 1.0 * t379 * t386 - t344 - t372 + 0.18311447306006545054e-3 * t13 * t340 * t389 + 0.5848223622634646207e0 * t395 * t402) - 0.18311447306006545054e-3 * t407 * t351 * t408 - 0.5848223622634646207e0 * t412 * t414) / 2.0);
        let t421 = t420 * t149;
        let t422 = t421 * t169;
        let t425 = 1.0 / t34 / t159;
        let t426 = t36 * t425;
        let t427 = t426 * t105;
        let t430 = t95 * t109;
        let t431 = t430 * t111;
        let t432 = t113 * t98;
        let t434 = 1.0 / t14 / t432;
        let t435 = t16 * t434;
        let t436 = t435 * t119;
        let t441 = t108 * t124;
        let t442 = t127 * rho[ip];
        let t443 = 1.0 / t442;
        let t444 = t126 * t443;
        let t445 = t444 * t131;
        let t450 = t123 * t136;
        let t451 = t450 * t138;
        let t452 = t127 * t159;
        let t454 = 1.0 / t34 / t452;
        let t456 = t454 * t145 * t36;
        let t461 = t136 * param_gamma_ss;
        let t462 = t135 * t461;
        let t463 = t138 * sigma[ip];
        let t464 = t462 * t463;
        let t465 = t127 * t432;
        let t467 = 1.0 / t14 / t465;
        let t468 = t16 * t467;
        let t470 = 1.0 / t144 / t104;
        let t471 = t468 * t470;
        let t474 = -8.0 / 3.0 * t97 * t427 + 16.0 / 3.0 * t431 * t436 - 32.0 / 3.0 * t112 * t436 + 64.0 / 3.0 * t441 * t445 - 32.0 * t125 * t445 + 32.0 * t451 * t456 - 128.0 / 3.0 * t139 * t456 + 256.0 / 3.0 * t464 * t471;
        let t475 = t93 * t474;
        let t476 = t475 * t169;
        let t478 = t150 * sigma[ip];
        let t479 = 1.0 / t98;
        let t480 = t479 * t153;
        let t481 = t480 * t168;
        let t482 = t478 * t481;
        let t484 = t156 * t157;
        let t485 = t150 * t484;
        let t487 = 1.0 / t14 / t113;
        let t488 = t16 * t487;
        let t489 = t163 * t167;
        let t490 = t488 * t489;
        let t491 = t485 * t490;
        let t496 = t184 * t184;
        let t497 = 1.0 / t496;
        let t498 = t175 * t497;
        let t500 = 1.0 / t176 * t7;
        let t501 = t9 * t12;
        let t502 = t501 * t339;
        let t503 = t500 * t502;
        let t505 = t10 * t351;
        let t507 = f64::sqrt(t173);
        let t508 = t507 * t7;
        let t509 = t508 * t502;
        let t512 = t32 * t11 * t256;
        let t514 = -0.632975e0 * t503 - 0.29896666666666666667e0 * t505 - 0.1023875e0 * t509 - 0.82156666666666666667e-1 * t512;
        let t515 = 1.0 / t187;
        let t516 = t514 * t515;
        let t519 = t194 * t7;
        let t524 = t194 * t196;
        let t525 = t201 * t201;
        let t526 = 1.0 / t525;
        let t531 = -0.86308333333333333334e0 * t503 - 0.301925e0 * t505 - 0.5501625e-1 * t509 - 0.82785e-1 * t512;
        let t533 = 1.0 / t204;
        let t534 = t526 * t531 * t533;
        let t538 = 0.11073470983333333333e-2 * t10 * t351 * t188 + 1.0 * t498 * t516 - 0.18311447306006545054e-3 * t519 * t501 * t339 * t205 - 0.5848223622634646207e0 * t524 * t534 - 2.0 * t420;
        let t539 = t538 * t249;
        let t540 = t426 * t219;
        let t543 = t212 * t224;
        let t544 = t543 * t111;
        let t545 = t435 * t228;
        let t550 = t223 * t233;
        let t551 = t444 * t236;
        let t556 = t232 * t241;
        let t557 = t556 * t138;
        let t559 = t454 * t245 * t36;
        let t564 = t241 * param_gamma_ab;
        let t565 = t240 * t564;
        let t566 = t565 * t463;
        let t568 = 1.0 / t244 / t218;
        let t569 = t468 * t568;
        let t572 = -16.0 / 3.0 * t214 * t540 + 64.0 / 3.0 * t544 * t545 - 128.0 / 3.0 * t226 * t545 + 512.0 / 3.0 * t550 * t551 - 256.0 * t234 * t551 + 512.0 * t557 * t559 - 2048.0 / 3.0 * t243 * t559 + 8192.0 / 3.0 * t566 * t569;
        let t573 = t210 * t572;
        let t574 = t420 * t296;
        let t575 = t574 * t156;
        let t577 = t251 * t279;
        let t578 = t252 * t425;
        let t580 = t254 * t100;
        let t582 = -8.0 / 3.0 * t578 - 10.0 / 3.0 * t580;
        let t583 = param_alpha_ss * t582;
        let t587 = t274 * tau[ip];
        let t590 = -8.0 / 3.0 * t272 * t426 - 10.0 / 3.0 * t587 * t101;
        let t592 = t277 * t294;
        let t600 = t117 * tau[ip];
        let t603 = t289 * t275;
        let t606 = -32.0 / 3.0 * t282 * t435 - 8.0 / 3.0 * t286 * t426 * t275 - 20.0 / 3.0 * t286 * t600 - 20.0 / 3.0 * t603 * t580;
        let t608 = t278 * t278;
        let t609 = 1.0 / t608;
        let t610 = t292 * t609;
        let t613 = t590 * t279 + t606 * t294 - t577 * t583 - 2.0 * t592 * t583 - 3.0 * t610 * t583;
        let t614 = t93 * t613;
        let t615 = t614 * t156;
        let t617 = sigma[ip] * t479;
        let t618 = t617 * t153;
        let t619 = t297 * t618;
        let t621 = t538 * t336;
        let t622 = t300 * t318;
        let t625 = -16.0 / 3.0 * t578 - 20.0 / 3.0 * t580;
        let t626 = param_alpha_ab * t625;
        let t630 = t313 * tau[ip];
        let t633 = -16.0 / 3.0 * t310 * t426 - 20.0 / 3.0 * t630 * t101;
        let t635 = t316 * t334;
        let t645 = t329 * t314;
        let t648 = -128.0 / 3.0 * t321 * t435 - 16.0 / 3.0 * t325 * t426 * t314 - 80.0 / 3.0 * t325 * t600 - 40.0 / 3.0 * t645 * t580;
        let t650 = t317 * t317;
        let t651 = 1.0 / t650;
        let t652 = t332 * t651;
        let t655 = t633 * t318 + t648 * t334 - t622 * t626 - 2.0 * t635 * t626 - 3.0 * t652 * t626;
        let t656 = t210 * t655;
        let t657 = 2.0 * t422 + 2.0 * t476 + t482 / 4.0 - 160.0 / 3.0 * t491 + t539 + t573 + 2.0 * t575 + 2.0 * t615 + t619 / 4.0 + t621 + t656;
        let tvrho0 = rho[ip] * t657 + t171 + t250 + t299 + t337;
        vrho[ip] += tvrho0;
        let t663 = t110 * sigma[ip];
        let t666 = t111 * t128;
        let t667 = t666 * t131;
        let t672 = t450 * t126;
        let t675 = t137 * t126;
        let t678 = t462 * t138;
        let t679 = t127 * t114;
        let t681 = 1.0 / t14 / t679;
        let t682 = t16 * t681;
        let t683 = t682 * t470;
        let t686 = -2.0 * t430 * sigma[ip] * t120 + t96 * t106 + 4.0 * t663 * t120 + 12.0 * t125 * t667 - 12.0 * t672 * t146 + 16.0 * t675 * t146 - 8.0 * t441 * t667 - 32.0 * t678 * t683;
        let t687 = t93 * t686;
        let t689 = 2.0 * t687 * t169;
        let t690 = t151 * t153;
        let t691 = t690 * t168;
        let t693 = t150 * t691 / 4.0;
        let t699 = t225 * sigma[ip];
        let t702 = t666 * t236;
        let t707 = t556 * t126;
        let t710 = t242 * t126;
        let t713 = t565 * t138;
        let t714 = t682 * t568;
        let t717 = -8.0 * t543 * sigma[ip] * t229 + 2.0 * t213 * t220 + 16.0 * t699 * t229 + 96.0 * t234 * t702 - 192.0 * t707 * t246 + 256.0 * t710 * t246 - 64.0 * t550 * t702 - 1024.0 * t713 * t714;
        let t718 = t210 * t717;
        let t719 = param_alpha_ss * t36;
        let t720 = t719 * t100;
        let t721 = t577 * t720;
        let t722 = t271 * t36;
        let t723 = t100 * t279;
        let t725 = t592 * t720;
        let t727 = t281 * sigma[ip];
        let t730 = t285 * t36;
        let t733 = t730 * t100 * t275 + 4.0 * t727 * t117;
        let t735 = t610 * t720;
        let t737 = t733 * t294 + t722 * t723 - t721 - 2.0 * t725 - 3.0 * t735;
        let t738 = t93 * t737;
        let t740 = 2.0 * t738 * t156;
        let t742 = t297 * t690 / 4.0;
        let t743 = param_alpha_ab * t36;
        let t744 = t743 * t100;
        let t745 = t622 * t744;
        let t747 = t309 * t36;
        let t748 = t100 * t318;
        let t751 = t635 * t744;
        let t753 = t320 * sigma[ip];
        let t756 = t324 * t36;
        let t760 = 2.0 * t756 * t100 * t314 + 16.0 * t753 * t117;
        let t762 = t652 * t744;
        let t764 = t760 * t334 + 2.0 * t747 * t748 - 2.0 * t745 - 4.0 * t751 - 6.0 * t762;
        let t765 = t210 * t764;
        let tvsigma0 = rho[ip] * (t689 - t693 + t718 + t740 - t742 + t765);
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t767 = 1.0 / t157;
        let t768 = t151 * t767;
        let t769 = t768 * t168;
        let t771 = t478 * t769 / 4.0;
        let t772 = t156 * tau[ip];
        let t773 = t150 * t772;
        let t774 = t16 * t161;
        let t775 = t774 * t489;
        let t777 = 32.0 * t773 * t775;
        let t778 = t719 * t256;
        let t781 = t274 * t36;
        let t790 = 4.0 * t286 * t488 + 4.0 * t603 * t364;
        let t794 = 2.0 * t781 * t256 * t279 + t790 * t294 - 2.0 * t577 * t778 - 4.0 * t592 * t778 - 6.0 * t610 * t778;
        let t795 = t93 * t794;
        let t797 = 2.0 * t795 * t156;
        let t798 = t152 * t767;
        let t800 = t297 * t798 / 4.0;
        let t801 = t743 * t256;
        let t804 = t313 * t36;
        let t814 = 16.0 * t325 * t488 + 8.0 * t645 * t364;
        let t818 = 4.0 * t804 * t256 * t318 + t814 * t334 - 4.0 * t622 * t801 - 8.0 * t635 * t801 - 12.0 * t652 * t801;
        let t819 = t210 * t818;
        let tvtau0 = rho[ip] * (t771 + t777 + t797 + t800 + t819);
        vtau[ip] += tvtau0;
    }
}
