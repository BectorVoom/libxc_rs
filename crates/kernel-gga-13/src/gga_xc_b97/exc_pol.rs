//! GGA_XC_B97 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 278 shared lines across all orders.
//! Delta: 278 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_xc_b97_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_c_ab_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ss_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_x_0: f64,
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_c_x_3: f64,
    param_c_x_4: f64,
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
        // --- shared preamble (278 lines) ---
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = 1.0 + t5;
        let t7 = t6 <= zeta_threshold;
        let t8 = rho0 <= dens_threshold || t7;
        let t9 = piecewise3(t7, zeta_threshold, t6);
        let t10 = rho0 * t4;
        let t12 = 2.0 * t10 <= zeta_threshold;
        let t13 = pow_1_3(zeta_threshold);
        let t14 = 1.0 / t13;
        let t15 = M_CBRT2;
        let t16 = t15 * t15;
        let t17 = pow_1_3(t10);
        let t21 = piecewise3(t12, t14, t16 / t17 / 2.0);
        let t22 = t21 * t21;
        let t24 = 1.0 / t22 / t21;
        let t27 = t3 * t24 / 2.0 <= dens_threshold;
        let t28 = M_CBRT3;
        let t29 = M_CBRTPI;
        let t31 = t28 / t29;
        let t32 = t31 * t16;
        let t34 = t13 * zeta_threshold;
        let t36 = piecewise3(2.0 <= zeta_threshold, t34, 2.0 * t15);
        let t37 = pow_1_3(t3);
        let t38 = t36 * t37;
        let t39 = 1.0 / t21;
        let t43 = piecewise3(t27, 0.0, -3.0 / 16.0 * t32 * t38 * t39);
        let t44 = 0.0 <= dens_threshold;
        let t46 = piecewise3(0.0 <= zeta_threshold, t34, 0.0);
        let t47 = t46 * t37;
        let t51 = piecewise3(t44, 0.0, -3.0 / 16.0 * t32 * t47 * t39);
        let t52 = t43 + t51;
        let t55 = piecewise3(t8, 0.0, t9 * t52 / 2.0);
        let t56 = param_c_x_0;
        let t57 = param_c_x_1;
        let t58 = t57 * sigma0;
        let t59 = rho0 * rho0;
        let t60 = pow_1_3(rho0);
        let t61 = t60 * t60;
        let t63 = 1.0 / t61 / t59;
        let t64 = sigma0 * t63;
        let t66 = 1.0 + 0.4e-2 * t64;
        let t67 = 1.0 / t66;
        let t71 = param_c_x_2;
        let t72 = sigma0 * sigma0;
        let t73 = t71 * t72;
        let t74 = t59 * t59;
        let t75 = t74 * rho0;
        let t77 = 1.0 / t60 / t75;
        let t78 = t66 * t66;
        let t79 = 1.0 / t78;
        let t80 = t77 * t79;
        let t83 = param_c_x_3;
        let t84 = t72 * sigma0;
        let t85 = t83 * t84;
        let t86 = t74 * t74;
        let t87 = 1.0 / t86;
        let t88 = t78 * t66;
        let t89 = 1.0 / t88;
        let t90 = t87 * t89;
        let t93 = param_c_x_4;
        let t94 = t72 * t72;
        let t95 = t93 * t94;
        let t96 = t86 * t59;
        let t98 = 1.0 / t61 / t96;
        let t99 = t78 * t78;
        let t100 = 1.0 / t99;
        let t101 = t98 * t100;
        let t104 = t56 + 0.4e-2 * t58 * t63 * t67 + 0.16e-4 * t73 * t80 + 0.64e-7 * t85 * t90 + 0.256e-9 * t95 * t101;
        let t105 = t55 * t104;
        let t107 = 1.0 - t5;
        let t108 = t107 <= zeta_threshold;
        let t109 = rho1 <= dens_threshold || t108;
        let t110 = piecewise3(t108, zeta_threshold, t107);
        let t111 = rho1 * t4;
        let t113 = 2.0 * t111 <= zeta_threshold;
        let t114 = pow_1_3(t111);
        let t118 = piecewise3(t113, t14, t16 / t114 / 2.0);
        let t119 = t118 * t118;
        let t121 = 1.0 / t119 / t118;
        let t124 = t3 * t121 / 2.0 <= dens_threshold;
        let t125 = 1.0 / t118;
        let t129 = piecewise3(t124, 0.0, -3.0 / 16.0 * t32 * t38 * t125);
        let t133 = piecewise3(t44, 0.0, -3.0 / 16.0 * t32 * t47 * t125);
        let t134 = t129 + t133;
        let t137 = piecewise3(t109, 0.0, t110 * t134 / 2.0);
        let t138 = t57 * sigma2;
        let t139 = rho1 * rho1;
        let t140 = pow_1_3(rho1);
        let t141 = t140 * t140;
        let t143 = 1.0 / t141 / t139;
        let t144 = sigma2 * t143;
        let t146 = 1.0 + 0.4e-2 * t144;
        let t147 = 1.0 / t146;
        let t151 = sigma2 * sigma2;
        let t152 = t71 * t151;
        let t153 = t139 * t139;
        let t154 = t153 * rho1;
        let t156 = 1.0 / t140 / t154;
        let t157 = t146 * t146;
        let t158 = 1.0 / t157;
        let t159 = t156 * t158;
        let t162 = t151 * sigma2;
        let t163 = t83 * t162;
        let t164 = t153 * t153;
        let t165 = 1.0 / t164;
        let t166 = t157 * t146;
        let t167 = 1.0 / t166;
        let t168 = t165 * t167;
        let t171 = t151 * t151;
        let t172 = t93 * t171;
        let t173 = t164 * t139;
        let t175 = 1.0 / t141 / t173;
        let t176 = t157 * t157;
        let t177 = 1.0 / t176;
        let t178 = t175 * t177;
        let t181 = t56 + 0.4e-2 * t138 * t143 * t147 + 0.16e-4 * t152 * t159 + 0.64e-7 * t163 * t168 + 0.256e-9 * t172 * t178;
        let t182 = t137 * t181;
        let t183 = 1.0 / M_PI;
        let t184 = pow_1_3(t183);
        let t185 = t28 * t184;
        let t186 = M_CBRT4;
        let t187 = t186 * t186;
        let t188 = t185 * t187;
        let t189 = 1.0 / t37;
        let t190 = t189 * t15;
        let t191 = pow_1_3(t6);
        let t193 = piecewise3(t7, t14, 1.0 / t191);
        let t195 = t188 * t190 * t193;
        let t197 = 1.0 + 0.53425e-1 * t195;
        let t198 = f64::sqrt(t195);
        let t201 = pow_3_2(t195);
        let t203 = t28 * t28;
        let t204 = t184 * t184;
        let t205 = t203 * t204;
        let t206 = t205 * t186;
        let t207 = t37 * t37;
        let t208 = 1.0 / t207;
        let t209 = t208 * t16;
        let t210 = t193 * t193;
        let t212 = t206 * t209 * t210;
        let t214 = 0.379785e1 * t198 + 0.8969e0 * t195 + 0.204775e0 * t201 + 0.123235e0 * t212;
        let t217 = 1.0 + 0.16081824322151104822e2 / t214;
        let t218 = f64::ln(t217);
        let t220 = 0.62182e-1 * t197 * t218;
        let t224 = 1.0 / (2.0 * t15 - 2.0);
        let t225 = (t36 + t46 - 2.0) * t224;
        let t227 = 1.0 + 0.5137e-1 * t195;
        let t232 = 0.705945e1 * t198 + 0.1549425e1 * t195 + 0.420775e0 * t201 + 0.1562925e0 * t212;
        let t235 = 1.0 + 0.32164683177870697974e2 / t232;
        let t236 = f64::ln(t235);
        let t240 = 1.0 + 0.278125e-1 * t195;
        let t245 = 0.51785e1 * t198 + 0.905775e0 * t195 + 0.1100325e0 * t201 + 0.1241775e0 * t212;
        let t248 = 1.0 + 0.29608574643216675549e2 / t245;
        let t249 = f64::ln(t248);
        let t250 = t240 * t249;
        let t256 = -t220 + t225 * (-0.3109e-1 * t227 * t236 + t220 - 0.19751789702565206229e-1 * t250) + 0.19751789702565206229e-1 * t225 * t250;
        let t259 = piecewise3(t8, 0.0, t9 * t256 / 2.0);
        let t260 = param_c_ss_0;
        let t261 = param_c_ss_1;
        let t262 = t261 * sigma0;
        let t264 = 1.0 + 0.2e0 * t64;
        let t265 = 1.0 / t264;
        let t269 = param_c_ss_2;
        let t270 = t269 * t72;
        let t271 = t264 * t264;
        let t272 = 1.0 / t271;
        let t273 = t77 * t272;
        let t276 = param_c_ss_3;
        let t277 = t276 * t84;
        let t278 = t271 * t264;
        let t279 = 1.0 / t278;
        let t280 = t87 * t279;
        let t283 = param_c_ss_4;
        let t284 = t283 * t94;
        let t285 = t271 * t271;
        let t286 = 1.0 / t285;
        let t287 = t98 * t286;
        let t290 = t260 + 0.2e0 * t262 * t63 * t265 + 0.4e-1 * t270 * t273 + 0.8e-2 * t277 * t280 + 0.16e-2 * t284 * t287;
        let t291 = t259 * t290;
        let t292 = pow_1_3(t107);
        let t294 = piecewise3(t108, t14, 1.0 / t292);
        let t296 = t188 * t190 * t294;
        let t298 = 1.0 + 0.53425e-1 * t296;
        let t299 = f64::sqrt(t296);
        let t302 = pow_3_2(t296);
        let t304 = t294 * t294;
        let t306 = t206 * t209 * t304;
        let t308 = 0.379785e1 * t299 + 0.8969e0 * t296 + 0.204775e0 * t302 + 0.123235e0 * t306;
        let t311 = 1.0 + 0.16081824322151104822e2 / t308;
        let t312 = f64::ln(t311);
        let t314 = 0.62182e-1 * t298 * t312;
        let t316 = 1.0 + 0.5137e-1 * t296;
        let t321 = 0.705945e1 * t299 + 0.1549425e1 * t296 + 0.420775e0 * t302 + 0.1562925e0 * t306;
        let t324 = 1.0 + 0.32164683177870697974e2 / t321;
        let t325 = f64::ln(t324);
        let t329 = 1.0 + 0.278125e-1 * t296;
        let t334 = 0.51785e1 * t299 + 0.905775e0 * t296 + 0.1100325e0 * t302 + 0.1241775e0 * t306;
        let t337 = 1.0 + 0.29608574643216675549e2 / t334;
        let t338 = f64::ln(t337);
        let t339 = t329 * t338;
        let t345 = -t314 + t225 * (-0.3109e-1 * t316 * t325 + t314 - 0.19751789702565206229e-1 * t339) + 0.19751789702565206229e-1 * t225 * t339;
        let t348 = piecewise3(t109, 0.0, t110 * t345 / 2.0);
        let t349 = t261 * sigma2;
        let t351 = 1.0 + 0.2e0 * t144;
        let t352 = 1.0 / t351;
        let t356 = t269 * t151;
        let t357 = t351 * t351;
        let t358 = 1.0 / t357;
        let t359 = t156 * t358;
        let t362 = t276 * t162;
        let t363 = t357 * t351;
        let t364 = 1.0 / t363;
        let t365 = t165 * t364;
        let t368 = t283 * t171;
        let t369 = t357 * t357;
        let t370 = 1.0 / t369;
        let t371 = t175 * t370;
        let t374 = t260 + 0.2e0 * t349 * t143 * t352 + 0.4e-1 * t356 * t359 + 0.8e-2 * t362 * t365 + 0.16e-2 * t368 * t371;
        let t375 = t348 * t374;
        let t377 = t185 * t187 * t189;
        let t379 = 1.0 + 0.53425e-1 * t377;
        let t380 = f64::sqrt(t377);
        let t383 = pow_3_2(t377);
        let t386 = t205 * t186 * t208;
        let t388 = 0.379785e1 * t380 + 0.8969e0 * t377 + 0.204775e0 * t383 + 0.123235e0 * t386;
        let t391 = 1.0 + 0.16081824322151104822e2 / t388;
        let t392 = f64::ln(t391);
        let t394 = 0.62182e-1 * t379 * t392;
        let t395 = t2 * t2;
        let t396 = t395 * t395;
        let t397 = t3 * t3;
        let t398 = t397 * t397;
        let t399 = 1.0 / t398;
        let t400 = t396 * t399;
        let t401 = t191 * t6;
        let t402 = piecewise3(t7, t34, t401);
        let t403 = t292 * t107;
        let t404 = piecewise3(t108, t34, t403);
        let t405 = t402 + t404 - 2.0;
        let t406 = t405 * t224;
        let t408 = 1.0 + 0.5137e-1 * t377;
        let t413 = 0.705945e1 * t380 + 0.1549425e1 * t377 + 0.420775e0 * t383 + 0.1562925e0 * t386;
        let t416 = 1.0 + 0.32164683177870697974e2 / t413;
        let t417 = f64::ln(t416);
        let t421 = 1.0 + 0.278125e-1 * t377;
        let t426 = 0.51785e1 * t380 + 0.905775e0 * t377 + 0.1100325e0 * t383 + 0.1241775e0 * t386;
        let t429 = 1.0 + 0.29608574643216675549e2 / t426;
        let t430 = f64::ln(t429);
        let t431 = t421 * t430;
        let t433 = -0.3109e-1 * t408 * t417 + t394 - 0.19751789702565206229e-1 * t431;
        let t434 = t406 * t433;
        let t438 = -t394 + t400 * t434 + 0.19751789702565206229e-1 * t406 * t431 - t259 - t348;
        let t440 = param_c_ab_1;
        let t441 = t64 + t144;
        let t442 = t440 * t441;
        let t445 = 1.0 + 0.3e-2 * t64 + 0.3e-2 * t144;
        let t446 = 1.0 / t445;
        let t449 = param_c_ab_2;
        let t450 = t441 * t441;
        let t451 = t449 * t450;
        let t452 = t445 * t445;
        let t453 = 1.0 / t452;
        let t456 = param_c_ab_3;
        let t457 = t450 * t441;
        let t458 = t456 * t457;
        let t459 = t452 * t445;
        let t460 = 1.0 / t459;
        let t463 = param_c_ab_4;
        let t464 = t450 * t450;
        let t465 = t463 * t464;
        let t466 = t452 * t452;
        let t467 = 1.0 / t466;
        let t470 = param_c_ab_0 + 0.3e-2 * t442 * t446 + 0.9e-5 * t451 * t453 + 0.27e-7 * t458 * t460 + 0.81e-10 * t465 * t467;
        let t471 = t438 * t470;
        let tzk0 = t105 + t182 + t291 + t375 + t471;
        zk[ip] += tzk0;
    }
}
