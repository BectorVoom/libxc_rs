//! GGA_XC_TH1 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th1.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th1_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
    param_omega_20: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    param_omega_18: f64,
    param_omega_19: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = param_omega_0;
        let t2 = rmath::pow(rho0, 1.0 / 6.0);
        let t3 = t2 * rho0;
        let t4 = rmath::pow(rho1, 1.0 / 6.0);
        let t5 = t4 * rho1;
        let t6 = t3 + t5;
        let t8 = param_omega_1;
        let t9 = pow_1_3(rho0);
        let t10 = t9 * rho0;
        let t11 = pow_1_3(rho1);
        let t12 = t11 * rho1;
        let t13 = t10 + t12;
        let t15 = param_omega_2;
        let t16 = rmath::sqrt(rho0);
        let t17 = t16 * rho0;
        let t18 = rmath::sqrt(rho1);
        let t19 = t18 * rho1;
        let t20 = t17 + t19;
        let t22 = param_omega_3;
        let t23 = t9 * t9;
        let t24 = t23 * rho0;
        let t25 = t11 * t11;
        let t26 = t25 * rho1;
        let t27 = t24 + t26;
        let t29 = param_omega_4;
        let t30 = t29 * t13;
        let t31 = rmath::sqrt(sigma0);
        let t32 = 1.0 / t10;
        let t33 = t31 * t32;
        let t34 = rho0 - rho1;
        let t35 = rho0 + rho1;
        let t36 = 1.0 / t35;
        let t37 = t34 * t36;
        let t38 = 1.0 + t37;
        let t39 = t38 <= zeta_threshold;
        let t40 = pow_1_3(zeta_threshold);
        let t41 = t40 * zeta_threshold;
        let t42 = pow_1_3(t38);
        let t44 = piecewise3(t39, t41, t42 * t38);
        let t45 = M_CBRT2;
        let t46 = t45 * t45;
        let t47 = t44 * t46;
        let t49 = rmath::sqrt(sigma2);
        let t50 = 1.0 / t12;
        let t51 = t49 * t50;
        let t52 = 1.0 - t37;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t41, t54 * t52);
        let t57 = t56 * t46;
        let t60 = t33 * t47 / 4.0 + t51 * t57 / 4.0;
        let t63 = param_omega_5;
        let t64 = t63 * t20;
        let t67 = param_omega_6;
        let t68 = t67 * t27;
        let t71 = param_omega_7;
        let t72 = t2 * t2;
        let t73 = t72 * t72;
        let t74 = t73 * t2;
        let t75 = t74 * rho0;
        let t76 = t4 * t4;
        let t77 = t76 * t76;
        let t78 = t77 * t4;
        let t79 = t78 * rho1;
        let t80 = t75 + t79;
        let t81 = t71 * t80;
        let t84 = param_omega_8;
        let t85 = t84 * t20;
        let t86 = rho0 * rho0;
        let t88 = 1.0 / t23 / t86;
        let t89 = sigma0 * t88;
        let t90 = t44 * t44;
        let t91 = t90 * t45;
        let t92 = t89 * t91;
        let t93 = rho1 * rho1;
        let t95 = 1.0 / t25 / t93;
        let t96 = sigma2 * t95;
        let t97 = t56 * t56;
        let t98 = t97 * t45;
        let t99 = t96 * t98;
        let t101 = t92 / 8.0 + t99 / 8.0;
        let t104 = param_omega_9;
        let t105 = t104 * t27;
        let t109 = param_omega_10;
        let t110 = t109 * t80;
        let t113 = param_omega_11;
        let t114 = t86 + t93;
        let t115 = t113 * t114;
        let t118 = param_omega_12;
        let t119 = t118 * t20;
        let t123 = sigma0 + 2.0 * sigma1 + sigma2;
        let t124 = t35 * t35;
        let t125 = pow_1_3(t35);
        let t126 = t125 * t125;
        let t128 = 1.0 / t126 / t124;
        let t130 = t92 / 4.0 + t99 / 4.0 - t123 * t128;
        let t132 = param_omega_13;
        let t133 = t132 * t27;
        let t135 = param_omega_14;
        let t136 = t135 * t80;
        let t138 = param_omega_15;
        let t139 = t138 * t114;
        let t141 = param_omega_16;
        let t142 = t141 * t6;
        let t143 = t34 * t34;
        let t144 = 1.0 / t124;
        let t145 = t143 * t144;
        let t147 = param_omega_17;
        let t148 = t147 * t13;
        let t150 = param_omega_18;
        let t151 = t150 * t20;
        let t153 = param_omega_19;
        let t154 = t153 * t27;
        let t156 = param_omega_20;
        let t158 = t110 * t101 / 2.0 + t115 * t101 / 2.0 + t119 * t130 + t133 * t130 + t136 * t130 + t139 * t130 + t142 * t145 + t148 * t145 + t151 * t145 + t154 * t145 + t156 * t35;
        let tzk0 = (t1 * t6 + t8 * t13 + t15 * t20 + t22 * t27 + t30 * t60 / 2.0 + t64 * t60 / 2.0 + t68 * t60 / 2.0 + t81 * t60 / 2.0 + t85 * t101 / 2.0 + t105 * t101 / 2.0 + t158) * t36;
        zk[ip] += tzk0;
        let t168 = t118 * t16;
        let t171 = t86 * rho0;
        let t173 = 1.0 / t23 / t171;
        let t174 = sigma0 * t173;
        let t175 = t174 * t91;
        let t177 = t44 * t45;
        let t178 = t34 * t144;
        let t179 = t36 - t178;
        let t182 = piecewise3(t39, 0.0, 4.0 / 3.0 * t42 * t179);
        let t183 = t177 * t182;
        let t184 = t89 * t183;
        let t186 = t56 * t45;
        let t187 = -t179;
        let t190 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t187);
        let t191 = t186 * t190;
        let t192 = t96 * t191;
        let t194 = t124 * t35;
        let t196 = 1.0 / t126 / t194;
        let t198 = 8.0 / 3.0 * t123 * t196;
        let t199 = -2.0 / 3.0 * t175 + t184 / 2.0 + t192 / 2.0 + t198;
        let t201 = t132 * t23;
        let t205 = t135 * t74;
        let t209 = t138 * rho0;
        let t213 = 1.0 / t9 / t86;
        let t214 = t31 * t213;
        let t217 = t182 * t46;
        let t220 = t190 * t46;
        let t223 = -t214 * t47 / 3.0 + t33 * t217 / 4.0 + t51 * t220 / 4.0;
        let t226 = t29 * t9;
        let t231 = t63 * t16;
        let t236 = t67 * t23;
        let t241 = t71 * t74;
        let t244 = t156 + 7.0 / 6.0 * t1 * t2 + 4.0 / 3.0 * t8 * t9 + 3.0 / 2.0 * t15 * t16 + 5.0 / 3.0 * t22 * t23 + 3.0 / 2.0 * t168 * t130 + t133 * t199 + 5.0 / 3.0 * t201 * t130 + t136 * t199 + 11.0 / 6.0 * t205 * t130 + t139 * t199 + 2.0 * t209 * t130 + t30 * t223 / 2.0 + 2.0 / 3.0 * t226 * t60 + t64 * t223 / 2.0 + 3.0 / 4.0 * t231 * t60 + t68 * t223 / 2.0 + 5.0 / 6.0 * t236 * t60 + t81 * t223 / 2.0 + 11.0 / 12.0 * t241 * t60;
        let t248 = -t175 / 3.0 + t184 / 4.0 + t192 / 4.0;
        let t251 = t84 * t16;
        let t256 = t104 * t23;
        let t261 = t109 * t74;
        let t266 = t113 * rho0;
        let t269 = 1.0 / t194;
        let t270 = t143 * t269;
        let t272 = 2.0 * t154 * t270;
        let t275 = 2.0 * t142 * t178;
        let t277 = 2.0 * t142 * t270;
        let t279 = 2.0 * t148 * t178;
        let t281 = 2.0 * t148 * t270;
        let t283 = 2.0 * t151 * t178;
        let t285 = 2.0 * t151 * t270;
        let t287 = 2.0 * t154 * t178;
        let t288 = t141 * t2;
        let t291 = t147 * t9;
        let t294 = t150 * t16;
        let t297 = t153 * t23;
        let t300 = t275 - t277 + t279 - t281 + t283 - t285 + t287 + 7.0 / 6.0 * t288 * t145 + 4.0 / 3.0 * t291 * t145 + 3.0 / 2.0 * t294 * t145 + 5.0 / 3.0 * t297 * t145;
        let tvrho0 = t244 + t85 * t248 / 2.0 + 3.0 / 4.0 * t251 * t101 + t105 * t248 / 2.0 + 5.0 / 6.0 * t256 * t101 + t110 * t248 / 2.0 + 11.0 / 12.0 * t261 * t101 + t115 * t248 / 2.0 + t266 * t101 + t119 * t199 - t272 + t300;
        vrho[ip * 2] += tvrho0;
        let t310 = -t36 - t178;
        let t313 = piecewise3(t39, 0.0, 4.0 / 3.0 * t42 * t310);
        let t314 = t177 * t313;
        let t315 = t89 * t314;
        let t317 = t93 * rho1;
        let t319 = 1.0 / t25 / t317;
        let t320 = sigma2 * t319;
        let t321 = t320 * t98;
        let t323 = -t310;
        let t326 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t323);
        let t327 = t186 * t326;
        let t328 = t96 * t327;
        let t330 = t315 / 2.0 - 2.0 / 3.0 * t321 + t328 / 2.0 + t198;
        let t332 = t132 * t25;
        let t336 = t135 * t78;
        let t340 = t138 * rho1;
        let t343 = t313 * t46;
        let t347 = 1.0 / t11 / t93;
        let t348 = t49 * t347;
        let t351 = t326 * t46;
        let t354 = t33 * t343 / 4.0 - t348 * t57 / 3.0 + t51 * t351 / 4.0;
        let t357 = t29 * t11;
        let t362 = t63 * t18;
        let t367 = t67 * t25;
        let t372 = t71 * t78;
        let t378 = t315 / 4.0 - t321 / 3.0 + t328 / 4.0;
        let t381 = t156 + 7.0 / 6.0 * t1 * t4 + 4.0 / 3.0 * t8 * t11 + 3.0 / 2.0 * t15 * t18 + 5.0 / 3.0 * t22 * t25 + t133 * t330 + 5.0 / 3.0 * t332 * t130 + t136 * t330 + 11.0 / 6.0 * t336 * t130 + t139 * t330 + 2.0 * t340 * t130 + t30 * t354 / 2.0 + 2.0 / 3.0 * t357 * t60 + t64 * t354 / 2.0 + 3.0 / 4.0 * t362 * t60 + t68 * t354 / 2.0 + 5.0 / 6.0 * t367 * t60 + t81 * t354 / 2.0 + 11.0 / 12.0 * t372 * t60 + t85 * t378 / 2.0;
        let t382 = t84 * t18;
        let t387 = t104 * t25;
        let t392 = t109 * t78;
        let t397 = t113 * rho1;
        let t400 = t118 * t18;
        let t404 = t141 * t4;
        let t407 = t147 * t11;
        let t410 = t150 * t18;
        let t413 = t153 * t25;
        let t416 = 7.0 / 6.0 * t404 * t145 + 4.0 / 3.0 * t407 * t145 + 3.0 / 2.0 * t410 * t145 + 5.0 / 3.0 * t413 * t145 - t275 - t277 - t279 - t281 - t283 - t285 - t287;
        let tvrho1 = t381 + 3.0 / 4.0 * t382 * t101 + t105 * t378 / 2.0 + 5.0 / 6.0 * t387 * t101 + t110 * t378 / 2.0 + 11.0 / 12.0 * t392 * t101 + t115 * t378 / 2.0 + t397 * t101 + t119 * t330 + 3.0 / 2.0 * t400 * t130 - t272 + t416;
        vrho[ip * 2 + 1] += tvrho1;
        let t418 = 1.0 / t31;
        let t419 = t30 * t418;
        let t421 = t32 * t44 * t46;
        let t424 = t64 * t418;
        let t427 = t68 * t418;
        let t430 = t81 * t418;
        let t434 = t88 * t90 * t45;
        let t444 = t434 / 4.0 - t128;
        let tvsigma0 = t419 * t421 / 16.0 + t424 * t421 / 16.0 + t427 * t421 / 16.0 + t430 * t421 / 16.0 + t85 * t434 / 16.0 + t105 * t434 / 16.0 + t110 * t434 / 16.0 + t115 * t434 / 16.0 + t119 * t444 + t133 * t444 + t136 * t444 + t139 * t444;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -2.0 * t119 * t128 - 2.0 * t133 * t128 - 2.0 * t136 * t128 - 2.0 * t139 * t128;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t454 = 1.0 / t49;
        let t455 = t30 * t454;
        let t457 = t50 * t56 * t46;
        let t460 = t64 * t454;
        let t463 = t68 * t454;
        let t466 = t81 * t454;
        let t470 = t95 * t97 * t45;
        let t480 = t470 / 4.0 - t128;
        let tvsigma2 = t455 * t457 / 16.0 + t460 * t457 / 16.0 + t463 * t457 / 16.0 + t466 * t457 / 16.0 + t85 * t470 / 16.0 + t105 * t470 / 16.0 + t110 * t470 / 16.0 + t115 * t470 / 16.0 + t119 * t480 + t133 * t480 + t136 * t480 + t139 * t480;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
