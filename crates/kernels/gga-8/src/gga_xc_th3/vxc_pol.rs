//! GGA_XC_TH3 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 122 shared lines across all orders.
//! Delta: 109 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_xc_th3_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    param_omega_18: f64,
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
        // --- shared preamble (122 lines) ---
        let t1 = param_omega_0;
        let t2 = f64::powf(rho0, 1.0 / 6.0);
        let t3 = t2 * rho0;
        let t4 = f64::powf(rho1, 1.0 / 6.0);
        let t5 = t4 * rho1;
        let t6 = t3 + t5;
        let t8 = param_omega_1;
        let t9 = pow_1_3(rho0);
        let t10 = t9 * rho0;
        let t11 = pow_1_3(rho1);
        let t12 = t11 * rho1;
        let t13 = t10 + t12;
        let t15 = param_omega_2;
        let t16 = f64::sqrt(rho0);
        let t17 = t16 * rho0;
        let t18 = f64::sqrt(rho1);
        let t19 = t18 * rho1;
        let t20 = t17 + t19;
        let t22 = param_omega_3;
        let t23 = t9 * t9;
        let t24 = t23 * rho0;
        let t25 = t11 * t11;
        let t26 = t25 * rho1;
        let t27 = t24 + t26;
        let t29 = param_omega_4;
        let t30 = f64::powf(rho0, 1.0 / 12.0);
        let t31 = t30 * t30;
        let t32 = t31 * t31;
        let t33 = t32 * t30;
        let t35 = f64::powf(rho1, 1.0 / 12.0);
        let t36 = t35 * t35;
        let t37 = t36 * t36;
        let t38 = t37 * t35;
        let t41 = t29 * (t33 * rho0 + t38 * rho1);
        let t42 = f64::sqrt(sigma0);
        let t43 = 1.0 / t10;
        let t44 = t42 * t43;
        let t45 = rho0 - rho1;
        let t46 = rho0 + rho1;
        let t47 = 1.0 / t46;
        let t48 = t45 * t47;
        let t49 = 1.0 + t48;
        let t50 = t49 <= zeta_threshold;
        let t51 = pow_1_3(zeta_threshold);
        let t52 = t51 * zeta_threshold;
        let t53 = pow_1_3(t49);
        let t55 = piecewise3(t50, t52, t53 * t49);
        let t56 = M_CBRT2;
        let t57 = t56 * t56;
        let t58 = t55 * t57;
        let t60 = f64::sqrt(sigma2);
        let t61 = 1.0 / t12;
        let t62 = t60 * t61;
        let t63 = 1.0 - t48;
        let t64 = t63 <= zeta_threshold;
        let t65 = pow_1_3(t63);
        let t67 = piecewise3(t64, t52, t65 * t63);
        let t68 = t67 * t57;
        let t71 = t44 * t58 / 4.0 + t62 * t68 / 4.0;
        let t74 = param_omega_5;
        let t75 = t74 * t20;
        let t78 = param_omega_6;
        let t79 = t78 * t27;
        let t82 = param_omega_7;
        let t83 = t2 * t2;
        let t84 = t83 * t83;
        let t85 = t84 * t2;
        let t86 = t85 * rho0;
        let t87 = t4 * t4;
        let t88 = t87 * t87;
        let t89 = t88 * t4;
        let t90 = t89 * rho1;
        let t91 = t86 + t90;
        let t92 = t82 * t91;
        let t95 = param_omega_8;
        let t96 = t95 * t27;
        let t97 = rho0 * rho0;
        let t99 = 1.0 / t23 / t97;
        let t100 = sigma0 * t99;
        let t101 = t55 * t55;
        let t102 = t101 * t56;
        let t103 = t100 * t102;
        let t104 = rho1 * rho1;
        let t106 = 1.0 / t25 / t104;
        let t107 = sigma2 * t106;
        let t108 = t67 * t67;
        let t109 = t108 * t56;
        let t110 = t107 * t109;
        let t112 = t103 / 8.0 + t110 / 8.0;
        let t115 = param_omega_9;
        let t116 = t115 * t91;
        let t119 = param_omega_10;
        let t120 = t97 + t104;
        let t121 = t119 * t120;
        let t124 = param_omega_11;
        let t125 = t124 * t27;
        let t129 = sigma0 + 2.0 * sigma1 + sigma2;
        let t130 = t46 * t46;
        let t131 = pow_1_3(t46);
        let t132 = t131 * t131;
        let t134 = 1.0 / t132 / t130;
        let t136 = t103 / 4.0 + t110 / 4.0 - t129 * t134;
        let t138 = param_omega_12;
        let t139 = t138 * t91;
        let t141 = param_omega_13;
        let t142 = t141 * t120;
        let t144 = param_omega_14;
        let t145 = t144 * t6;
        let t146 = t45 * t45;
        let t147 = 1.0 / t130;
        let t148 = t146 * t147;
        let t150 = param_omega_15;
        let t151 = t150 * t13;
        let t153 = param_omega_16;
        let t154 = t153 * t20;
        let t156 = param_omega_17;
        let t157 = t156 * t27;
        let t159 = param_omega_18;
        let t160 = f64::powf(rho0, 0.10833333333333333333e1);
        let t161 = f64::powf(rho1, 0.10833333333333333333e1);
        let t164 = t1 * t6 + t8 * t13 + t15 * t20 + t22 * t27 + t41 * t71 / 2.0 + t75 * t71 / 2.0 + t79 * t71 / 2.0 + t92 * t71 / 2.0 + t96 * t112 / 2.0 + t116 * t112 / 2.0 + t121 * t112 / 2.0 + t125 * t136 + t139 * t136 + t142 * t136 + t145 * t148 + t151 * t148 + t154 * t148 + t157 * t148 + t159 * (t160 + t161);
        let tzk0 = t164 * t47;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (109 lines) ---
        let t173 = f64::powf(rho0, 0.833333333333333333e-1);
        let t177 = 1.0 / t9 / t97;
        let t178 = t42 * t177;
        let t181 = t45 * t147;
        let t182 = t47 - t181;
        let t185 = piecewise3(t50, 0.0, 4.0 / 3.0 * t53 * t182);
        let t186 = t185 * t57;
        let t189 = -t182;
        let t192 = piecewise3(t64, 0.0, 4.0 / 3.0 * t65 * t189);
        let t193 = t192 * t57;
        let t196 = -t178 * t58 / 3.0 + t44 * t186 / 4.0 + t62 * t193 / 4.0;
        let t199 = t29 * t33;
        let t204 = t74 * t16;
        let t209 = t78 * t23;
        let t214 = t82 * t85;
        let t217 = t97 * rho0;
        let t219 = 1.0 / t23 / t217;
        let t220 = sigma0 * t219;
        let t221 = t220 * t102;
        let t223 = t55 * t56;
        let t224 = t223 * t185;
        let t225 = t100 * t224;
        let t227 = t67 * t56;
        let t228 = t227 * t192;
        let t229 = t107 * t228;
        let t231 = -t221 / 3.0 + t225 / 4.0 + t229 / 4.0;
        let t234 = t95 * t23;
        let t239 = t115 * t85;
        let t244 = 7.0 / 6.0 * t1 * t2 + 4.0 / 3.0 * t8 * t9 + 3.0 / 2.0 * t15 * t16 + 5.0 / 3.0 * t22 * t23 + 0.10833333333333333333e1 * t159 * t173 + t41 * t196 / 2.0 + 17.0 / 24.0 * t199 * t71 + t75 * t196 / 2.0 + 3.0 / 4.0 * t204 * t71 + t79 * t196 / 2.0 + 5.0 / 6.0 * t209 * t71 + t92 * t196 / 2.0 + 11.0 / 12.0 * t214 * t71 + t96 * t231 / 2.0 + 5.0 / 6.0 * t234 * t112 + t116 * t231 / 2.0 + 11.0 / 12.0 * t239 * t112 + t121 * t231 / 2.0;
        let t245 = t119 * rho0;
        let t250 = t130 * t46;
        let t252 = 1.0 / t132 / t250;
        let t254 = 8.0 / 3.0 * t129 * t252;
        let t255 = -2.0 / 3.0 * t221 + t225 / 2.0 + t229 / 2.0 + t254;
        let t257 = t124 * t23;
        let t261 = t138 * t85;
        let t265 = t141 * rho0;
        let t268 = 1.0 / t250;
        let t269 = t146 * t268;
        let t271 = 2.0 * t145 * t269;
        let t273 = 2.0 * t151 * t181;
        let t275 = 2.0 * t151 * t269;
        let t277 = 2.0 * t154 * t181;
        let t279 = 2.0 * t154 * t269;
        let t281 = 2.0 * t157 * t181;
        let t283 = 2.0 * t157 * t269;
        let t285 = 2.0 * t145 * t181;
        let t286 = t144 * t2;
        let t289 = t150 * t9;
        let t292 = t153 * t16;
        let t295 = t156 * t23;
        let t298 = t245 * t112 + t125 * t255 + 5.0 / 3.0 * t257 * t136 + t139 * t255 + 11.0 / 6.0 * t261 * t136 + t142 * t255 + 2.0 * t265 * t136 - t271 + t273 - t275 + t277 - t279 + t281 - t283 + t285 + 7.0 / 6.0 * t286 * t148 + 4.0 / 3.0 * t289 * t148 + 3.0 / 2.0 * t292 * t148 + 5.0 / 3.0 * t295 * t148;
        let tvrho0 = t244 + t298;
        vrho[ip * 2] += tvrho0;
        let t299 = -t47 - t181;
        let t302 = piecewise3(t50, 0.0, 4.0 / 3.0 * t53 * t299);
        let t303 = t302 * t57;
        let t307 = 1.0 / t11 / t104;
        let t308 = t60 * t307;
        let t311 = -t299;
        let t314 = piecewise3(t64, 0.0, 4.0 / 3.0 * t65 * t311);
        let t315 = t314 * t57;
        let t318 = t44 * t303 / 4.0 - t308 * t68 / 3.0 + t62 * t315 / 4.0;
        let t321 = t29 * t38;
        let t326 = t74 * t18;
        let t331 = t78 * t25;
        let t336 = t82 * t89;
        let t339 = t223 * t302;
        let t340 = t100 * t339;
        let t342 = t104 * rho1;
        let t344 = 1.0 / t25 / t342;
        let t345 = sigma2 * t344;
        let t346 = t345 * t109;
        let t348 = t227 * t314;
        let t349 = t107 * t348;
        let t351 = t340 / 4.0 - t346 / 3.0 + t349 / 4.0;
        let t354 = t95 * t25;
        let t359 = t115 * t89;
        let t364 = t119 * rho1;
        let t369 = t340 / 2.0 - 2.0 / 3.0 * t346 + t349 / 2.0 + t254;
        let t371 = t124 * t25;
        let t375 = t138 * t89;
        let t378 = t41 * t318 / 2.0 + 17.0 / 24.0 * t321 * t71 + t75 * t318 / 2.0 + 3.0 / 4.0 * t326 * t71 + t79 * t318 / 2.0 + 5.0 / 6.0 * t331 * t71 + t92 * t318 / 2.0 + 11.0 / 12.0 * t336 * t71 + t96 * t351 / 2.0 + 5.0 / 6.0 * t354 * t112 + t116 * t351 / 2.0 + 11.0 / 12.0 * t359 * t112 + t121 * t351 / 2.0 + t364 * t112 + t125 * t369 + 5.0 / 3.0 * t371 * t136 + t139 * t369 + 11.0 / 6.0 * t375 * t136;
        let t380 = t141 * rho1;
        let t383 = t144 * t4;
        let t386 = t150 * t11;
        let t389 = t153 * t18;
        let t392 = t156 * t25;
        let t403 = f64::powf(rho1, 0.833333333333333333e-1);
        let t406 = t142 * t369 + 2.0 * t380 * t136 - t271 - t273 - t275 - t277 - t279 - t281 - t283 + 7.0 / 6.0 * t383 * t148 + 4.0 / 3.0 * t386 * t148 + 3.0 / 2.0 * t389 * t148 + 5.0 / 3.0 * t392 * t148 - t285 + 7.0 / 6.0 * t1 * t4 + 4.0 / 3.0 * t8 * t11 + 3.0 / 2.0 * t15 * t18 + 5.0 / 3.0 * t22 * t25 + 0.10833333333333333333e1 * t159 * t403;
        let tvrho1 = t378 + t406;
        vrho[ip * 2 + 1] += tvrho1;
        let t407 = 1.0 / t42;
        let t408 = t41 * t407;
        let t410 = t43 * t55 * t57;
        let t413 = t75 * t407;
        let t416 = t79 * t407;
        let t419 = t92 * t407;
        let t423 = t99 * t101 * t56;
        let t431 = t423 / 4.0 - t134;
        let tvsigma0 = t408 * t410 / 16.0 + t413 * t410 / 16.0 + t416 * t410 / 16.0 + t419 * t410 / 16.0 + t96 * t423 / 16.0 + t116 * t423 / 16.0 + t121 * t423 / 16.0 + t125 * t431 + t139 * t431 + t142 * t431;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -2.0 * t125 * t134 - 2.0 * t139 * t134 - 2.0 * t142 * t134;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t439 = 1.0 / t60;
        let t440 = t41 * t439;
        let t442 = t61 * t67 * t57;
        let t445 = t75 * t439;
        let t448 = t79 * t439;
        let t451 = t92 * t439;
        let t455 = t106 * t108 * t56;
        let t463 = t455 / 4.0 - t134;
        let tvsigma2 = t440 * t442 / 16.0 + t445 * t442 / 16.0 + t448 * t442 / 16.0 + t451 * t442 / 16.0 + t96 * t455 / 16.0 + t116 * t455 / 16.0 + t121 * t455 / 16.0 + t125 * t463 + t139 * t463 + t142 * t463;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
