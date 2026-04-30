//! MGGA_XC_B97MV exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_b97mv.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_b97mv_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_c_os_0: f64,
    param_c_os_1: f64,
    param_c_os_2: f64,
    param_c_os_3: f64,
    param_c_os_4: f64,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = 1.0 + t5;
        let t7 = t6 <= zeta_threshold;
        let t8 = piecewise3(t7, zeta_threshold, t6);
        let t9 = rho0 <= dens_threshold;
        let t10 = M_CBRT3;
        let t11 = M_CBRTPI;
        let t13 = t10 / t11;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * zeta_threshold;
        let t17 = M_CBRT2;
        let t19 = piecewise3(2.0 <= zeta_threshold, t16, 2.0 * t17);
        let t20 = pow_1_3(t3);
        let t21 = t19 * t20;
        let t22 = 1.0 / rho0;
        let t23 = t3 * t22;
        let t24 = pow_1_3(t23);
        let t25 = 1.0 / t24;
        let t29 = piecewise3(t9, 0.0, -3.0 / 8.0 * t13 * t21 * t25);
        let t30 = 0.0 <= dens_threshold;
        let t32 = piecewise3(0.0 <= zeta_threshold, t16, 0.0);
        let t33 = t32 * t20;
        let t37 = piecewise3(t30, 0.0, -3.0 / 8.0 * t13 * t33 * t25);
        let t38 = t29 + t37;
        let t39 = t8 * t38;
        let t40 = param_c_x_0;
        let t41 = param_c_x_1;
        let t42 = t41 * sigma0;
        let t43 = rho0 * rho0;
        let t44 = pow_1_3(rho0);
        let t45 = t44 * t44;
        let t47 = 1.0 / t45 / t43;
        let t48 = sigma0 * t47;
        let t50 = 1.0 + 0.4e-2 * t48;
        let t51 = 1.0 / t50;
        let t55 = param_c_x_2;
        let t56 = sigma0 * sigma0;
        let t57 = t55 * t56;
        let t58 = t43 * t43;
        let t59 = t58 * rho0;
        let t61 = 1.0 / t44 / t59;
        let t62 = t50 * t50;
        let t63 = 1.0 / t62;
        let t64 = t61 * t63;
        let t67 = param_c_x_3;
        let t68 = M_CBRT6;
        let t69 = t68 * t68;
        let t70 = M_PI * M_PI;
        let t71 = pow_1_3(t70);
        let t72 = t71 * t71;
        let t73 = t69 * t72;
        let t74 = 3.0 / 10.0 * t73;
        let t76 = 1.0 / t45 / rho0;
        let t77 = tau0 * t76;
        let t78 = t74 - t77;
        let t79 = t67 * t78;
        let t80 = t74 + t77;
        let t81 = 1.0 / t80;
        let t83 = param_c_x_4;
        let t84 = t83 * t78;
        let t85 = t84 * t81;
        let t89 = t40 + 0.4e-2 * t42 * t47 * t51 + 0.16e-4 * t57 * t64 + t79 * t81 + 0.4e-2 * t85 * t48 * t51;
        let t91 = t39 * t89 / 2.0;
        let t92 = 1.0 - t5;
        let t93 = t92 <= zeta_threshold;
        let t94 = piecewise3(t93, zeta_threshold, t92);
        let t95 = rho1 <= dens_threshold;
        let t96 = 1.0 / rho1;
        let t97 = t3 * t96;
        let t98 = pow_1_3(t97);
        let t99 = 1.0 / t98;
        let t103 = piecewise3(t95, 0.0, -3.0 / 8.0 * t13 * t21 * t99);
        let t107 = piecewise3(t30, 0.0, -3.0 / 8.0 * t13 * t33 * t99);
        let t108 = t103 + t107;
        let t109 = t94 * t108;
        let t110 = t41 * sigma2;
        let t111 = rho1 * rho1;
        let t112 = pow_1_3(rho1);
        let t113 = t112 * t112;
        let t115 = 1.0 / t113 / t111;
        let t116 = sigma2 * t115;
        let t118 = 1.0 + 0.4e-2 * t116;
        let t119 = 1.0 / t118;
        let t123 = sigma2 * sigma2;
        let t124 = t55 * t123;
        let t125 = t111 * t111;
        let t126 = t125 * rho1;
        let t128 = 1.0 / t112 / t126;
        let t129 = t118 * t118;
        let t130 = 1.0 / t129;
        let t131 = t128 * t130;
        let t135 = 1.0 / t113 / rho1;
        let t136 = tau1 * t135;
        let t137 = t74 - t136;
        let t138 = t67 * t137;
        let t139 = t74 + t136;
        let t140 = 1.0 / t139;
        let t142 = t83 * t137;
        let t143 = t142 * t140;
        let t147 = t40 + 0.4e-2 * t110 * t115 * t119 + 0.16e-4 * t124 * t131 + t138 * t140 + 0.4e-2 * t143 * t116 * t119;
        let t149 = t109 * t147 / 2.0;
        let t150 = t9 || t7;
        let t151 = 1.0 / M_PI;
        let t152 = pow_1_3(t151);
        let t153 = t10 * t152;
        let t154 = M_CBRT4;
        let t155 = t154 * t154;
        let t156 = t153 * t155;
        let t157 = 1.0 / t20;
        let t158 = t157 * t17;
        let t159 = 1.0 / t15;
        let t160 = pow_1_3(t6);
        let t162 = piecewise3(t7, t159, 1.0 / t160);
        let t164 = t156 * t158 * t162;
        let t166 = 1.0 + 0.53425e-1 * t164;
        let t167 = f64::sqrt(t164);
        let t170 = pow_3_2(t164);
        let t172 = t10 * t10;
        let t173 = t152 * t152;
        let t174 = t172 * t173;
        let t175 = t174 * t154;
        let t176 = t20 * t20;
        let t177 = 1.0 / t176;
        let t178 = t17 * t17;
        let t179 = t177 * t178;
        let t180 = t162 * t162;
        let t182 = t175 * t179 * t180;
        let t184 = 0.379785e1 * t167 + 0.8969e0 * t164 + 0.204775e0 * t170 + 0.123235e0 * t182;
        let t187 = 1.0 + 0.16081979498692535067e2 / t184;
        let t188 = f64::ln(t187);
        let t190 = 0.621814e-1 * t166 * t188;
        let t194 = 1.0 / (2.0 * t17 - 2.0);
        let t195 = (t19 + t32 - 2.0) * t194;
        let t197 = 1.0 + 0.5137e-1 * t164;
        let t202 = 0.705945e1 * t167 + 0.1549425e1 * t164 + 0.420775e0 * t170 + 0.1562925e0 * t182;
        let t205 = 1.0 + 0.32163958997385070134e2 / t202;
        let t206 = f64::ln(t205);
        let t210 = 1.0 + 0.278125e-1 * t164;
        let t215 = 0.51785e1 * t167 + 0.905775e0 * t164 + 0.1100325e0 * t170 + 0.1241775e0 * t182;
        let t218 = 1.0 + 0.29608749977793437516e2 / t215;
        let t219 = f64::ln(t218);
        let t220 = t210 * t219;
        let t226 = -t190 + t195 * (-0.310907e-1 * t197 * t206 + t190 - 0.19751673498613801407e-1 * t220) + 0.19751673498613801407e-1 * t195 * t220;
        let t229 = piecewise3(t150, 0.0, t8 * t226 / 2.0);
        let t230 = param_c_ss_0;
        let t231 = param_c_ss_1;
        let t232 = t231 * t56;
        let t234 = 1.0 + 0.2e0 * t48;
        let t235 = t234 * t234;
        let t236 = 1.0 / t235;
        let t237 = t61 * t236;
        let t240 = param_c_ss_2;
        let t241 = t240 * t78;
        let t243 = param_c_ss_3;
        let t244 = t78 * t78;
        let t245 = t244 * t78;
        let t246 = t243 * t245;
        let t247 = t80 * t80;
        let t248 = t247 * t80;
        let t249 = 1.0 / t248;
        let t250 = t246 * t249;
        let t252 = t56 * t61 * t236;
        let t255 = param_c_ss_4;
        let t256 = t244 * t244;
        let t257 = t255 * t256;
        let t258 = t247 * t247;
        let t259 = 1.0 / t258;
        let t260 = t257 * t259;
        let t263 = t230 + 0.4e-1 * t232 * t237 + t241 * t81 + 0.4e-1 * t250 * t252 + 0.4e-1 * t260 * t252;
        let t264 = t229 * t263;
        let t265 = t95 || t93;
        let t266 = pow_1_3(t92);
        let t268 = piecewise3(t93, t159, 1.0 / t266);
        let t270 = t156 * t158 * t268;
        let t272 = 1.0 + 0.53425e-1 * t270;
        let t273 = f64::sqrt(t270);
        let t276 = pow_3_2(t270);
        let t278 = t268 * t268;
        let t280 = t175 * t179 * t278;
        let t282 = 0.379785e1 * t273 + 0.8969e0 * t270 + 0.204775e0 * t276 + 0.123235e0 * t280;
        let t285 = 1.0 + 0.16081979498692535067e2 / t282;
        let t286 = f64::ln(t285);
        let t288 = 0.621814e-1 * t272 * t286;
        let t290 = 1.0 + 0.5137e-1 * t270;
        let t295 = 0.705945e1 * t273 + 0.1549425e1 * t270 + 0.420775e0 * t276 + 0.1562925e0 * t280;
        let t298 = 1.0 + 0.32163958997385070134e2 / t295;
        let t299 = f64::ln(t298);
        let t303 = 1.0 + 0.278125e-1 * t270;
        let t308 = 0.51785e1 * t273 + 0.905775e0 * t270 + 0.1100325e0 * t276 + 0.1241775e0 * t280;
        let t311 = 1.0 + 0.29608749977793437516e2 / t308;
        let t312 = f64::ln(t311);
        let t313 = t303 * t312;
        let t319 = -t288 + t195 * (-0.310907e-1 * t290 * t299 + t288 - 0.19751673498613801407e-1 * t313) + 0.19751673498613801407e-1 * t195 * t313;
        let t322 = piecewise3(t265, 0.0, t94 * t319 / 2.0);
        let t323 = t231 * t123;
        let t325 = 1.0 + 0.2e0 * t116;
        let t326 = t325 * t325;
        let t327 = 1.0 / t326;
        let t328 = t128 * t327;
        let t331 = t240 * t137;
        let t333 = t137 * t137;
        let t334 = t333 * t137;
        let t335 = t243 * t334;
        let t336 = t139 * t139;
        let t337 = t336 * t139;
        let t338 = 1.0 / t337;
        let t339 = t335 * t338;
        let t341 = t123 * t128 * t327;
        let t344 = t333 * t333;
        let t345 = t255 * t344;
        let t346 = t336 * t336;
        let t347 = 1.0 / t346;
        let t348 = t345 * t347;
        let t351 = t230 + 0.4e-1 * t323 * t328 + t331 * t140 + 0.4e-1 * t339 * t341 + 0.4e-1 * t348 * t341;
        let t352 = t322 * t351;
        let t354 = t153 * t155 * t157;
        let t356 = 1.0 + 0.53425e-1 * t354;
        let t357 = f64::sqrt(t354);
        let t360 = pow_3_2(t354);
        let t363 = t174 * t154 * t177;
        let t365 = 0.379785e1 * t357 + 0.8969e0 * t354 + 0.204775e0 * t360 + 0.123235e0 * t363;
        let t368 = 1.0 + 0.16081979498692535067e2 / t365;
        let t369 = f64::ln(t368);
        let t371 = 0.621814e-1 * t356 * t369;
        let t372 = t2 * t2;
        let t373 = t372 * t372;
        let t374 = t3 * t3;
        let t375 = t374 * t374;
        let t376 = 1.0 / t375;
        let t377 = t373 * t376;
        let t378 = t160 * t6;
        let t379 = piecewise3(t7, t16, t378);
        let t380 = t266 * t92;
        let t381 = piecewise3(t93, t16, t380);
        let t382 = t379 + t381 - 2.0;
        let t383 = t382 * t194;
        let t385 = 1.0 + 0.5137e-1 * t354;
        let t390 = 0.705945e1 * t357 + 0.1549425e1 * t354 + 0.420775e0 * t360 + 0.1562925e0 * t363;
        let t393 = 1.0 + 0.32163958997385070134e2 / t390;
        let t394 = f64::ln(t393);
        let t398 = 1.0 + 0.278125e-1 * t354;
        let t403 = 0.51785e1 * t357 + 0.905775e0 * t354 + 0.1100325e0 * t360 + 0.1241775e0 * t363;
        let t406 = 1.0 + 0.29608749977793437516e2 / t403;
        let t407 = f64::ln(t406);
        let t408 = t398 * t407;
        let t410 = -0.310907e-1 * t385 * t394 + t371 - 0.19751673498613801407e-1 * t408;
        let t411 = t383 * t410;
        let t415 = -t371 + t377 * t411 + 0.19751673498613801407e-1 * t383 * t408 - t229 - t322;
        let t417 = param_c_os_1;
        let t418 = t48 + t116;
        let t419 = t417 * t418;
        let t422 = 1.0 + 0.3e-2 * t48 + 0.3e-2 * t116;
        let t423 = 1.0 / t422;
        let t426 = param_c_os_2;
        let t427 = t418 * t418;
        let t429 = t426 * t427 * t418;
        let t430 = t422 * t422;
        let t431 = t430 * t422;
        let t432 = 1.0 / t431;
        let t435 = param_c_os_3;
        let t438 = 3.0 / 10.0 * t73 * (t77 + t136);
        let t440 = 2.0 * t77 * t136;
        let t441 = t438 - t440;
        let t442 = t435 * t441;
        let t443 = t438 + t440;
        let t444 = 1.0 / t443;
        let t446 = param_c_os_4;
        let t447 = t441 * t441;
        let t449 = t446 * t447 * t441;
        let t450 = t443 * t443;
        let t451 = t450 * t443;
        let t452 = 1.0 / t451;
        let t453 = t452 * t427;
        let t454 = 1.0 / t430;
        let t455 = t453 * t454;
        let t458 = param_c_os_0 + 0.3e-2 * t419 * t423 + 0.27e-7 * t429 * t432 + t442 * t444 + 0.9e-5 * t449 * t455;
        let t459 = t415 * t458;
        let tzk0 = t91 + t149 + t264 + t352 + t459;
        zk[ip] += tzk0;
    }
}
