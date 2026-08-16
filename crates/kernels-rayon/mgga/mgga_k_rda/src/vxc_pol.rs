//! MGGA_K_RDA vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_rda.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_rda_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_A0: f64,
    param_A1: f64,
    param_A2: f64,
    param_A3: f64,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t45 = t38 * sigma0 * t43;
        let t47 = t33 * t33;
        let t49 = 1.0 / t35 / t34;
        let t50 = t47 * t49;
        let t51 = sigma0 * sigma0;
        let t52 = t39 * t39;
        let t53 = t52 * rho0;
        let t55 = 1.0 / t40 / t53;
        let t57 = t50 * t51 * t55;
        let t58 = param_a * t47;
        let t59 = lapl0 * lapl0;
        let t60 = t49 * t59;
        let t61 = t39 * rho0;
        let t63 = 1.0 / t40 / t61;
        let t64 = t60 * t63;
        let t66 = t58 * t64 + t57;
        let t68 = f64::sqrt(t66);
        let t71 = 1.0 + param_beta1 * t68 / 24.0;
        let t72 = t71 * t71;
        let t73 = 1.0 / t72;
        let t76 = param_b * t47;
        let t78 = t76 * t64 + t57;
        let t79 = t78 * t78;
        let t81 = f64::sqrt(t78);
        let t84 = 1.0 + param_beta2 * t81 / 24.0;
        let t85 = t84 * t84;
        let t86 = t85 * t85;
        let t87 = 1.0 / t86;
        let t90 = param_c * t33;
        let t91 = t37 * lapl0;
        let t93 = 1.0 / t41 / rho0;
        let t97 = t90 * t91 * t93 / 24.0 + t45 / 24.0;
        let t98 = param_A3 * t97;
        let t100 = param_beta3 * t97 + 1.0;
        let t101 = 1.0 / t100;
        let t103 = 5.0 / 72.0 * t45 + param_A0 + param_A1 * t66 * t73 / 576.0 + param_A2 * t79 * t87 / 331776.0 + t98 * t101;
        let t107 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t103);
        let t108 = rho1 <= dens_threshold;
        let t109 = -t18;
        let t111 = piecewise5(t16, t13, t12, t17, t109 * t9);
        let t112 = 1.0 + t111;
        let t113 = t112 <= zeta_threshold;
        let t114 = pow_1_3(t112);
        let t115 = t114 * t114;
        let t117 = piecewise3(t113, t25, t115 * t112);
        let t118 = t117 * t31;
        let t119 = rho1 * rho1;
        let t120 = pow_1_3(rho1);
        let t121 = t120 * t120;
        let t123 = 1.0 / t121 / t119;
        let t125 = t38 * sigma2 * t123;
        let t127 = sigma2 * sigma2;
        let t128 = t119 * t119;
        let t129 = t128 * rho1;
        let t131 = 1.0 / t120 / t129;
        let t133 = t50 * t127 * t131;
        let t134 = lapl1 * lapl1;
        let t135 = t49 * t134;
        let t136 = t119 * rho1;
        let t138 = 1.0 / t120 / t136;
        let t139 = t135 * t138;
        let t141 = t58 * t139 + t133;
        let t143 = f64::sqrt(t141);
        let t146 = 1.0 + param_beta1 * t143 / 24.0;
        let t147 = t146 * t146;
        let t148 = 1.0 / t147;
        let t152 = t76 * t139 + t133;
        let t153 = t152 * t152;
        let t155 = f64::sqrt(t152);
        let t158 = 1.0 + param_beta2 * t155 / 24.0;
        let t159 = t158 * t158;
        let t160 = t159 * t159;
        let t161 = 1.0 / t160;
        let t164 = t37 * lapl1;
        let t166 = 1.0 / t121 / rho1;
        let t170 = t90 * t164 * t166 / 24.0 + t125 / 24.0;
        let t171 = param_A3 * t170;
        let t173 = param_beta3 * t170 + 1.0;
        let t174 = 1.0 / t173;
        let t176 = 5.0 / 72.0 * t125 + param_A0 + param_A1 * t141 * t148 / 576.0 + param_A2 * t153 * t161 / 331776.0 + t171 * t174;
        let t180 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t118 * t176);
        let tzk0 = t107 + t180;
        zk[ip] += tzk0;
        let t181 = t8 * t8;
        let t182 = 1.0 / t181;
        let t183 = t18 * t182;
        let t185 = piecewise5(t12, 0.0, t16, 0.0, t9 - t183);
        let t188 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t185);
        let t189 = t188 * t31;
        let t193 = 1.0 / t30;
        let t194 = t29 * t193;
        let t197 = t7 * t194 * t103 / 10.0;
        let t199 = 1.0 / t41 / t61;
        let t201 = t38 * sigma0 * t199;
        let t203 = t52 * t39;
        let t205 = 1.0 / t40 / t203;
        let t208 = 16.0 / 3.0 * t50 * t51 * t205;
        let t210 = 1.0 / t40 / t52;
        let t211 = t60 * t210;
        let t214 = -t208 - 10.0 / 3.0 * t58 * t211;
        let t218 = param_A1 * t68;
        let t220 = 1.0 / t72 / t71;
        let t221 = t220 * param_beta1;
        let t225 = param_A2 * t78;
        let t228 = -t208 - 10.0 / 3.0 * t76 * t211;
        let t232 = t81 * t78;
        let t233 = param_A2 * t232;
        let t235 = 1.0 / t86 / t84;
        let t236 = t235 * param_beta2;
        let t244 = -t201 / 9.0 - 5.0 / 72.0 * t90 * t91 * t43;
        let t245 = param_A3 * t244;
        let t247 = t100 * t100;
        let t248 = 1.0 / t247;
        let t249 = t248 * param_beta3;
        let t250 = t249 * t244;
        let t252 = -5.0 / 27.0 * t201 + param_A1 * t214 * t73 / 576.0 - t218 * t221 * t214 / 13824.0 + t225 * t87 * t228 / 165888.0 - t233 * t236 * t228 / 3981312.0 + t245 * t101 - t98 * t250;
        let t257 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t189 * t103 + t197 + 3.0 / 20.0 * t7 * t32 * t252);
        let t258 = t109 * t182;
        let t260 = piecewise5(t16, 0.0, t12, 0.0, -t9 - t258);
        let t263 = piecewise3(t113, 0.0, 5.0 / 3.0 * t115 * t260);
        let t264 = t263 * t31;
        let t268 = t117 * t193;
        let t271 = t7 * t268 * t176 / 10.0;
        let t273 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t264 * t176 + t271);
        let tvrho0 = t107 + t180 + t8 * (t257 + t273);
        vrho[ip * 2] += tvrho0;
        let t277 = piecewise5(t12, 0.0, t16, 0.0, -t9 - t183);
        let t280 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t277);
        let t281 = t280 * t31;
        let t286 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t281 * t103 + t197);
        let t288 = piecewise5(t16, 0.0, t12, 0.0, t9 - t258);
        let t291 = piecewise3(t113, 0.0, 5.0 / 3.0 * t115 * t288);
        let t292 = t291 * t31;
        let t297 = 1.0 / t121 / t136;
        let t299 = t38 * sigma2 * t297;
        let t301 = t128 * t119;
        let t303 = 1.0 / t120 / t301;
        let t306 = 16.0 / 3.0 * t50 * t127 * t303;
        let t308 = 1.0 / t120 / t128;
        let t309 = t135 * t308;
        let t312 = -t306 - 10.0 / 3.0 * t58 * t309;
        let t316 = param_A1 * t143;
        let t318 = 1.0 / t147 / t146;
        let t319 = t318 * param_beta1;
        let t323 = param_A2 * t152;
        let t326 = -t306 - 10.0 / 3.0 * t76 * t309;
        let t330 = t155 * t152;
        let t331 = param_A2 * t330;
        let t333 = 1.0 / t160 / t158;
        let t334 = t333 * param_beta2;
        let t342 = -t299 / 9.0 - 5.0 / 72.0 * t90 * t164 * t123;
        let t343 = param_A3 * t342;
        let t345 = t173 * t173;
        let t346 = 1.0 / t345;
        let t347 = t346 * param_beta3;
        let t348 = t347 * t342;
        let t350 = -5.0 / 27.0 * t299 + param_A1 * t312 * t148 / 576.0 - t316 * t319 * t312 / 13824.0 + t323 * t161 * t326 / 165888.0 - t331 * t334 * t326 / 3981312.0 + t343 * t174 - t171 * t348;
        let t355 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t292 * t176 + t271 + 3.0 / 20.0 * t7 * t118 * t350);
        let tvrho1 = t107 + t180 + t8 * (t286 + t355);
        vrho[ip * 2 + 1] += tvrho1;
        let t360 = param_A1 * t47;
        let t361 = t360 * t49;
        let t362 = sigma0 * t55;
        let t366 = t218 * t221;
        let t367 = t50 * t362;
        let t370 = t225 * t87;
        let t373 = t233 * t236;
        let t376 = param_A3 * t33;
        let t377 = t37 * t43;
        let t378 = t377 * t101;
        let t381 = t98 * t248;
        let t382 = param_beta3 * t33;
        let t386 = 5.0 / 72.0 * t38 * t43 + t361 * t362 * t73 / 288.0 - t366 * t367 / 6912.0 + t370 * t367 / 82944.0 - t373 * t367 / 1990656.0 + t376 * t378 / 24.0 - t381 * t382 * t377 / 24.0;
        let t390 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t386);
        let tvsigma0 = t8 * t390;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t393 = sigma2 * t131;
        let t397 = t316 * t319;
        let t398 = t50 * t393;
        let t401 = t323 * t161;
        let t404 = t331 * t334;
        let t407 = t37 * t123;
        let t408 = t407 * t174;
        let t411 = t171 * t346;
        let t415 = 5.0 / 72.0 * t38 * t123 + t361 * t393 * t148 / 288.0 - t397 * t398 / 6912.0 + t401 * t398 / 82944.0 - t404 * t398 / 1990656.0 + t376 * t408 / 24.0 - t411 * t382 * t407 / 24.0;
        let t419 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t118 * t415);
        let tvsigma2 = t8 * t419;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t420 = param_A1 * param_a;
        let t421 = t420 * t47;
        let t422 = t49 * lapl0;
        let t427 = t422 * t63;
        let t431 = t87 * param_b;
        let t432 = t225 * t431;
        let t433 = lapl0 * t63;
        let t434 = t50 * t433;
        let t440 = param_A3 * param_c;
        let t441 = t440 * t33;
        let t442 = t37 * t93;
        let t446 = t98 * t249;
        let t447 = t90 * t442;
        let t450 = t421 * t422 * t63 * t73 / 288.0 - t366 * t58 * t427 / 6912.0 + t432 * t434 / 82944.0 - t373 * t76 * t427 / 1990656.0 + t441 * t442 * t101 / 24.0 - t446 * t447 / 24.0;
        let t454 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t450);
        let tvlapl0 = t8 * t454;
        vlapl[ip * 2] += tvlapl0;
        let t455 = t49 * lapl1;
        let t460 = t455 * t138;
        let t464 = t161 * param_b;
        let t465 = t323 * t464;
        let t466 = lapl1 * t138;
        let t467 = t50 * t466;
        let t473 = t37 * t166;
        let t477 = t171 * t347;
        let t478 = t90 * t473;
        let t481 = t421 * t455 * t138 * t148 / 288.0 - t397 * t58 * t460 / 6912.0 + t465 * t467 / 82944.0 - t404 * t76 * t460 / 1990656.0 + t441 * t473 * t174 / 24.0 - t477 * t478 / 24.0;
        let t485 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t118 * t481);
        let tvlapl1 = t8 * t485;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
