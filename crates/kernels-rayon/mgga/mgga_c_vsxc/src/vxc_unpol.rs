//! MGGA_C_VSXC vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_vsxc.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_vsxc_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_alpha_ab: f64,
    param_alpha_ss: f64,
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
        let t94 = param_dss_0;
        let t95 = sigma[ip] * t36;
        let t96 = rho[ip] * rho[ip];
        let t98 = 1.0 / t34 / t96;
        let t99 = t95 * t98;
        let t100 = tau[ip] * t36;
        let t102 = 1.0 / t34 / rho[ip];
        let t103 = t100 * t102;
        let t104 = 2.0 * t103;
        let t105 = M_CBRT6;
        let t106 = t105 * t105;
        let t107 = M_PI * M_PI;
        let t108 = pow_1_3(t107);
        let t109 = t108 * t108;
        let t110 = t106 * t109;
        let t111 = 3.0 / 5.0 * t110;
        let t114 = 1.0 + param_alpha_ss * (t99 + t104 - t111);
        let t117 = param_dss_1;
        let t118 = t117 * sigma[ip];
        let t119 = t36 * t98;
        let t121 = param_dss_2;
        let t122 = t104 - t111;
        let t124 = t118 * t119 + t121 * t122;
        let t125 = t114 * t114;
        let t126 = 1.0 / t125;
        let t128 = param_dss_3;
        let t129 = sigma[ip] * sigma[ip];
        let t130 = t128 * t129;
        let t131 = t96 * t96;
        let t132 = t131 * rho[ip];
        let t134 = 1.0 / t14 / t132;
        let t135 = t16 * t134;
        let t138 = param_dss_4;
        let t139 = t138 * sigma[ip];
        let t142 = param_dss_5;
        let t143 = t122 * t122;
        let t145 = t119 * t122 * t139 + 2.0 * t130 * t135 + t142 * t143;
        let t146 = t125 * t114;
        let t147 = 1.0 / t146;
        let t149 = t94 / t114 + t124 * t126 + t145 * t147;
        let t150 = t93 * t149;
        let t151 = 1.0 / rho[ip];
        let t152 = sigma[ip] * t151;
        let t153 = 1.0 / tau[ip];
        let t156 = 1.0 - t152 * t153 / 8.0;
        let t158 = 2.0 * t150 * t156;
        let t160 = t10 * t12 * t15;
        let t162 = 1.0 + 0.53425e-1 * t160;
        let t163 = f64::sqrt(t160);
        let t166 = pow_3_2(t160);
        let t169 = t32 * t11 * t35;
        let t171 = 0.379785e1 * t163 + 0.8969e0 * t160 + 0.204775e0 * t166 + 0.123235e0 * t169;
        let t174 = 1.0 + 0.16081979498692535067e2 / t171;
        let t175 = f64::ln(t174);
        let t178 = piecewise3(t4, t50, 1.0);
        let t181 = (2.0 * t178 - 2.0) * t58;
        let t183 = 1.0 + 0.278125e-1 * t160;
        let t188 = 0.51785e1 * t163 + 0.905775e0 * t160 + 0.1100325e0 * t166 + 0.1241775e0 * t169;
        let t191 = 1.0 + 0.29608749977793437516e2 / t188;
        let t192 = f64::ln(t191);
        let t197 = -0.621814e-1 * t162 * t175 + 0.19751673498613801407e-1 * t181 * t183 * t192 - 2.0 * t93;
        let t198 = param_dab_0;
        let t200 = 4.0 * t103;
        let t201 = 6.0 / 5.0 * t110;
        let t204 = 1.0 + param_alpha_ab * (2.0 * t99 + t200 - t201);
        let t207 = param_dab_1;
        let t208 = t207 * sigma[ip];
        let t211 = param_dab_2;
        let t212 = t200 - t201;
        let t214 = 2.0 * t119 * t208 + t211 * t212;
        let t215 = t204 * t204;
        let t216 = 1.0 / t215;
        let t218 = param_dab_3;
        let t219 = t218 * t129;
        let t222 = param_dab_4;
        let t223 = t222 * sigma[ip];
        let t227 = param_dab_5;
        let t228 = t212 * t212;
        let t230 = 2.0 * t119 * t212 * t223 + 8.0 * t135 * t219 + t227 * t228;
        let t231 = t215 * t204;
        let t232 = 1.0 / t231;
        let t234 = t198 / t204 + t214 * t216 + t230 * t232;
        let t235 = t197 * t234;
        let tzk0 = t158 + t235;
        zk[ip] += tzk0;
        let t237 = 1.0 / t14 / rho[ip];
        let t238 = t237 * t16;
        let t239 = t20 * t46;
        let t242 = 0.11073470983333333333e-2 * t13 * t238 * t239;
        let t243 = t42 * t42;
        let t244 = 1.0 / t243;
        let t245 = t24 * t244;
        let t248 = 1.0 / t25 * t7 * t9;
        let t249 = t12 * t237;
        let t250 = t16 * t20;
        let t251 = t249 * t250;
        let t252 = t248 * t251;
        let t254 = t238 * t20;
        let t255 = t13 * t254;
        let t257 = f64::sqrt(t22);
        let t259 = t257 * t7 * t9;
        let t260 = t259 * t251;
        let t262 = t102 * t36;
        let t264 = t33 * t262 * t38;
        let t266 = -0.632975e0 * t252 - 0.29896666666666666667e0 * t255 - 0.1023875e0 * t260 - 0.82156666666666666667e-1 * t264;
        let t267 = 1.0 / t45;
        let t268 = t266 * t267;
        let t270 = 1.0 * t245 * t268;
        let t271 = t20 * t70;
        let t275 = t66 * t66;
        let t276 = 1.0 / t275;
        let t277 = t61 * t276;
        let t282 = -0.1176575e1 * t252 - 0.516475e0 * t255 - 0.2103875e0 * t260 - 0.104195e0 * t264;
        let t283 = 1.0 / t69;
        let t284 = t282 * t283;
        let t287 = t20 * t83;
        let t291 = t79 * t79;
        let t292 = 1.0 / t291;
        let t293 = t74 * t292;
        let t298 = -0.86308333333333333334e0 * t252 - 0.301925e0 * t255 - 0.5501625e-1 * t260 - 0.82785e-1 * t264;
        let t299 = 1.0 / t82;
        let t300 = t298 * t299;
        let t305 = t59 * t10;
        let t306 = t250 * t83;
        let t310 = t59 * t74;
        let t312 = t292 * t298 * t299;
        let t318 = piecewise3(t5, 0.0, t6 * (t242 + t270 + t59 * (0.53237641966666666666e-3 * t13 * t238 * t271 + 1.0 * t277 * t284 - t242 - t270 + 0.18311447306006545054e-3 * t13 * t238 * t287 + 0.5848223622634646207e0 * t293 * t300) - 0.18311447306006545054e-3 * t305 * t249 * t306 - 0.5848223622634646207e0 * t310 * t312) / 2.0);
        let t319 = t318 * t149;
        let t320 = t319 * t156;
        let t322 = t94 * t126;
        let t323 = t96 * rho[ip];
        let t325 = 1.0 / t34 / t323;
        let t326 = t95 * t325;
        let t328 = t100 * t98;
        let t330 = -8.0 / 3.0 * t326 - 10.0 / 3.0 * t328;
        let t331 = param_alpha_ss * t330;
        let t333 = t36 * t325;
        let t336 = t121 * tau[ip];
        let t339 = -8.0 / 3.0 * t118 * t333 - 10.0 / 3.0 * t336 * t119;
        let t341 = t124 * t147;
        let t344 = t131 * t96;
        let t346 = 1.0 / t14 / t344;
        let t347 = t16 * t346;
        let t353 = t135 * tau[ip];
        let t356 = t142 * t122;
        let t359 = -32.0 / 3.0 * t130 * t347 - 8.0 / 3.0 * t139 * t333 * t122 - 20.0 / 3.0 * t139 * t353 - 20.0 / 3.0 * t356 * t328;
        let t361 = t125 * t125;
        let t362 = 1.0 / t361;
        let t363 = t145 * t362;
        let t366 = t126 * t339 + t147 * t359 - t322 * t331 - 2.0 * t331 * t341 - 3.0 * t331 * t363;
        let t367 = t93 * t366;
        let t368 = t367 * t156;
        let t370 = 1.0 / t96;
        let t371 = sigma[ip] * t370;
        let t372 = t371 * t153;
        let t373 = t150 * t372;
        let t378 = t171 * t171;
        let t379 = 1.0 / t378;
        let t380 = t162 * t379;
        let t382 = 1.0 / t163 * t7;
        let t383 = t9 * t12;
        let t384 = t383 * t237;
        let t385 = t382 * t384;
        let t387 = t10 * t249;
        let t389 = f64::sqrt(t160);
        let t390 = t389 * t7;
        let t391 = t390 * t384;
        let t394 = t32 * t11 * t102;
        let t396 = -0.632975e0 * t385 - 0.29896666666666666667e0 * t387 - 0.1023875e0 * t391 - 0.82156666666666666667e-1 * t394;
        let t397 = 1.0 / t174;
        let t398 = t396 * t397;
        let t401 = t181 * t7;
        let t406 = t181 * t183;
        let t407 = t188 * t188;
        let t408 = 1.0 / t407;
        let t413 = -0.86308333333333333334e0 * t385 - 0.301925e0 * t387 - 0.5501625e-1 * t391 - 0.82785e-1 * t394;
        let t415 = 1.0 / t191;
        let t416 = t408 * t413 * t415;
        let t420 = 0.11073470983333333333e-2 * t10 * t249 * t175 + 1.0 * t380 * t398 - 0.18311447306006545054e-3 * t401 * t383 * t237 * t192 - 0.5848223622634646207e0 * t406 * t416 - 2.0 * t318;
        let t421 = t420 * t234;
        let t422 = t198 * t216;
        let t425 = -16.0 / 3.0 * t326 - 20.0 / 3.0 * t328;
        let t426 = param_alpha_ab * t425;
        let t430 = t211 * tau[ip];
        let t433 = -16.0 / 3.0 * t208 * t333 - 20.0 / 3.0 * t430 * t119;
        let t435 = t214 * t232;
        let t445 = t227 * t212;
        let t448 = -128.0 / 3.0 * t219 * t347 - 16.0 / 3.0 * t223 * t333 * t212 - 80.0 / 3.0 * t223 * t353 - 40.0 / 3.0 * t445 * t328;
        let t450 = t215 * t215;
        let t451 = 1.0 / t450;
        let t452 = t230 * t451;
        let t455 = t216 * t433 + t232 * t448 - t422 * t426 - 2.0 * t426 * t435 - 3.0 * t426 * t452;
        let t456 = t197 * t455;
        let tvrho0 = t158 + t235 + rho[ip] * (2.0 * t320 + 2.0 * t368 + t373 / 4.0 + t421 + t456);
        vrho[ip] += tvrho0;
        let t459 = param_alpha_ss * t36;
        let t460 = t459 * t98;
        let t461 = t322 * t460;
        let t462 = t117 * t36;
        let t463 = t98 * t126;
        let t465 = t341 * t460;
        let t467 = t128 * sigma[ip];
        let t470 = t138 * t36;
        let t473 = t122 * t470 * t98 + 4.0 * t135 * t467;
        let t475 = t363 * t460;
        let t477 = t147 * t473 + t462 * t463 - t461 - 2.0 * t465 - 3.0 * t475;
        let t478 = t93 * t477;
        let t480 = 2.0 * t478 * t156;
        let t481 = t151 * t153;
        let t483 = t150 * t481 / 4.0;
        let t484 = param_alpha_ab * t36;
        let t485 = t484 * t98;
        let t486 = t422 * t485;
        let t488 = t207 * t36;
        let t489 = t98 * t216;
        let t492 = t435 * t485;
        let t494 = t218 * sigma[ip];
        let t497 = t222 * t36;
        let t501 = 2.0 * t212 * t497 * t98 + 16.0 * t135 * t494;
        let t503 = t452 * t485;
        let t505 = t232 * t501 + 2.0 * t488 * t489 - 2.0 * t486 - 4.0 * t492 - 6.0 * t503;
        let t506 = t197 * t505;
        let tvsigma0 = rho[ip] * (t480 - t483 + t506);
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t508 = t459 * t102;
        let t511 = t121 * t36;
        let t518 = 1.0 / t14 / t131;
        let t519 = t16 * t518;
        let t523 = 4.0 * t139 * t519 + 4.0 * t262 * t356;
        let t527 = 2.0 * t102 * t126 * t511 + t147 * t523 - 2.0 * t322 * t508 - 4.0 * t341 * t508 - 6.0 * t363 * t508;
        let t528 = t93 * t527;
        let t530 = 2.0 * t528 * t156;
        let t531 = tau[ip] * tau[ip];
        let t532 = 1.0 / t531;
        let t533 = t152 * t532;
        let t535 = t150 * t533 / 4.0;
        let t536 = t484 * t102;
        let t539 = t211 * t36;
        let t549 = 16.0 * t223 * t519 + 8.0 * t262 * t445;
        let t553 = 4.0 * t102 * t216 * t539 + t232 * t549 - 4.0 * t422 * t536 - 8.0 * t435 * t536 - 12.0 * t452 * t536;
        let t554 = t197 * t553;
        let tvtau0 = rho[ip] * (t530 + t535 + t554);
        vtau[ip] += tvtau0;
    }
}
