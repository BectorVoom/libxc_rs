//! GGA_X_VMT84 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_vmt84.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_vmt84_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_mu: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = param_mu * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t25 * sigma[ip];
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t32 = t31 * t30;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t36 = param_alpha * t20 * t25;
        let t37 = sigma[ip] * t29;
        let t38 = t37 * t33;
        let t41 = f64::exp(-t36 * t38 / 24.0);
        let t42 = t21 * t25;
        let t45 = 1.0 + t42 * t38 / 24.0;
        let t46 = 1.0 / t45;
        let t47 = t41 * t46;
        let t48 = t34 * t47;
        let t51 = t20 * t20;
        let t54 = 1.0 / t23 / t22;
        let t55 = param_alpha * t51 * t54;
        let t56 = sigma[ip] * sigma[ip];
        let t57 = t56 * t28;
        let t58 = t30 * t30;
        let t59 = t58 * rho[ip];
        let t61 = 1.0 / t18 / t59;
        let t65 = f64::exp(-t55 * t57 * t61 / 288.0);
        let t68 = (1.0 - t65) * t51 * t24;
        let t69 = 1.0 / sigma[ip];
        let t70 = t69 * t28;
        let t74 = t27 * t48 / 24.0 + 2.0 * t68 * t70 * t32 + t65;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t17 / t31;
        let t84 = t30 * rho[ip];
        let t86 = 1.0 / t31 / t84;
        let t88 = t29 * t86 * t47;
        let t91 = param_mu * t51;
        let t92 = t54 * t56;
        let t93 = t91 * t92;
        let t94 = t58 * t30;
        let t96 = 1.0 / t18 / t94;
        let t97 = t28 * t96;
        let t98 = param_alpha * t41;
        let t99 = t98 * t46;
        let t103 = param_mu * param_mu;
        let t104 = t103 * t51;
        let t105 = t104 * t92;
        let t106 = t45 * t45;
        let t107 = 1.0 / t106;
        let t108 = t41 * t107;
        let t109 = t97 * t108;
        let t112 = t86 * t65;
        let t116 = t31 * rho[ip];
        let t120 = t96 * t65;
        let t124 = -t27 * t88 / 9.0 + t93 * t97 * t99 / 108.0 + t105 * t109 / 108.0 - 2.0 / 9.0 * t36 * t37 * t112 + 16.0 / 3.0 * t68 * t70 * t116 + t55 * t57 * t120 / 54.0;
        let t129 = piecewise3(t2, 0.0, -t6 * t80 * t74 / 8.0 - 3.0 / 8.0 * t6 * t19 * t124);
        let tvrho0 = 2.0 * rho[ip] * t129 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t134 = t54 * sigma[ip];
        let t136 = t28 * t61;
        let t141 = t136 * t108;
        let t147 = 1.0 / t56;
        let t148 = t147 * t28;
        let t152 = sigma[ip] * t28;
        let t157 = t42 * t48 / 24.0 - t91 * t134 * t136 * t99 / 288.0 - t104 * t134 * t141 / 288.0 + t36 * t34 * t65 / 12.0 - 2.0 * t68 * t148 * t32 - t55 * t152 * t61 * t65 / 144.0;
        let t161 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t157);
        let tvsigma0 = 2.0 * rho[ip] * t161;
        vsigma[ip] += tvsigma0;
        let t165 = t17 / t116;
        let t173 = 1.0 / t31 / t58;
        let t174 = t29 * t173;
        let t175 = t174 * t47;
        let t178 = t58 * t84;
        let t180 = 1.0 / t18 / t178;
        let t181 = t28 * t180;
        let t185 = t181 * t108;
        let t188 = t22 * t22;
        let t189 = 1.0 / t188;
        let t190 = param_mu * t189;
        let t191 = t56 * sigma[ip];
        let t192 = t190 * t191;
        let t193 = t58 * t58;
        let t194 = t193 * t30;
        let t195 = 1.0 / t194;
        let t196 = param_alpha * param_alpha;
        let t201 = t103 * t189;
        let t202 = t201 * t191;
        let t208 = t103 * param_mu * t189;
        let t209 = t208 * t191;
        let t212 = 1.0 / t106 / t45;
        let t216 = t173 * t65;
        let t220 = t196 * t189;
        let t228 = t180 * t65;
        let t234 = 1.0 / t24 / t188;
        let t235 = t196 * t20 * t234;
        let t236 = t56 * t56;
        let t237 = t236 * t29;
        let t238 = t193 * t58;
        let t240 = 1.0 / t31 / t238;
        let t241 = t240 * t65;
        let t245 = 11.0 / 27.0 * t27 * t175 - t93 * t181 * t99 / 12.0 - t105 * t185 / 12.0 + t192 * t195 * t196 * t47 / 81.0 + 2.0 / 81.0 * t202 * t195 * param_alpha * t108 + 2.0 / 81.0 * t209 * t195 * t41 * t212 + 2.0 / 9.0 * t36 * t37 * t216 - 4.0 / 81.0 * t220 * t191 * t195 * t65 + 80.0 / 9.0 * t68 * t70 * t31 - 19.0 / 162.0 * t55 * t57 * t228 + t235 * t237 * t241 / 486.0;
        let t250 = piecewise3(t2, 0.0, t6 * t165 * t74 / 12.0 - t6 * t80 * t124 / 4.0 - 3.0 / 8.0 * t6 * t19 * t245);
        let tv2rho20 = 2.0 * rho[ip] * t250 + 4.0 * t129;
        v2rho2[ip] += tv2rho20;
        let t258 = t54 * t28;
        let t259 = t91 * t258;
        let t260 = t96 * param_alpha;
        let t262 = sigma[ip] * t41 * t46;
        let t266 = t104 * t258;
        let t268 = t107 * sigma[ip];
        let t273 = t193 * rho[ip];
        let t274 = 1.0 / t273;
        let t286 = t274 * t41 * t212;
        let t299 = t191 * t29;
        let t300 = t193 * t84;
        let t302 = 1.0 / t31 / t300;
        let t307 = -t42 * t88 / 9.0 + t259 * t260 * t262 / 36.0 + t266 * t96 * t41 * t268 / 36.0 - t190 * t56 * t274 * t196 * t47 / 216.0 - t201 * t56 * t274 * param_alpha * t108 / 108.0 - t208 * t56 * t286 / 108.0 + t220 * t274 * t56 * t65 / 54.0 - 16.0 / 3.0 * t68 * t148 * t116 + t55 * t152 * t120 / 27.0 - t235 * t299 * t302 * t65 / 1296.0;
        let t312 = piecewise3(t2, 0.0, -t6 * t80 * t157 / 8.0 - 3.0 / 8.0 * t6 * t19 * t307);
        let tv2rhosigma0 = 2.0 * rho[ip] * t312 + 2.0 * t161;
        v2rhosigma[ip] += tv2rhosigma0;
        let t319 = t104 * t54;
        let t323 = 1.0 / t193;
        let t335 = t323 * t41 * t212;
        let t342 = t69 * t29;
        let t343 = t33 * t65;
        let t347 = 1.0 / t191;
        let t348 = t347 * t28;
        let t357 = 1.0 / t31 / t194;
        let t362 = -t259 * t61 * param_alpha * t47 / 144.0 - t319 * t141 / 144.0 + t190 * sigma[ip] * t323 * t196 * t47 / 576.0 + t201 * sigma[ip] * t323 * param_alpha * t108 / 288.0 + t208 * sigma[ip] * t335 / 288.0 - t220 * t323 * sigma[ip] * t65 / 144.0 - t36 * t342 * t343 / 12.0 + 4.0 * t68 * t348 * t32 - t55 * t136 * t65 / 144.0 + t235 * t56 * t29 * t357 * t65 / 3456.0;
        let t366 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t362);
        let tv2sigma20 = 2.0 * rho[ip] * t366;
        v2sigma2[ip] += tv2sigma20;
        let t369 = t17 * t33;
        let t379 = 1.0 / t300;
        let t385 = 1.0 / t18 / t193;
        let t386 = t385 * t65;
        let t390 = t28 * t385;
        let t395 = 1.0 / t31 / t59;
        let t400 = t196 * param_alpha;
        let t401 = t400 * t189;
        let t402 = t236 * sigma[ip];
        let t403 = t193 * t193;
        let t405 = 1.0 / t18 / t403;
        let t408 = t51 * t54;
        let t410 = t408 * t28 * t65;
        let t413 = t193 * t59;
        let t415 = 1.0 / t31 / t413;
        let t416 = t236 * t415;
        let t419 = t20 * t25;
        let t420 = t29 * t41;
        let t421 = t420 * t46;
        let t422 = t419 * t421;
        let t427 = t419 * t29;
        let t428 = t108 * t427;
        let t433 = t41 * t212;
        let t434 = t433 * t427;
        let t437 = t415 * t65;
        let t449 = 1.0 / t18;
        let t457 = t188 * t188;
        let t458 = 1.0 / t457;
        let t459 = t400 * t458;
        let t460 = t236 * t56;
        let t461 = t403 * t84;
        let t462 = 1.0 / t461;
        let t467 = t29 * t395;
        let t468 = t467 * t47;
        let t474 = t103 * t103;
        let t475 = t474 * t189;
        let t477 = t106 * t106;
        let t478 = 1.0 / t477;
        let t479 = t41 * t478;
        let t480 = t479 * t427;
        let t483 = -38.0 / 81.0 * t209 * t379 * t41 * t212 + 209.0 / 243.0 * t55 * t57 * t386 + 341.0 / 486.0 * t105 * t390 * t108 - 164.0 / 81.0 * t36 * t37 * t395 * t65 - 2.0 / 2187.0 * t401 * t402 * t405 * t410 + t190 * t416 * t400 * t422 / 729.0 + t201 * t416 * t196 * t428 / 243.0 + 2.0 / 243.0 * t208 * t416 * param_alpha * t434 - 19.0 / 486.0 * t235 * t237 * t437 - 19.0 / 81.0 * t192 * t379 * t196 * t47 - 38.0 / 81.0 * t202 * t379 * param_alpha * t108 + 160.0 / 27.0 * t68 * t70 * t449 + 44.0 / 81.0 * t220 * t191 * t379 * t65 + t459 * t460 * t462 * t65 / 2187.0 - 154.0 / 81.0 * t27 * t468 + 341.0 / 486.0 * t93 * t390 * t99 + 2.0 / 243.0 * t475 * t416 * t480;
        let t488 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t369 * t74 + t6 * t165 * t124 / 4.0 - 3.0 / 8.0 * t6 * t80 * t245 - 3.0 / 8.0 * t6 * t19 * t483);
        let tv3rho30 = 2.0 * rho[ip] * t488 + 6.0 * t250;
        v3rho3[ip] += tv3rho30;
        let t498 = t208 * t195;
        let t499 = t433 * t56;
        let t511 = 1.0 / t18 / t193 / t178;
        let t525 = t191 * t240;
        let t544 = t190 * t195;
        let t546 = t196 * t56 * t47;
        let t549 = t201 * t195;
        let t551 = param_alpha * t56 * t108;
        let t554 = t403 * t30;
        let t555 = 1.0 / t554;
        let t560 = t180 * param_alpha;
        let t567 = 17.0 / 108.0 * t498 * t499 - t220 * t195 * t56 * t65 / 6.0 + 16.0 / 27.0 * t36 * t174 * t65 + t401 * t511 * t236 * t410 / 2916.0 - 19.0 / 81.0 * t55 * t152 * t228 + 11.0 / 27.0 * t42 * t175 - 65.0 / 324.0 * t266 * t180 * t41 * t268 - t190 * t525 * t400 * t422 / 1944.0 - t201 * t525 * t196 * t428 / 648.0 - t208 * t525 * param_alpha * t434 / 324.0 - 80.0 / 9.0 * t68 * t148 * t31 + 17.0 / 1296.0 * t235 * t299 * t241 + 17.0 / 216.0 * t544 * t546 + 17.0 / 108.0 * t549 * t551 - t459 * t402 * t555 * t65 / 5832.0 - 65.0 / 324.0 * t259 * t560 * t262 - t475 * t525 * t480 / 324.0;
        let t572 = piecewise3(t2, 0.0, t6 * t165 * t157 / 12.0 - t6 * t80 * t307 / 4.0 - 3.0 / 8.0 * t6 * t19 * t567);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t572 + 4.0 * t312;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t578 = t403 * rho[ip];
        let t579 = 1.0 / t578;
        let t585 = t433 * sigma[ip];
        let t592 = t193 * t94;
        let t594 = 1.0 / t18 / t592;
        let t604 = t56 * t302;
        let t617 = t190 * t274;
        let t619 = t196 * sigma[ip] * t47;
        let t622 = t201 * t274;
        let t623 = t98 * t268;
        let t632 = t29 * t302;
        let t633 = t56 * t65;
        let t643 = t459 * t236 * t579 * t65 / 15552.0 - 5.0 / 108.0 * t208 * t274 * t585 + t220 * t274 * sigma[ip] * t65 / 27.0 - t401 * t594 * t191 * t410 / 7776.0 - 2.0 / 9.0 * t36 * t342 * t112 + t319 * t109 / 27.0 + t208 * t604 * param_alpha * t434 / 864.0 + t190 * t604 * t400 * t422 / 5184.0 + t201 * t604 * t196 * t428 / 1728.0 - 5.0 / 216.0 * t617 * t619 - 5.0 / 108.0 * t622 * t623 + 32.0 / 3.0 * t68 * t348 * t116 + t55 * t97 * t65 / 27.0 - 5.0 / 1296.0 * t235 * t632 * t633 + t259 * t260 * t47 / 27.0 + t475 * t604 * t480 / 864.0;
        let t648 = piecewise3(t2, 0.0, -t6 * t80 * t362 / 8.0 - 3.0 / 8.0 * t6 * t19 * t643);
        let tv3rhosigma20 = 2.0 * rho[ip] * t648 + 2.0 * t366;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t653 = t196 * t41 * t46;
        let t657 = t98 * t107;
        let t662 = sigma[ip] * t357;
        let t679 = 1.0 / t18 / t413;
        let t684 = t147 * t29;
        let t689 = 1.0 / t236 * t28;
        let t693 = t29 * t357;
        let t694 = sigma[ip] * t65;
        let t698 = 1.0 / t403;
        let t703 = t190 * t323 * t653 / 192.0 + t201 * t323 * t657 / 96.0 + t208 * t335 / 96.0 - t190 * t662 * t400 * t422 / 13824.0 - t201 * t662 * t196 * t428 / 4608.0 - t208 * t662 * param_alpha * t434 / 2304.0 - t475 * t662 * t480 / 2304.0 + t401 * t679 * t56 * t410 / 20736.0 + t36 * t684 * t343 / 4.0 - 12.0 * t68 * t689 * t32 + t235 * t693 * t694 / 1152.0 - t459 * t191 * t698 * t65 / 41472.0;
        let t707 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t703);
        let tv3sigma30 = 2.0 * rho[ip] * t707;
        v3sigma3[ip] += tv3sigma30;
    }
}
