//! MGGA_X_MSB fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_msb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_msb_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    param_b: f64,
    param_c: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t36 = 5.0 / 972.0 * t26 * t34;
        let t37 = param_kappa + t36;
        let t41 = param_kappa * (1.0 - param_kappa / t37);
        let t42 = tau[ip] * t28;
        let t44 = 1.0 / t31 / rho[ip];
        let t45 = t42 * t44;
        let t47 = t45 - t34 / 8.0;
        let t48 = t47 * t47;
        let t49 = t21 * t21;
        let t52 = t45 + 3.0 / 10.0 * t49 * t24;
        let t53 = t52 * t52;
        let t54 = 1.0 / t53;
        let t57 = -4.0 * t48 * t54 + 1.0;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = t48 * t47;
        let t61 = t53 * t52;
        let t62 = 1.0 / t61;
        let t65 = t48 * t48;
        let t67 = param_b * t65 * t48;
        let t68 = t53 * t53;
        let t70 = 1.0 / t68 / t53;
        let t73 = 8.0 * t60 * t62 + 64.0 * t67 * t70 + 1.0;
        let t74 = 1.0 / t73;
        let t75 = t59 * t74;
        let t76 = param_kappa + t36 + param_c;
        let t81 = param_kappa * (1.0 - param_kappa / t76) - t41;
        let t83 = t75 * t81 + t41 + 1.0;
        let t87 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t83);
        let tzk0 = 2.0 * t87;
        zk[ip] += tzk0;
        let t89 = t18 / t31;
        let t93 = param_kappa * param_kappa;
        let t94 = t37 * t37;
        let t97 = t93 / t94 * t21;
        let t98 = t25 * sigma[ip];
        let t99 = t30 * rho[ip];
        let t101 = 1.0 / t31 / t99;
        let t102 = t28 * t101;
        let t103 = t98 * t102;
        let t104 = t97 * t103;
        let t106 = t58 * t74;
        let t107 = t47 * t54;
        let t108 = t42 * t33;
        let t112 = -5.0 / 3.0 * t108 + t29 * t101 / 3.0;
        let t115 = t48 * t62;
        let t118 = -8.0 * t107 * t112 - 40.0 / 3.0 * t115 * t108;
        let t119 = t81 * t118;
        let t122 = t73 * t73;
        let t123 = 1.0 / t122;
        let t124 = t59 * t123;
        let t127 = 1.0 / t68;
        let t128 = t60 * t127;
        let t132 = param_b * t65 * t47;
        let t133 = t70 * t112;
        let t137 = 1.0 / t68 / t61;
        let t138 = t67 * t137;
        let t141 = 40.0 * t128 * t108 + 640.0 * t138 * t108 + 24.0 * t115 * t112 + 384.0 * t132 * t133;
        let t142 = t81 * t141;
        let t144 = t76 * t76;
        let t147 = t93 / t144 * t21;
        let t150 = -10.0 / 729.0 * t147 * t103 + 10.0 / 729.0 * t104;
        let t152 = -10.0 / 729.0 * t104 + 3.0 * t106 * t119 - t124 * t142 + t75 * t150;
        let t157 = piecewise3(t3, 0.0, -t7 * t89 * t83 / 8.0 - 3.0 / 8.0 * t7 * t20 * t152);
        let tvrho0 = 2.0 * rho[ip] * t157 + 2.0 * t87;
        vrho[ip] += tvrho0;
        let t160 = t25 * t28;
        let t161 = t160 * t33;
        let t162 = t97 * t161;
        let t164 = t106 * t81;
        let t165 = t28 * t33;
        let t166 = t107 * t165;
        let t169 = t115 * t165;
        let t171 = t70 * t28;
        let t173 = t132 * t171 * t33;
        let t175 = -3.0 * t169 - 48.0 * t173;
        let t176 = t81 * t175;
        let t180 = 5.0 / 972.0 * t147 * t161 - 5.0 / 972.0 * t162;
        let t182 = 5.0 / 972.0 * t162 + 3.0 * t164 * t166 - t124 * t176 + t75 * t180;
        let t186 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t182);
        let tvsigma0 = 2.0 * rho[ip] * t186;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t188 = t28 * t44;
        let t190 = t115 * t188;
        let t192 = -8.0 * t107 * t188 + 8.0 * t190;
        let t193 = t81 * t192;
        let t202 = t137 * t28;
        let t206 = 384.0 * t132 * t171 * t44 - 384.0 * t67 * t202 * t44 - 24.0 * t128 * t188 + 24.0 * t190;
        let t207 = t81 * t206;
        let t209 = 3.0 * t106 * t193 - t124 * t207;
        let t213 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t209);
        let tvtau0 = 2.0 * rho[ip] * t213;
        vtau[ip] += tvtau0;
        let t216 = t18 * t44;
        let t226 = t93 / t94 / t37 * t49;
        let t228 = 1.0 / t23 / t22;
        let t229 = sigma[ip] * sigma[ip];
        let t230 = t228 * t229;
        let t231 = t30 * t30;
        let t232 = t231 * t99;
        let t234 = 1.0 / t19 / t232;
        let t235 = t27 * t234;
        let t236 = t230 * t235;
        let t238 = 400.0 / 531441.0 * t226 * t236;
        let t240 = 1.0 / t31 / t231;
        let t241 = t28 * t240;
        let t242 = t98 * t241;
        let t244 = 110.0 / 2187.0 * t97 * t242;
        let t245 = t57 * t74;
        let t246 = t118 * t118;
        let t247 = t81 * t246;
        let t250 = t58 * t123;
        let t254 = t150 * t118;
        let t257 = t112 * t112;
        let t260 = t47 * t62;
        let t261 = t260 * t112;
        let t264 = t42 * t101;
        let t268 = 40.0 / 9.0 * t264 - 11.0 / 9.0 * t29 * t240;
        let t271 = t48 * t127;
        let t272 = tau[ip] * tau[ip];
        let t273 = t272 * t27;
        let t274 = t231 * rho[ip];
        let t276 = 1.0 / t19 / t274;
        let t277 = t273 * t276;
        let t282 = -8.0 * t257 * t54 - 160.0 / 3.0 * t261 * t108 - 8.0 * t107 * t268 - 400.0 / 3.0 * t271 * t277 + 320.0 / 9.0 * t115 * t264;
        let t283 = t81 * t282;
        let t287 = 1.0 / t122 / t73;
        let t288 = t59 * t287;
        let t289 = t141 * t141;
        let t290 = t81 * t289;
        let t293 = t150 * t141;
        let t298 = t271 * t112;
        let t304 = 1.0 / t68 / t52;
        let t305 = t60 * t304;
        let t310 = param_b * t65;
        let t311 = t70 * t257;
        let t314 = t132 * t137;
        let t315 = t112 * tau[ip];
        let t322 = t68 * t68;
        let t323 = 1.0 / t322;
        let t324 = t67 * t323;
        let t329 = 48.0 * t260 * t257 + 240.0 * t298 * t108 + 24.0 * t115 * t268 + 1600.0 / 3.0 * t305 * t277 - 320.0 / 3.0 * t128 * t264 + 1920.0 * t310 * t311 + 7680.0 * t314 * t315 * t165 + 384.0 * t132 * t70 * t268 + 44800.0 / 3.0 * t324 * t277 - 5120.0 / 3.0 * t138 * t264;
        let t335 = t93 / t144 / t76 * t49;
        let t340 = -400.0 / 531441.0 * t335 * t236 + 110.0 / 2187.0 * t147 * t242 + t238 - t244;
        let t342 = -6.0 * t250 * t119 * t141 - t124 * t81 * t329 + 6.0 * t106 * t254 + 3.0 * t106 * t283 - 2.0 * t124 * t293 + 6.0 * t245 * t247 + 2.0 * t288 * t290 + t75 * t340 - t238 + t244;
        let t347 = piecewise3(t3, 0.0, t7 * t216 * t83 / 12.0 - t7 * t89 * t152 / 4.0 - 3.0 / 8.0 * t7 * t20 * t342);
        let tv2rho20 = 2.0 * rho[ip] * t347 + 4.0 * t157;
        v2rho2[ip] += tv2rho20;
        let t353 = t228 * t27;
        let t354 = t231 * t30;
        let t356 = 1.0 / t19 / t354;
        let t358 = t353 * t356 * sigma[ip];
        let t360 = 50.0 / 177147.0 * t226 * t358;
        let t361 = t160 * t101;
        let t363 = 10.0 / 729.0 * t97 * t361;
        let t364 = t81 * t47;
        let t365 = t245 * t364;
        let t366 = t54 * t28;
        let t367 = t33 * t118;
        let t368 = t366 * t367;
        let t371 = t250 * t364;
        let t373 = t366 * t33 * t141;
        let t376 = t106 * t150;
        let t379 = t112 * t54;
        let t380 = t379 * t165;
        let t383 = t106 * t364;
        let t384 = t62 * t27;
        let t386 = t384 * t276 * tau[ip];
        let t389 = t107 * t102;
        let t398 = t150 * t175;
        let t400 = t165 * t112;
        let t401 = t260 * t400;
        let t403 = t27 * t276;
        let t404 = t403 * tau[ip];
        let t405 = t271 * t404;
        let t407 = t115 * t102;
        let t409 = t310 * t70;
        let t410 = t409 * t400;
        let t412 = t314 * t404;
        let t415 = t132 * t171 * t101;
        let t417 = -6.0 * t401 - 30.0 * t405 + 8.0 * t407 - 240.0 * t410 - 960.0 * t412 + 128.0 * t415;
        let t418 = t81 * t417;
        let t420 = t180 * t118;
        let t423 = t180 * t141;
        let t429 = 50.0 / 177147.0 * t335 * t358 - 10.0 / 729.0 * t147 * t361 - t360 + t363;
        let t431 = -3.0 * t250 * t176 * t118 + 2.0 * t288 * t176 * t141 + 3.0 * t106 * t420 - t124 * t398 - t124 * t418 - t124 * t423 + 3.0 * t164 * t380 - 8.0 * t164 * t389 + 3.0 * t376 * t166 + 6.0 * t365 * t368 - 3.0 * t371 * t373 + 20.0 * t383 * t386 + t75 * t429 + t360 - t363;
        let t436 = piecewise3(t3, 0.0, -t7 * t89 * t182 / 8.0 - 3.0 / 8.0 * t7 * t20 * t431);
        let tv2rhosigma0 = 2.0 * rho[ip] * t436 + 2.0 * t186;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t445 = t193 * t141;
        let t448 = t150 * t192;
        let t454 = 1.0 / t19 / t231;
        let t455 = t27 * t454;
        let t456 = t455 * tau[ip];
        let t460 = t188 * t112;
        let t461 = t260 * t460;
        let t463 = t271 * t456;
        let t466 = -8.0 * t379 * t188 - 160.0 / 3.0 * t260 * t456 + 40.0 / 3.0 * t166 + 16.0 * t461 + 80.0 * t463 - 40.0 / 3.0 * t169;
        let t467 = t81 * t466;
        let t476 = t150 * t206;
        let t499 = 640.0 * t67 * t202 * t33 + 40.0 * t128 * t165 - 72.0 * t271 * t460 - 320.0 * t305 * t456 + 7680.0 * t314 * t456 - 2304.0 * t314 * t460 - 8960.0 * t324 * t456 + 1920.0 * t409 * t460 - 40.0 * t169 - 640.0 * t173 + 48.0 * t461 + 240.0 * t463;
        let t500 = t81 * t499;
        let t502 = 6.0 * t245 * t193 * t118 - 3.0 * t250 * t207 * t118 + 2.0 * t288 * t207 * t141 + 3.0 * t106 * t448 + 3.0 * t106 * t467 - t124 * t476 - t124 * t500 - 3.0 * t250 * t445;
        let t507 = piecewise3(t3, 0.0, -t7 * t89 * t209 / 8.0 - 3.0 / 8.0 * t7 * t20 * t502);
        let tv2rhotau0 = 2.0 * rho[ip] * t507 + 2.0 * t213;
        v2rhotau[ip] += tv2rhotau0;
        let t510 = t353 * t276;
        let t511 = t226 * t510;
        let t513 = t245 * t81;
        let t514 = t271 * t403;
        let t517 = t33 * t175;
        let t518 = t366 * t517;
        let t521 = t106 * t180;
        let t524 = t403 * t54;
        let t527 = t175 * t175;
        let t528 = t81 * t527;
        let t531 = t180 * t175;
        let t534 = t260 * t403;
        let t536 = t70 * t27;
        let t538 = t310 * t536 * t276;
        let t540 = 3.0 / 2.0 * t534 + 60.0 * t538;
        let t541 = t81 * t540;
        let t545 = -25.0 / 236196.0 * t335 * t510 + 25.0 / 236196.0 * t511;
        let t547 = -25.0 / 236196.0 * t511 + 12.0 * t513 * t514 - 6.0 * t371 * t518 + 6.0 * t521 * t166 - 3.0 / 4.0 * t164 * t524 + 2.0 * t288 * t528 - 2.0 * t124 * t531 - t124 * t541 + t75 * t545;
        let t551 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t547);
        let tv2sigma20 = 2.0 * rho[ip] * t551;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t553 = t245 * t193;
        let t556 = t193 * t175;
        let t559 = t180 * t192;
        let t562 = t455 * t54;
        let t564 = t260 * t455;
        let t566 = 2.0 * t562 - 4.0 * t564;
        let t567 = t81 * t566;
        let t570 = t250 * t207;
        let t576 = t180 * t206;
        let t579 = t271 * t455;
        let t582 = t310 * t536 * t454;
        let t584 = t137 * t27;
        let t586 = t132 * t584 * t454;
        let t588 = -12.0 * t564 + 18.0 * t579 - 480.0 * t582 + 576.0 * t586;
        let t589 = t81 * t588;
        let t591 = 2.0 * t288 * t207 * t175 + 3.0 * t106 * t559 + 3.0 * t106 * t567 - t124 * t576 - t124 * t589 + 6.0 * t553 * t166 - 3.0 * t570 * t166 - 3.0 * t250 * t556;
        let t595 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t591);
        let tv2sigmatau0 = 2.0 * rho[ip] * t595;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t597 = t192 * t192;
        let t598 = t81 * t597;
        let t601 = t193 * t206;
        let t605 = 1.0 / t19 / t99;
        let t606 = t27 * t605;
        let t609 = t260 * t606;
        let t611 = t271 * t606;
        let t613 = -16.0 * t606 * t54 + 64.0 * t609 - 48.0 * t611;
        let t614 = t81 * t613;
        let t617 = t206 * t206;
        let t618 = t81 * t617;
        let t631 = t323 * t27;
        let t635 = -9216.0 * t132 * t584 * t605 + 3840.0 * t310 * t536 * t605 + 5376.0 * t67 * t631 * t605 + 192.0 * t305 * t606 + 96.0 * t609 - 288.0 * t611;
        let t636 = t81 * t635;
        let t638 = 3.0 * t106 * t614 - t124 * t636 + 6.0 * t245 * t598 - 6.0 * t250 * t601 + 2.0 * t288 * t618;
        let t642 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t638);
        let tv2tau20 = 2.0 * rho[ip] * t642;
        v2tau2[ip] += tv2tau20;
    }
}
