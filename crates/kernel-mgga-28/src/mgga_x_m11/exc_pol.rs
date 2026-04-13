//! MGGA_X_M11 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m11.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_m11_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_b_9: f64,
    param_b_10: f64,
    param_b_11: f64,
    param_hyb_omega_0: f64,
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
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t18 = t17 * t8;
        let t19 = piecewise5(t11, t12, t15, t16, t18);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t29 = pow_1_3(9.0);
        let t30 = t29 * t29;
        let t32 = pow_1_3(1.0 / M_PI);
        let t33 = t32 * t32;
        let t34 = t30 * t33;
        let t35 = t34 * param_hyb_omega_0;
        let t36 = 1.0 / t28;
        let t37 = t3 * t36;
        let t39 = 1.0 + t18 <= zeta_threshold;
        let t41 = 1.0 - t18 <= zeta_threshold;
        let t42 = piecewise5(t39, t12, t41, t16, t18);
        let t43 = 1.0 + t42;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(t43);
        let t46 = piecewise3(t44, t22, t45);
        let t47 = 1.0 / t46;
        let t50 = t35 * t37 * t47 / 18.0;
        let t51 = 0.135e1 <= t50;
        let t52 = 0.135e1 < t50;
        let t53 = piecewise3(t52, t50, 0.135e1);
        let t54 = t53 * t53;
        let t57 = t54 * t54;
        let t58 = 1.0 / t57;
        let t60 = t57 * t54;
        let t61 = 1.0 / t60;
        let t63 = t57 * t57;
        let t64 = 1.0 / t63;
        let t67 = 1.0 / t63 / t54;
        let t70 = 1.0 / t63 / t57;
        let t73 = 1.0 / t63 / t60;
        let t75 = t63 * t63;
        let t76 = 1.0 / t75;
        let t79 = piecewise3(t52, 0.135e1, t50);
        let t80 = f64::sqrt(M_PI);
        let t81 = 1.0 / t79;
        let t83 = erf_approx(t81 / 2.0);
        let t85 = t79 * t79;
        let t86 = 1.0 / t85;
        let t88 = f64::exp(-t86 / 4.0);
        let t89 = t88 - 1.0;
        let t92 = t88 - 3.0 / 2.0 - 2.0 * t85 * t89;
        let t95 = 2.0 * t79 * t92 + t80 * t83;
        let t99 = piecewise3(t51, 1.0 / t54 / 36.0 - t58 / 960.0 + t61 / 26880.0 - t64 / 829440.0 + t67 / 28385280.0 - t70 / 0.107347968e10 + t73 / 0.445906944e11 - t76 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t79 * t95);
        let t100 = t28 * t99;
        let t101 = M_CBRT6;
        let t102 = M_PI * M_PI;
        let t103 = pow_1_3(t102);
        let t104 = t103 * t103;
        let t105 = 1.0 / t104;
        let t106 = t101 * t105;
        let t107 = rho0 * rho0;
        let t108 = pow_1_3(rho0);
        let t109 = t108 * t108;
        let t111 = 1.0 / t109 / t107;
        let t113 = t106 * sigma0 * t111;
        let t115 = 0.804e0 + 0.914625e-2 * t113;
        let t118 = 0.1804e1 - 0.646416e0 / t115;
        let t119 = param_a_0;
        let t120 = param_a_1;
        let t121 = t101 * t101;
        let t123 = 3.0 / 10.0 * t121 * t104;
        let t125 = 1.0 / t109 / rho0;
        let t126 = tau0 * t125;
        let t127 = t123 - t126;
        let t128 = t120 * t127;
        let t129 = t123 + t126;
        let t130 = 1.0 / t129;
        let t132 = param_a_2;
        let t133 = t127 * t127;
        let t134 = t132 * t133;
        let t135 = t129 * t129;
        let t136 = 1.0 / t135;
        let t138 = param_a_3;
        let t139 = t133 * t127;
        let t140 = t138 * t139;
        let t141 = t135 * t129;
        let t142 = 1.0 / t141;
        let t144 = param_a_4;
        let t145 = t133 * t133;
        let t146 = t144 * t145;
        let t147 = t135 * t135;
        let t148 = 1.0 / t147;
        let t150 = param_a_5;
        let t151 = t145 * t127;
        let t152 = t150 * t151;
        let t153 = t147 * t129;
        let t154 = 1.0 / t153;
        let t156 = param_a_6;
        let t157 = t145 * t133;
        let t158 = t156 * t157;
        let t159 = t147 * t135;
        let t160 = 1.0 / t159;
        let t162 = param_a_7;
        let t163 = t145 * t139;
        let t164 = t162 * t163;
        let t165 = t147 * t141;
        let t166 = 1.0 / t165;
        let t168 = param_a_8;
        let t169 = t145 * t145;
        let t170 = t168 * t169;
        let t171 = t147 * t147;
        let t172 = 1.0 / t171;
        let t174 = param_a_9;
        let t175 = t169 * t127;
        let t176 = t174 * t175;
        let t178 = 1.0 / t171 / t129;
        let t180 = param_a_10;
        let t181 = t169 * t133;
        let t182 = t180 * t181;
        let t184 = 1.0 / t171 / t135;
        let t186 = param_a_11;
        let t187 = t169 * t139;
        let t188 = t186 * t187;
        let t190 = 1.0 / t171 / t141;
        let t192 = t128 * t130 + t134 * t136 + t140 * t142 + t146 * t148 + t152 * t154 + t158 * t160 + t164 * t166 + t170 * t172 + t176 * t178 + t182 * t184 + t188 * t190 + t119;
        let t195 = f64::exp(-0.93189002206715572255e-2 * t113);
        let t197 = 0.1552e1 - 0.552e0 * t195;
        let t198 = param_b_0;
        let t199 = param_b_1;
        let t200 = t199 * t127;
        let t202 = param_b_2;
        let t203 = t202 * t133;
        let t205 = param_b_3;
        let t206 = t205 * t139;
        let t208 = param_b_4;
        let t209 = t208 * t145;
        let t211 = param_b_5;
        let t212 = t211 * t151;
        let t214 = param_b_6;
        let t215 = t214 * t157;
        let t217 = param_b_7;
        let t218 = t217 * t163;
        let t220 = param_b_8;
        let t221 = t220 * t169;
        let t223 = param_b_9;
        let t224 = t223 * t175;
        let t226 = param_b_10;
        let t227 = t226 * t181;
        let t229 = param_b_11;
        let t230 = t229 * t187;
        let t232 = t200 * t130 + t203 * t136 + t206 * t142 + t209 * t148 + t212 * t154 + t215 * t160 + t218 * t166 + t221 * t172 + t224 * t178 + t227 * t184 + t230 * t190 + t198;
        let t234 = t118 * t192 + t197 * t232;
        let t235 = t100 * t234;
        let t238 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t235);
        let t239 = rho1 <= dens_threshold;
        let t240 = -t17;
        let t242 = piecewise5(t15, t12, t11, t16, t240 * t8);
        let t243 = 1.0 + t242;
        let t244 = t243 <= zeta_threshold;
        let t245 = pow_1_3(t243);
        let t247 = piecewise3(t244, t23, t245 * t243);
        let t248 = t6 * t247;
        let t249 = piecewise5(t41, t12, t39, t16, -t18);
        let t250 = 1.0 + t249;
        let t251 = t250 <= zeta_threshold;
        let t252 = pow_1_3(t250);
        let t253 = piecewise3(t251, t22, t252);
        let t254 = 1.0 / t253;
        let t257 = t35 * t37 * t254 / 18.0;
        let t258 = 0.135e1 <= t257;
        let t259 = 0.135e1 < t257;
        let t260 = piecewise3(t259, t257, 0.135e1);
        let t261 = t260 * t260;
        let t264 = t261 * t261;
        let t265 = 1.0 / t264;
        let t267 = t264 * t261;
        let t268 = 1.0 / t267;
        let t270 = t264 * t264;
        let t271 = 1.0 / t270;
        let t274 = 1.0 / t270 / t261;
        let t277 = 1.0 / t270 / t264;
        let t280 = 1.0 / t270 / t267;
        let t282 = t270 * t270;
        let t283 = 1.0 / t282;
        let t286 = piecewise3(t259, 0.135e1, t257);
        let t287 = 1.0 / t286;
        let t289 = erf_approx(t287 / 2.0);
        let t291 = t286 * t286;
        let t292 = 1.0 / t291;
        let t294 = f64::exp(-t292 / 4.0);
        let t295 = t294 - 1.0;
        let t298 = t294 - 3.0 / 2.0 - 2.0 * t291 * t295;
        let t301 = 2.0 * t286 * t298 + t80 * t289;
        let t305 = piecewise3(t258, 1.0 / t261 / 36.0 - t265 / 960.0 + t268 / 26880.0 - t271 / 829440.0 + t274 / 28385280.0 - t277 / 0.107347968e10 + t280 / 0.445906944e11 - t283 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t286 * t301);
        let t306 = t28 * t305;
        let t307 = rho1 * rho1;
        let t308 = pow_1_3(rho1);
        let t309 = t308 * t308;
        let t311 = 1.0 / t309 / t307;
        let t313 = t106 * sigma2 * t311;
        let t315 = 0.804e0 + 0.914625e-2 * t313;
        let t318 = 0.1804e1 - 0.646416e0 / t315;
        let t320 = 1.0 / t309 / rho1;
        let t321 = tau1 * t320;
        let t322 = t123 - t321;
        let t323 = t120 * t322;
        let t324 = t123 + t321;
        let t325 = 1.0 / t324;
        let t327 = t322 * t322;
        let t328 = t132 * t327;
        let t329 = t324 * t324;
        let t330 = 1.0 / t329;
        let t332 = t327 * t322;
        let t333 = t138 * t332;
        let t334 = t329 * t324;
        let t335 = 1.0 / t334;
        let t337 = t327 * t327;
        let t338 = t144 * t337;
        let t339 = t329 * t329;
        let t340 = 1.0 / t339;
        let t342 = t337 * t322;
        let t343 = t150 * t342;
        let t344 = t339 * t324;
        let t345 = 1.0 / t344;
        let t347 = t337 * t327;
        let t348 = t156 * t347;
        let t349 = t339 * t329;
        let t350 = 1.0 / t349;
        let t352 = t337 * t332;
        let t353 = t162 * t352;
        let t354 = t339 * t334;
        let t355 = 1.0 / t354;
        let t357 = t337 * t337;
        let t358 = t168 * t357;
        let t359 = t339 * t339;
        let t360 = 1.0 / t359;
        let t362 = t357 * t322;
        let t363 = t174 * t362;
        let t365 = 1.0 / t359 / t324;
        let t367 = t357 * t327;
        let t368 = t180 * t367;
        let t370 = 1.0 / t359 / t329;
        let t372 = t357 * t332;
        let t373 = t186 * t372;
        let t375 = 1.0 / t359 / t334;
        let t377 = t323 * t325 + t328 * t330 + t333 * t335 + t338 * t340 + t343 * t345 + t348 * t350 + t353 * t355 + t358 * t360 + t363 * t365 + t368 * t370 + t373 * t375 + t119;
        let t380 = f64::exp(-0.93189002206715572255e-2 * t313);
        let t382 = 0.1552e1 - 0.552e0 * t380;
        let t383 = t199 * t322;
        let t385 = t202 * t327;
        let t387 = t205 * t332;
        let t389 = t208 * t337;
        let t391 = t211 * t342;
        let t393 = t214 * t347;
        let t395 = t217 * t352;
        let t397 = t220 * t357;
        let t399 = t223 * t362;
        let t401 = t226 * t367;
        let t403 = t229 * t372;
        let t405 = t383 * t325 + t385 * t330 + t387 * t335 + t389 * t340 + t391 * t345 + t393 * t350 + t395 * t355 + t397 * t360 + t399 * t365 + t401 * t370 + t403 * t375 + t198;
        let t407 = t318 * t377 + t382 * t405;
        let t408 = t306 * t407;
        let t411 = piecewise3(t239, 0.0, -3.0 / 8.0 * t248 * t408);
        let tzk0 = t238 + t411;
        zk[ip] += tzk0;
    }
}
