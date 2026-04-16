//! MGGA_C_M06L exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 295 shared lines across all orders.
//! Delta: 295 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_c_m06l_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (295 lines) ---
        let t3 = rho0 - rho1;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
        let t7 = 1.0 + t6;
        let t8 = t7 <= zeta_threshold;
        let t9 = rho0 <= dens_threshold || t8;
        let t10 = piecewise3(t8, zeta_threshold, t7);
        let t11 = M_CBRT3;
        let t12 = 1.0 / M_PI;
        let t13 = pow_1_3(t12);
        let t14 = t11 * t13;
        let t15 = M_CBRT4;
        let t16 = t15 * t15;
        let t17 = t14 * t16;
        let t18 = pow_1_3(t4);
        let t19 = 1.0 / t18;
        let t20 = M_CBRT2;
        let t21 = t19 * t20;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = 1.0 / t22;
        let t24 = pow_1_3(t7);
        let t26 = piecewise3(t8, t23, 1.0 / t24);
        let t28 = t17 * t21 * t26;
        let t30 = 1.0 + 0.53425e-1 * t28;
        let t31 = f64::sqrt(t28);
        let t34 = pow_3_2(t28);
        let t36 = t11 * t11;
        let t37 = t13 * t13;
        let t38 = t36 * t37;
        let t39 = t38 * t15;
        let t40 = t18 * t18;
        let t41 = 1.0 / t40;
        let t42 = t20 * t20;
        let t43 = t41 * t42;
        let t44 = t26 * t26;
        let t46 = t39 * t43 * t44;
        let t48 = 0.379785e1 * t31 + 0.8969e0 * t28 + 0.204775e0 * t34 + 0.123235e0 * t46;
        let t51 = 1.0 + 0.16081979498692535067e2 / t48;
        let t52 = f64::ln(t51);
        let t54 = 0.621814e-1 * t30 * t52;
        let t56 = t22 * zeta_threshold;
        let t58 = piecewise3(2.0 <= zeta_threshold, t56, 2.0 * t20);
        let t60 = piecewise3(0.0 <= zeta_threshold, t56, 0.0);
        let t64 = 1.0 / (2.0 * t20 - 2.0);
        let t65 = (t58 + t60 - 2.0) * t64;
        let t67 = 1.0 + 0.5137e-1 * t28;
        let t72 = 0.705945e1 * t31 + 0.1549425e1 * t28 + 0.420775e0 * t34 + 0.1562925e0 * t46;
        let t75 = 1.0 + 0.32163958997385070134e2 / t72;
        let t76 = f64::ln(t75);
        let t80 = 1.0 + 0.278125e-1 * t28;
        let t85 = 0.51785e1 * t31 + 0.905775e0 * t28 + 0.1100325e0 * t34 + 0.1241775e0 * t46;
        let t88 = 1.0 + 0.29608749977793437516e2 / t85;
        let t89 = f64::ln(t88);
        let t90 = t80 * t89;
        let t96 = -t54 + t65 * (-0.310907e-1 * t67 * t76 + t54 - 0.19751673498613801407e-1 * t90) + 0.19751673498613801407e-1 * t65 * t90;
        let t99 = piecewise3(t9, 0.0, t10 * t96 / 2.0);
        let t100 = param_css_0;
        let t101 = param_css_1;
        let t102 = t101 * param_gamma_ss;
        let t103 = rho0 * rho0;
        let t104 = pow_1_3(rho0);
        let t105 = t104 * t104;
        let t107 = 1.0 / t105 / t103;
        let t108 = sigma0 * t107;
        let t111 = param_gamma_ss * sigma0 * t107 + 1.0;
        let t112 = 1.0 / t111;
        let t115 = param_css_2;
        let t116 = param_gamma_ss * param_gamma_ss;
        let t117 = t115 * t116;
        let t118 = sigma0 * sigma0;
        let t119 = t103 * t103;
        let t120 = t119 * rho0;
        let t122 = 1.0 / t104 / t120;
        let t124 = t111 * t111;
        let t125 = 1.0 / t124;
        let t128 = param_css_3;
        let t129 = t116 * param_gamma_ss;
        let t130 = t128 * t129;
        let t131 = t118 * sigma0;
        let t132 = t119 * t119;
        let t133 = 1.0 / t132;
        let t135 = t124 * t111;
        let t136 = 1.0 / t135;
        let t139 = param_css_4;
        let t140 = t116 * t116;
        let t141 = t139 * t140;
        let t142 = t118 * t118;
        let t143 = t132 * t103;
        let t145 = 1.0 / t105 / t143;
        let t147 = t124 * t124;
        let t148 = 1.0 / t147;
        let t151 = t117 * t118 * t122 * t125 + t130 * t131 * t133 * t136 + t141 * t142 * t145 * t148 + t102 * t108 * t112 + t100;
        let t152 = t99 * t151;
        let t153 = 1.0 / rho0;
        let t154 = sigma0 * t153;
        let t155 = 1.0 / tau0;
        let t158 = 1.0 - t154 * t155 / 8.0;
        let t159 = tau0 * tau0;
        let t160 = t103 * rho0;
        let t162 = 1.0 / t104 / t160;
        let t164 = param_Fermi_D_cnst * param_Fermi_D_cnst;
        let t165 = 1.0 / t164;
        let t168 = f64::exp(-4.0 * t159 * t162 * t165);
        let t169 = 1.0 - t168;
        let t170 = t158 * t169;
        let t171 = t152 * t170;
        let t173 = 1.0 - t6;
        let t174 = t173 <= zeta_threshold;
        let t175 = rho1 <= dens_threshold || t174;
        let t176 = piecewise3(t174, zeta_threshold, t173);
        let t177 = pow_1_3(t173);
        let t179 = piecewise3(t174, t23, 1.0 / t177);
        let t181 = t17 * t21 * t179;
        let t183 = 1.0 + 0.53425e-1 * t181;
        let t184 = f64::sqrt(t181);
        let t187 = pow_3_2(t181);
        let t189 = t179 * t179;
        let t191 = t39 * t43 * t189;
        let t193 = 0.379785e1 * t184 + 0.8969e0 * t181 + 0.204775e0 * t187 + 0.123235e0 * t191;
        let t196 = 1.0 + 0.16081979498692535067e2 / t193;
        let t197 = f64::ln(t196);
        let t199 = 0.621814e-1 * t183 * t197;
        let t201 = 1.0 + 0.5137e-1 * t181;
        let t206 = 0.705945e1 * t184 + 0.1549425e1 * t181 + 0.420775e0 * t187 + 0.1562925e0 * t191;
        let t209 = 1.0 + 0.32163958997385070134e2 / t206;
        let t210 = f64::ln(t209);
        let t214 = 1.0 + 0.278125e-1 * t181;
        let t219 = 0.51785e1 * t184 + 0.905775e0 * t181 + 0.1100325e0 * t187 + 0.1241775e0 * t191;
        let t222 = 1.0 + 0.29608749977793437516e2 / t219;
        let t223 = f64::ln(t222);
        let t224 = t214 * t223;
        let t230 = -t199 + t65 * (-0.310907e-1 * t201 * t210 + t199 - 0.19751673498613801407e-1 * t224) + 0.19751673498613801407e-1 * t65 * t224;
        let t233 = piecewise3(t175, 0.0, t176 * t230 / 2.0);
        let t234 = rho1 * rho1;
        let t235 = pow_1_3(rho1);
        let t236 = t235 * t235;
        let t238 = 1.0 / t236 / t234;
        let t239 = sigma2 * t238;
        let t242 = param_gamma_ss * sigma2 * t238 + 1.0;
        let t243 = 1.0 / t242;
        let t246 = sigma2 * sigma2;
        let t247 = t234 * t234;
        let t248 = t247 * rho1;
        let t250 = 1.0 / t235 / t248;
        let t252 = t242 * t242;
        let t253 = 1.0 / t252;
        let t256 = t246 * sigma2;
        let t257 = t247 * t247;
        let t258 = 1.0 / t257;
        let t260 = t252 * t242;
        let t261 = 1.0 / t260;
        let t264 = t246 * t246;
        let t265 = t257 * t234;
        let t267 = 1.0 / t236 / t265;
        let t269 = t252 * t252;
        let t270 = 1.0 / t269;
        let t273 = t117 * t246 * t250 * t253 + t130 * t256 * t258 * t261 + t141 * t264 * t267 * t270 + t102 * t239 * t243 + t100;
        let t274 = t233 * t273;
        let t275 = 1.0 / rho1;
        let t276 = sigma2 * t275;
        let t277 = 1.0 / tau1;
        let t280 = 1.0 - t276 * t277 / 8.0;
        let t281 = tau1 * tau1;
        let t282 = t234 * rho1;
        let t284 = 1.0 / t235 / t282;
        let t288 = f64::exp(-4.0 * t281 * t284 * t165);
        let t289 = 1.0 - t288;
        let t290 = t280 * t289;
        let t291 = t274 * t290;
        let t293 = t14 * t16 * t19;
        let t295 = 1.0 + 0.53425e-1 * t293;
        let t296 = f64::sqrt(t293);
        let t299 = pow_3_2(t293);
        let t302 = t38 * t15 * t41;
        let t304 = 0.379785e1 * t296 + 0.8969e0 * t293 + 0.204775e0 * t299 + 0.123235e0 * t302;
        let t307 = 1.0 + 0.16081979498692535067e2 / t304;
        let t308 = f64::ln(t307);
        let t310 = 0.621814e-1 * t295 * t308;
        let t311 = t3 * t3;
        let t312 = t311 * t311;
        let t313 = t4 * t4;
        let t314 = t313 * t313;
        let t315 = 1.0 / t314;
        let t316 = t312 * t315;
        let t317 = t24 * t7;
        let t318 = piecewise3(t8, t56, t317);
        let t319 = t177 * t173;
        let t320 = piecewise3(t174, t56, t319);
        let t321 = t318 + t320 - 2.0;
        let t322 = t321 * t64;
        let t324 = 1.0 + 0.5137e-1 * t293;
        let t329 = 0.705945e1 * t296 + 0.1549425e1 * t293 + 0.420775e0 * t299 + 0.1562925e0 * t302;
        let t332 = 1.0 + 0.32163958997385070134e2 / t329;
        let t333 = f64::ln(t332);
        let t337 = 1.0 + 0.278125e-1 * t293;
        let t342 = 0.51785e1 * t296 + 0.905775e0 * t293 + 0.1100325e0 * t299 + 0.1241775e0 * t302;
        let t345 = 1.0 + 0.29608749977793437516e2 / t342;
        let t346 = f64::ln(t345);
        let t347 = t337 * t346;
        let t349 = -0.310907e-1 * t324 * t333 + t310 - 0.19751673498613801407e-1 * t347;
        let t350 = t322 * t349;
        let t354 = -t310 + t316 * t350 + 0.19751673498613801407e-1 * t322 * t347 - t99 - t233;
        let t356 = param_cab_1;
        let t357 = t356 * param_gamma_ab;
        let t358 = t108 + t239;
        let t360 = param_gamma_ab * t358 + 1.0;
        let t361 = 1.0 / t360;
        let t364 = param_cab_2;
        let t365 = param_gamma_ab * param_gamma_ab;
        let t366 = t364 * t365;
        let t367 = t358 * t358;
        let t368 = t360 * t360;
        let t369 = 1.0 / t368;
        let t372 = param_cab_3;
        let t373 = t365 * param_gamma_ab;
        let t374 = t372 * t373;
        let t375 = t367 * t358;
        let t376 = t368 * t360;
        let t377 = 1.0 / t376;
        let t380 = param_cab_4;
        let t381 = t365 * t365;
        let t382 = t380 * t381;
        let t383 = t367 * t367;
        let t384 = t368 * t368;
        let t385 = 1.0 / t384;
        let t388 = t357 * t358 * t361 + t366 * t367 * t369 + t374 * t375 * t377 + t382 * t383 * t385 + param_cab_0;
        let t389 = t354 * t388;
        let t390 = param_dss_0;
        let t392 = 1.0 / t105 / rho0;
        let t394 = 2.0 * tau0 * t392;
        let t395 = M_CBRT6;
        let t396 = t395 * t395;
        let t397 = M_PI * M_PI;
        let t398 = pow_1_3(t397);
        let t399 = t398 * t398;
        let t400 = t396 * t399;
        let t401 = 3.0 / 5.0 * t400;
        let t404 = 1.0 + param_alpha_ss * (t108 + t394 - t401);
        let t407 = param_dss_1;
        let t408 = t407 * sigma0;
        let t410 = param_dss_2;
        let t411 = t394 - t401;
        let t413 = t408 * t107 + t410 * t411;
        let t414 = t404 * t404;
        let t415 = 1.0 / t414;
        let t417 = param_dss_3;
        let t418 = t417 * t118;
        let t420 = param_dss_4;
        let t421 = t420 * sigma0;
        let t424 = param_dss_5;
        let t425 = t411 * t411;
        let t427 = t421 * t107 * t411 + t418 * t122 + t424 * t425;
        let t428 = t414 * t404;
        let t429 = 1.0 / t428;
        let t431 = t390 / t404 + t413 * t415 + t427 * t429;
        let t432 = t99 * t431;
        let t433 = t432 * t158;
        let t435 = 1.0 / t236 / rho1;
        let t437 = 2.0 * tau1 * t435;
        let t440 = 1.0 + param_alpha_ss * (t239 + t437 - t401);
        let t443 = t407 * sigma2;
        let t445 = t437 - t401;
        let t447 = t443 * t238 + t410 * t445;
        let t448 = t440 * t440;
        let t449 = 1.0 / t448;
        let t451 = t417 * t246;
        let t453 = t420 * sigma2;
        let t456 = t445 * t445;
        let t458 = t453 * t238 * t445 + t451 * t250 + t424 * t456;
        let t459 = t448 * t440;
        let t460 = 1.0 / t459;
        let t462 = t390 / t440 + t447 * t449 + t458 * t460;
        let t463 = t233 * t462;
        let t464 = t463 * t280;
        let t465 = param_dab_0;
        let t466 = 6.0 / 5.0 * t400;
        let t469 = 1.0 + param_alpha_ab * (t108 + t239 + t394 + t437 - t466);
        let t472 = param_dab_1;
        let t474 = param_dab_2;
        let t475 = t394 + t437 - t466;
        let t477 = t472 * t358 + t474 * t475;
        let t478 = t469 * t469;
        let t479 = 1.0 / t478;
        let t481 = param_dab_3;
        let t483 = param_dab_4;
        let t484 = t483 * t358;
        let t486 = param_dab_5;
        let t487 = t475 * t475;
        let t489 = t481 * t367 + t484 * t475 + t486 * t487;
        let t490 = t478 * t469;
        let t491 = 1.0 / t490;
        let t493 = t465 / t469 + t477 * t479 + t489 * t491;
        let t494 = t354 * t493;
        let tzk0 = t171 + t291 + t389 + t433 + t464 + t494;
        zk[ip] += tzk0;
    }
}
