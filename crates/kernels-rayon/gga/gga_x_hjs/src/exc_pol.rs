//! GGA_X_HJS exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hjs.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_hjs_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t17 = t16 * t7;
        let t18 = piecewise5(t10, t11, t14, t15, t17);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = t2 * t2;
        let t29 = param_hyb_omega_0 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t35 = 1.0 + t17 <= zeta_threshold;
        let t37 = 1.0 - t17 <= zeta_threshold;
        let t38 = piecewise5(t35, t11, t37, t15, t17);
        let t39 = 1.0 + t38;
        let t40 = t39 <= zeta_threshold;
        let t41 = pow_1_3(t39);
        let t42 = piecewise3(t40, t21, t41);
        let t43 = 1.0 / t42;
        let t44 = 1.0 / t26;
        let t45 = t43 * t44;
        let t46 = M_CBRT6;
        let t47 = t31 * t31;
        let t48 = 1.0 / t47;
        let t49 = t46 * t48;
        let t50 = t49 * sigma0;
        let t51 = rho0 * rho0;
        let t52 = pow_1_3(rho0);
        let t53 = t52 * t52;
        let t55 = 1.0 / t53 / t51;
        let t57 = param_a_0 * t46;
        let t58 = t48 * sigma0;
        let t59 = t58 * t55;
        let t63 = 1.0 / t30;
        let t64 = param_a_1 * t63;
        let t65 = rmath::sqrt(sigma0);
        let t66 = t65 * sigma0;
        let t67 = t51 * t51;
        let t68 = 1.0 / t67;
        let t69 = t66 * t68;
        let t73 = t46 * t46;
        let t74 = param_a_2 * t73;
        let t76 = 1.0 / t31 / t30;
        let t77 = sigma0 * sigma0;
        let t78 = t76 * t77;
        let t79 = t67 * rho0;
        let t81 = 1.0 / t52 / t79;
        let t82 = t78 * t81;
        let t86 = param_a_3 * t46;
        let t88 = 1.0 / t47 / t30;
        let t89 = t65 * t77;
        let t90 = t88 * t89;
        let t91 = t67 * t51;
        let t93 = 1.0 / t53 / t91;
        let t94 = t90 * t93;
        let t98 = t30 * t30;
        let t99 = 1.0 / t98;
        let t100 = param_a_4 * t99;
        let t101 = t77 * sigma0;
        let t102 = t67 * t67;
        let t103 = 1.0 / t102;
        let t104 = t101 * t103;
        let t108 = param_a_5 * t73;
        let t110 = 1.0 / t31 / t98;
        let t111 = t65 * t101;
        let t112 = t110 * t111;
        let t113 = t102 * rho0;
        let t115 = 1.0 / t52 / t113;
        let t116 = t112 * t115;
        let t119 = t57 * t59 / 24.0 + t64 * t69 / 48.0 + t74 * t82 / 576.0 + t86 * t94 / 1152.0 + t100 * t104 / 2304.0 + t108 * t116 / 27648.0;
        let t120 = t55 * t119;
        let t122 = param_b_0 * t73;
        let t123 = t32 * t65;
        let t125 = 1.0 / t52 / rho0;
        let t130 = param_b_1 * t46;
        let t134 = param_b_2 * t63;
        let t138 = param_b_3 * t73;
        let t142 = param_b_4 * t46;
        let t146 = param_b_5 * t99;
        let t150 = param_b_6 * t73;
        let t154 = param_b_7 * t46;
        let t156 = 1.0 / t47 / t98;
        let t157 = t77 * t77;
        let t158 = t156 * t157;
        let t159 = t102 * t51;
        let t161 = 1.0 / t53 / t159;
        let t168 = param_b_8 / t98 / t30;
        let t169 = t65 * t157;
        let t170 = t102 * t67;
        let t171 = 1.0 / t170;
        let t175 = 1.0 + t122 * t123 * t125 / 12.0 + t130 * t59 / 24.0 + t134 * t69 / 48.0 + t138 * t82 / 576.0 + t142 * t94 / 1152.0 + t146 * t104 / 2304.0 + t150 * t116 / 27648.0 + t154 * t158 * t161 / 55296.0 + t168 * t169 * t171 / 110592.0;
        let t176 = 1.0 / t175;
        let t177 = t120 * t176;
        let t179 = t50 * t177 / 24.0;
        let t180 = 1e-10 < t179;
        let t181 = piecewise3(t180, t179, 1e-10);
        let t182 = param_hyb_omega_0 * param_hyb_omega_0;
        let t183 = t182 * t2;
        let t184 = t42 * t42;
        let t185 = 1.0 / t184;
        let t186 = t48 * t185;
        let t187 = t26 * t26;
        let t188 = 1.0 / t187;
        let t190 = t183 * t186 * t188;
        let t192 = 0.60965 + t181 + t190 / 3.0;
        let t193 = rmath::sqrt(t192);
        let t194 = 1.0 / t193;
        let t196 = t33 * t45 * t194;
        let t198 = 1.0 - t196 / 3.0;
        let t199 = 0.60965 + t181;
        let t200 = 1.0 / t199;
        let t203 = sigma0 * t55;
        let t204 = t49 * t203;
        let t206 = 1.0 + t204 / 96.0;
        let t207 = 1.0 / t206;
        let t212 = 1.0 + 0.013006513974354691 * t49 * t203 * t207 + 4.21411052769092 * t181;
        let t214 = t182 * param_hyb_omega_0 * t63;
        let t215 = t184 * t42;
        let t216 = 1.0 / t215;
        let t217 = t216 * t7;
        let t219 = 1.0 / t193 / t192;
        let t221 = t214 * t217 * t219;
        let t223 = 2.0 - t196 + t221 / 3.0;
        let t224 = t212 * t223;
        let t225 = t199 * t199;
        let t226 = 1.0 / t225;
        let t232 = t225 * t199;
        let t234 = rmath::sqrt(t199);
        let t235 = t234 * t232;
        let t236 = rmath::sqrt(M_PI);
        let t237 = 4.0 / 5.0 * t236;
        let t238 = rmath::sqrt(t181);
        let t241 = 0.0 < 0.7572109999 + t181;
        let t243 = piecewise3(t241, 0.757211 + t181, 1e-10);
        let t244 = rmath::sqrt(t243);
        let t246 = t237 + 12.0 / 5.0 * t238 - 12.0 / 5.0 * t244;
        let t248 = 0.0474596 * t212 * t199 + 0.028363733333333332 * t225 - 0.9086532 * t232 - t235 * t246;
        let t251 = t182 * t182;
        let t253 = t251 * param_hyb_omega_0 * t2;
        let t254 = t253 * t88;
        let t255 = t184 * t184;
        let t257 = 1.0 / t255 / t42;
        let t259 = 1.0 / t187 / t6;
        let t260 = t257 * t259;
        let t261 = t192 * t192;
        let t263 = 1.0 / t193 / t261;
        let t267 = 8.0 - 5.0 * t196 + 10.0 / 3.0 * t221 - t254 * t260 * t263 / 3.0;
        let t268 = t248 * t267;
        let t269 = 1.0 / t232;
        let t273 = 3.0 * t190;
        let t274 = 9.0 * t181 + t273;
        let t275 = rmath::sqrt(t274);
        let t277 = 9.0 * t243 + t273;
        let t278 = rmath::sqrt(t277);
        let t280 = t275 / 3.0 - t278 / 3.0;
        let t284 = t32 * t43;
        let t286 = t29 * t284 * t44;
        let t288 = t286 / 3.0 + t275 / 3.0;
        let t290 = t286 / 3.0 + t193;
        let t291 = 1.0 / t290;
        let t293 = rmath::ln(t288 * t291);
        let t297 = t286 / 3.0 + t278 / 3.0;
        let t299 = rmath::ln(t297 * t291);
        let t302 = 0.757211 + 0.04727288888888889 * t198 * t200 + 0.026366444444444446 * t224 * t226 - t268 * t269 / 9.0 + 2.0 / 3.0 * t33 * t45 * t280 + 2.0 * t181 * t293 - 2.0 * t243 * t299;
        let t306 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t302);
        let t307 = rho1 <= dens_threshold;
        let t308 = -t16;
        let t310 = piecewise5(t14, t11, t10, t15, t308 * t7);
        let t311 = 1.0 + t310;
        let t312 = t311 <= zeta_threshold;
        let t313 = pow_1_3(t311);
        let t315 = piecewise3(t312, t22, t313 * t311);
        let t316 = t315 * t26;
        let t317 = piecewise5(t37, t11, t35, t15, -t17);
        let t318 = 1.0 + t317;
        let t319 = t318 <= zeta_threshold;
        let t320 = pow_1_3(t318);
        let t321 = piecewise3(t319, t21, t320);
        let t322 = 1.0 / t321;
        let t323 = t322 * t44;
        let t324 = t49 * sigma2;
        let t325 = rho1 * rho1;
        let t326 = pow_1_3(rho1);
        let t327 = t326 * t326;
        let t329 = 1.0 / t327 / t325;
        let t330 = t48 * sigma2;
        let t331 = t330 * t329;
        let t334 = rmath::sqrt(sigma2);
        let t335 = t334 * sigma2;
        let t336 = t325 * t325;
        let t337 = 1.0 / t336;
        let t338 = t335 * t337;
        let t341 = sigma2 * sigma2;
        let t342 = t76 * t341;
        let t343 = t336 * rho1;
        let t345 = 1.0 / t326 / t343;
        let t346 = t342 * t345;
        let t349 = t334 * t341;
        let t350 = t88 * t349;
        let t351 = t336 * t325;
        let t353 = 1.0 / t327 / t351;
        let t354 = t350 * t353;
        let t357 = t341 * sigma2;
        let t358 = t336 * t336;
        let t359 = 1.0 / t358;
        let t360 = t357 * t359;
        let t363 = t334 * t357;
        let t364 = t110 * t363;
        let t365 = t358 * rho1;
        let t367 = 1.0 / t326 / t365;
        let t368 = t364 * t367;
        let t371 = t57 * t331 / 24.0 + t64 * t338 / 48.0 + t74 * t346 / 576.0 + t86 * t354 / 1152.0 + t100 * t360 / 2304.0 + t108 * t368 / 27648.0;
        let t372 = t329 * t371;
        let t373 = t32 * t334;
        let t375 = 1.0 / t326 / rho1;
        let t391 = t341 * t341;
        let t392 = t156 * t391;
        let t393 = t358 * t325;
        let t395 = 1.0 / t327 / t393;
        let t399 = t334 * t391;
        let t400 = t358 * t336;
        let t401 = 1.0 / t400;
        let t405 = 1.0 + t122 * t373 * t375 / 12.0 + t130 * t331 / 24.0 + t134 * t338 / 48.0 + t138 * t346 / 576.0 + t142 * t354 / 1152.0 + t146 * t360 / 2304.0 + t150 * t368 / 27648.0 + t154 * t392 * t395 / 55296.0 + t168 * t399 * t401 / 110592.0;
        let t406 = 1.0 / t405;
        let t407 = t372 * t406;
        let t409 = t324 * t407 / 24.0;
        let t410 = 1e-10 < t409;
        let t411 = piecewise3(t410, t409, 1e-10);
        let t412 = t321 * t321;
        let t413 = 1.0 / t412;
        let t414 = t48 * t413;
        let t416 = t183 * t414 * t188;
        let t418 = 0.60965 + t411 + t416 / 3.0;
        let t419 = rmath::sqrt(t418);
        let t420 = 1.0 / t419;
        let t422 = t33 * t323 * t420;
        let t424 = 1.0 - t422 / 3.0;
        let t425 = 0.60965 + t411;
        let t426 = 1.0 / t425;
        let t429 = sigma2 * t329;
        let t430 = t49 * t429;
        let t432 = 1.0 + t430 / 96.0;
        let t433 = 1.0 / t432;
        let t438 = 1.0 + 0.013006513974354691 * t49 * t429 * t433 + 4.21411052769092 * t411;
        let t439 = t412 * t321;
        let t440 = 1.0 / t439;
        let t441 = t440 * t7;
        let t443 = 1.0 / t419 / t418;
        let t445 = t214 * t441 * t443;
        let t447 = 2.0 - t422 + t445 / 3.0;
        let t448 = t438 * t447;
        let t449 = t425 * t425;
        let t450 = 1.0 / t449;
        let t456 = t449 * t425;
        let t458 = rmath::sqrt(t425);
        let t459 = t458 * t456;
        let t460 = rmath::sqrt(t411);
        let t463 = 0.0 < 0.7572109999 + t411;
        let t465 = piecewise3(t463, 0.757211 + t411, 1e-10);
        let t466 = rmath::sqrt(t465);
        let t468 = t237 + 12.0 / 5.0 * t460 - 12.0 / 5.0 * t466;
        let t470 = 0.0474596 * t438 * t425 + 0.028363733333333332 * t449 - 0.9086532 * t456 - t459 * t468;
        let t473 = t412 * t412;
        let t475 = 1.0 / t473 / t321;
        let t476 = t475 * t259;
        let t477 = t418 * t418;
        let t479 = 1.0 / t419 / t477;
        let t483 = 8.0 - 5.0 * t422 + 10.0 / 3.0 * t445 - t254 * t476 * t479 / 3.0;
        let t484 = t470 * t483;
        let t485 = 1.0 / t456;
        let t489 = 3.0 * t416;
        let t490 = 9.0 * t411 + t489;
        let t491 = rmath::sqrt(t490);
        let t493 = 9.0 * t465 + t489;
        let t494 = rmath::sqrt(t493);
        let t496 = t491 / 3.0 - t494 / 3.0;
        let t500 = t32 * t322;
        let t502 = t29 * t500 * t44;
        let t504 = t502 / 3.0 + t491 / 3.0;
        let t506 = t502 / 3.0 + t419;
        let t507 = 1.0 / t506;
        let t509 = rmath::ln(t504 * t507);
        let t513 = t502 / 3.0 + t494 / 3.0;
        let t515 = rmath::ln(t513 * t507);
        let t518 = 0.757211 + 0.04727288888888889 * t424 * t426 + 0.026366444444444446 * t448 * t450 - t484 * t485 / 9.0 + 2.0 / 3.0 * t33 * t323 * t496 + 2.0 * t411 * t509 - 2.0 * t465 * t515;
        let t522 = piecewise3(t307, 0.0, -3.0 / 8.0 * t5 * t316 * t518);
        let tzk0 = t306 + t522;
        zk[ip] += tzk0;
    }
}
