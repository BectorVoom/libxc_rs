//! MGGA_X_GVT4 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gvt4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gvt4_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rho2lapl: &mut [f64],
    v3rho2tau: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3rhosigmalapl: &mut [f64],
    v3rhosigmatau: &mut [f64],
    v3rholapl2: &mut [f64],
    v3rholapltau: &mut [f64],
    v3rhotau2: &mut [f64],
    v3sigma3: &mut [f64],
    v3sigma2lapl: &mut [f64],
    v3sigma2tau: &mut [f64],
    v3sigmalapl2: &mut [f64],
    v3sigmalapltau: &mut [f64],
    v3sigmatau2: &mut [f64],
    v3lapl3: &mut [f64],
    v3lapl2tau: &mut [f64],
    v3lapltau2: &mut [f64],
    v3tau3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRTPI;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3(zeta_threshold);
        let t14 = pow_1_3(t10);
        let t16 = piecewise3(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = 1.0 / t4 * t16;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT2;
        let t21 = t20 * t20;
        let t22 = sigma[ip] * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t27 = t22 * t26;
        let t29 = tau[ip] * t21;
        let t31 = 1.0 / t24 / rho[ip];
        let t32 = t29 * t31;
        let t34 = M_CBRT6;
        let t35 = t34 * t34;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3(t36);
        let t38 = t37 * t37;
        let t39 = t35 * t38;
        let t41 = 1.0 + 0.00186726 * t27 + 0.00373452 * t32 - 0.001120356 * t39;
        let t47 = -0.003556788 * t27 + 0.012500652 * t32 - 0.0037501956 * t39;
        let t48 = t41 * t41;
        let t49 = 1.0 / t48;
        let t51 = sigma[ip] * sigma[ip];
        let t52 = t51 * t20;
        let t53 = t23 * t23;
        let t54 = t53 * rho[ip];
        let t56 = 1.0 / t18 / t54;
        let t61 = 2.0 * t32 - 3.0 / 5.0 * t39;
        let t65 = t61 * t61;
        let t67 = -4.709036e-05 * t52 * t56 - 0.0001282732 * t22 * t26 * t61 + 0.0003574822 * t65;
        let t68 = t48 * t41;
        let t69 = 1.0 / t68;
        let t73 = pow_1_3(1.0 / M_PI);
        let t74 = 1.0 / t73;
        let t76 = M_CBRT4;
        let t77 = (-0.9800683 / t41 + t47 * t49 + t67 * t69) * t74 * t76;
        let t80 = piecewise3(t3, 0.0, t19 * t77 / 4.0);
        let tzk0 = 2.0 * t80;
        zk[ip] += tzk0;
        let t82 = t17 / t24;
        let t85 = t23 * rho[ip];
        let t87 = 1.0 / t24 / t85;
        let t88 = t22 * t87;
        let t90 = t29 * t26;
        let t92 = -0.00497936 * t88 - 0.0062242 * t90;
        let t97 = 0.009484768 * t88 - 0.02083442 * t90;
        let t99 = t47 * t69;
        let t102 = t53 * t23;
        let t104 = 1.0 / t18 / t102;
        let t110 = sigma[ip] * t20;
        let t114 = t61 * tau[ip];
        let t115 = t21 * t26;
        let t118 = 0.00025114858666666666 * t52 * t104 + 0.00034206186666666666 * t22 * t87 * t61 + 0.0008551546666666666 * t110 * t56 * tau[ip] - 0.0023832146666666666 * t114 * t115;
        let t120 = t48 * t48;
        let t121 = 1.0 / t120;
        let t122 = t67 * t121;
        let t127 = (0.9800683 * t49 * t92 + t97 * t49 - 2.0 * t99 * t92 + t118 * t69 - 3.0 * t122 * t92) * t74 * t76;
        let t131 = piecewise3(t3, 0.0, t82 * t77 / 12.0 + t19 * t127 / 4.0);
        let tvrho0 = 2.0 * rho[ip] * t131 + 2.0 * t80;
        vrho[ip] += tvrho0;
        let t134 = t49 * t21;
        let t135 = t134 * t26;
        let t137 = t99 * t115;
        let t139 = t110 * t56;
        let t141 = t115 * t61;
        let t143 = -9.418072e-05 * t139 - 0.0001282732 * t141;
        let t145 = t122 * t115;
        let t149 = (-0.001726745666142 * t135 - 0.00373452 * t137 + t143 * t69 - 0.00560178 * t145) * t74 * t76;
        let t152 = piecewise3(t3, 0.0, t19 * t149 / 4.0);
        let tvsigma0 = 2.0 * rho[ip] * t152;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t156 = t21 * t31;
        let t160 = 1.0 / t18 / t53;
        let t166 = -0.0005130928 * t110 * t160 + 0.0014299288 * t61 * t21 * t31;
        let t172 = (0.016160736667716 * t134 * t31 - 0.00746904 * t99 * t156 + t166 * t69 - 0.01120356 * t122 * t156) * t74 * t76;
        let t175 = piecewise3(t3, 0.0, t19 * t172 / 4.0);
        let tvtau0 = 2.0 * rho[ip] * t175;
        vtau[ip] += tvtau0;
        let t178 = t17 * t31;
        let t183 = t92 * t92;
        let t187 = 1.0 / t24 / t53;
        let t188 = t22 * t187;
        let t190 = t29 * t87;
        let t192 = 0.018257653333333332 * t188 + 0.016597866666666666 * t190;
        let t197 = -0.034777482666666665 * t188 + 0.055558453333333334 * t190;
        let t199 = t97 * t69;
        let t202 = t47 * t121;
        let t207 = t53 * t85;
        let t209 = 1.0 / t18 / t207;
        let t218 = tau[ip] * tau[ip];
        let t219 = t218 * t20;
        let t222 = t21 * t87;
        let t225 = -0.0015906077155555555 * t52 * t209 - 0.0012542268444444445 * t22 * t187 * t61 - 0.006841237333333333 * t110 * t104 * tau[ip] + 0.015888097777777777 * t219 * t56 + 0.006355239111111111 * t114 * t222;
        let t227 = t118 * t121;
        let t231 = 1.0 / t120 / t41;
        let t232 = t67 * t231;
        let t239 = (-1.9601366 * t69 * t183 + 0.9800683 * t49 * t192 + t197 * t49 - 4.0 * t199 * t92 + 6.0 * t202 * t183 - 2.0 * t99 * t192 + t225 * t69 - 6.0 * t227 * t92 + 12.0 * t232 * t183 - 3.0 * t122 * t192) * t74 * t76;
        let t243 = piecewise3(t3, 0.0, -t178 * t77 / 18.0 + t82 * t127 / 6.0 + t19 * t239 / 4.0);
        let tv2rho20 = 2.0 * rho[ip] * t243 + 4.0 * t131;
        v2rho2[ip] += tv2rho20;
        let t248 = t69 * t21;
        let t250 = t248 * t26 * t92;
        let t252 = t134 * t87;
        let t254 = t199 * t115;
        let t256 = t115 * t92;
        let t257 = t202 * t256;
        let t259 = t99 * t222;
        let t261 = t110 * t104;
        let t263 = t222 * t61;
        let t265 = t20 * t56;
        let t266 = t265 * tau[ip];
        let t268 = 0.0005022971733333333 * t261 + 0.00034206186666666666 * t263 + 0.0008551546666666666 * t266;
        let t270 = t143 * t121;
        let t273 = t227 * t115;
        let t275 = t232 * t256;
        let t277 = t122 * t222;
        let t281 = (0.003453491332284 * t250 + 0.004604655109712 * t252 - 0.00373452 * t254 + 0.01120356 * t257 + 0.00995872 * t259 + t268 * t69 - 3.0 * t270 * t92 - 0.00560178 * t273 + 0.02240712 * t275 + 0.01493808 * t277) * t74 * t76;
        let t285 = piecewise3(t3, 0.0, t82 * t149 / 12.0 + t19 * t281 / 4.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t285 + 2.0 * t152;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t290 = t31 * t92;
        let t296 = t156 * t92;
        let t305 = 0.0022234021333333333 * t139 - 0.009532858666666666 * tau[ip] * t20 * t160 - 0.0023832146666666666 * t141;
        let t307 = t166 * t121;
        let t317 = (-0.032321473335432 * t248 * t290 - 0.02693456111286 * t135 - 0.00746904 * t199 * t156 + 0.02240712 * t202 * t296 + 0.0124484 * t137 + t305 * t69 - 3.0 * t307 * t92 - 0.01120356 * t227 * t156 + 0.04481424 * t232 * t296 + 0.0186726 * t145) * t74 * t76;
        let t321 = piecewise3(t3, 0.0, t82 * t172 / 12.0 + t19 * t317 / 4.0);
        let tv2rhotau0 = 2.0 * rho[ip] * t321 + 2.0 * t175;
        v2rhotau[ip] += tv2rhotau0;
        let t324 = t69 * t20;
        let t325 = t324 * t56;
        let t327 = t202 * t265;
        let t329 = t270 * t115;
        let t331 = t232 * t265;
        let t335 = (-5.471779570623876e-05 * t325 + 4.18399188912e-05 * t327 - 0.01120356 * t329 + 8.36798377824e-05 * t331) * t74 * t76;
        let t338 = piecewise3(t3, 0.0, t19 * t335 / 4.0);
        let tv2sigma20 = 2.0 * rho[ip] * t338;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t340 = t324 * t160;
        let t342 = t20 * t160;
        let t343 = t202 * t342;
        let t345 = t307 * t115;
        let t349 = t232 * t342;
        let t353 = (-0.0005806664049135975 * t340 + 8.36798377824e-05 * t343 - 0.00560178 * t345 - 0.01120356 * t270 * t156 + 0.0001673596755648 * t349) * t74 * t76;
        let t356 = piecewise3(t3, 0.0, t19 * t353 / 4.0);
        let tv2sigmatau0 = 2.0 * rho[ip] * t356;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t359 = 1.0 / t18 / t85;
        let t362 = t20 * t359;
        let t371 = (0.005291569083170565 * t324 * t359 + 0.0001673596755648 * t202 * t362 - 0.02240712 * t307 * t156 + 0.0003347193511296 * t232 * t362) * t74 * t76;
        let t374 = piecewise3(t3, 0.0, t19 * t371 / 4.0);
        let tv2tau20 = 2.0 * rho[ip] * t374;
        v2tau2[ip] += tv2tau20;
        let t377 = t17 * t26;
        let t385 = 1.0 / t24 / t54;
        let t386 = t22 * t385;
        let t388 = t29 * t187;
        let t390 = 0.1622949191111111 * t386 - 0.20371432888888888 * t388;
        let t392 = t53 * t53;
        let t394 = 1.0 / t18 / t392;
        let t405 = t21 * t187;
        let t408 = 0.01166445658074074 * t52 * t394 + 0.005853058607407408 * t22 * t385 * t61 + 0.05168934874074074 * t110 * t209 * tau[ip] - 0.1271047822222222 * t219 * t104 - 0.02330254340740741 * t114 * t405;
        let t412 = -0.08520238222222222 * t386 - 0.06085884444444444 * t388;
        let t415 = t118 * t231;
        let t419 = 1.0 / t120 / t48;
        let t420 = t67 * t419;
        let t421 = t183 * t92;
        let t424 = t92 * t192;
        let t427 = t97 * t121;
        let t430 = t47 * t231;
        let t444 = t197 * t69;
        let t451 = t225 * t121;
        let t454 = t390 * t49 + t408 * t69 + 0.9800683 * t49 * t412 + 36.0 * t415 * t183 - 60.0 * t420 * t421 + 36.0 * t232 * t424 + 18.0 * t427 * t183 - 24.0 * t430 * t421 + 18.0 * t202 * t424 - 9.0 * t227 * t192 - 3.0 * t122 * t412 + 5.8804098 * t121 * t421 - 5.8804098 * t69 * t92 * t192 - 6.0 * t444 * t92 - 6.0 * t199 * t192 - 2.0 * t99 * t412 - 9.0 * t451 * t92;
        let t456 = t454 * t74 * t76;
        let t460 = piecewise3(t3, 0.0, 5.0 / 54.0 * t377 * t77 - t178 * t127 / 6.0 + t82 * t239 / 4.0 + t19 * t456 / 4.0);
        let tv3rho30 = 2.0 * rho[ip] * t460 + 6.0 * t243;
        v3rho3[ip] += tv3rho30;
        let t468 = t268 * t121;
        let t473 = t110 * t209;
        let t475 = t405 * t61;
        let t477 = t20 * t104;
        let t478 = t477 * tau[ip];
        let t480 = -0.003181215431111111 * t473 - 0.0012542268444444445 * t475 - 0.006841237333333333 * t478;
        let t482 = t451 * t115;
        let t484 = t26 * t192;
        let t485 = t248 * t484;
        let t487 = t134 * t187;
        let t489 = t444 * t115;
        let t491 = t143 * t231;
        let t494 = t115 * t192;
        let t495 = t202 * t494;
        let t497 = t99 * t405;
        let t499 = t415 * t256;
        let t501 = -6.0 * t468 * t92 - 3.0 * t270 * t192 + t480 * t69 - 0.00560178 * t482 + 0.003453491332284 * t485 - 0.016883735402277333 * t487 - 0.00373452 * t489 + 12.0 * t491 * t183 + 0.01120356 * t495 - 0.036515306666666664 * t497 + 0.04481424 * t499;
        let t502 = t227 * t222;
        let t504 = t232 * t494;
        let t506 = t122 * t405;
        let t508 = t121 * t21;
        let t510 = t508 * t26 * t183;
        let t513 = t248 * t87 * t92;
        let t515 = t427 * t256;
        let t517 = t199 * t222;
        let t519 = t115 * t183;
        let t520 = t430 * t519;
        let t522 = t222 * t92;
        let t523 = t202 * t522;
        let t525 = t420 * t519;
        let t527 = t232 * t522;
        let t529 = 0.02987616 * t502 + 0.02240712 * t504 - 0.05477296 * t506 - 0.010360473996852 * t510 - 0.018418620438848 * t513 + 0.02240712 * t515 + 0.01991744 * t517 - 0.04481424 * t520 - 0.05975232 * t523 - 0.1120356 * t525 - 0.11950464 * t527;
        let t532 = (t501 + t529) * t74 * t76;
        let t536 = piecewise3(t3, 0.0, -t178 * t149 / 18.0 + t82 * t281 / 6.0 + t19 * t532 / 4.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t536 + 4.0 * t285;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t544 = t305 * t121;
        let t554 = t166 * t231;
        let t562 = -0.01185814471111111 * t261 + 0.057197152 * t266 + 0.006355239111111111 * t263;
        let t564 = t156 * t183;
        let t571 = t156 * t192;
        let t574 = -6.0 * t544 * t92 - 3.0 * t307 * t192 - 0.032321473335432 * t248 * t31 * t192 - 0.00746904 * t444 * t156 + 12.0 * t554 * t183 - 0.01120356 * t451 * t156 + t562 * t69 - 0.08962848 * t430 * t564 - 0.2240712 * t420 * t564 + 0.08962848 * t415 * t296 + 0.04481424 * t232 * t571;
        let t590 = 0.096964420006296 * t508 * t31 * t183 + 0.04481424 * t427 * t296 + 0.02240712 * t202 * t571 - 0.0746904 * t257 - 0.03319573333333333 * t259 - 0.1493808 * t275 - 0.0497936 * t277 + 0.10773824445144 * t250 + 0.07182549630096 * t252 + 0.0248968 * t254 + 0.0373452 * t273;
        let t593 = (t574 + t590) * t74 * t76;
        let t597 = piecewise3(t3, 0.0, -t178 * t172 / 18.0 + t82 * t317 / 6.0 + t19 * t593 / 4.0);
        let tv3rho2tau0 = 2.0 * rho[ip] * t597 + 4.0 * t321;
        v3rho2tau[ip] += tv3rho2tau0;
        let t602 = t121 * t20;
        let t604 = t602 * t56 * t92;
        let t606 = t324 * t104;
        let t608 = t427 * t265;
        let t610 = t265 * t92;
        let t611 = t430 * t610;
        let t613 = t202 * t477;
        let t615 = t468 * t115;
        let t617 = t491 * t256;
        let t619 = t270 * t222;
        let t621 = t415 * t265;
        let t623 = t420 * t610;
        let t625 = t232 * t477;
        let t627 = 0.00016415338711871626 * t604 + 0.0002918282437666067 * t606 + 4.18399188912e-05 * t608 - 0.0001673596755648 * t611 - 0.0002231462340864 * t613 - 0.01120356 * t615 + 0.04481424 * t617 + 0.02987616 * t619 + 8.36798377824e-05 * t621 - 0.000418399188912 * t623 - 0.0004462924681728 * t625;
        let t629 = t627 * t74 * t76;
        let t633 = piecewise3(t3, 0.0, t82 * t335 / 12.0 + t19 * t629 / 4.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t633 + 2.0 * t338;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t639 = t602 * t160 * t92;
        let t642 = t427 * t342;
        let t644 = t342 * t92;
        let t645 = t430 * t644;
        let t648 = t544 * t115;
        let t650 = t554 * t256;
        let t652 = t307 * t222;
        let t659 = t415 * t342;
        let t661 = t420 * t644;
        let t664 = 0.0017419992147407926 * t639 + 0.0025162210879589226 * t325 + 8.36798377824e-05 * t642 - 0.0003347193511296 * t645 - 0.0003626126303904 * t327 - 0.00560178 * t648 + 0.02240712 * t650 + 0.01493808 * t652 - 0.01120356 * t468 * t156 + 0.04481424 * t491 * t296 + 0.0186726 * t329 + 0.0001673596755648 * t659 - 0.000836798377824 * t661 - 0.0007252252607808 * t331;
        let t666 = t664 * t74 * t76;
        let t670 = piecewise3(t3, 0.0, t82 * t353 / 12.0 + t19 * t666 / 4.0);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t670 + 2.0 * t356;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t681 = t362 * t92;
        let t695 = -0.015874707249511694 * t602 * t359 * t92 - 0.01763856361056855 * t340 + 0.0001673596755648 * t427 * t362 - 0.0006694387022592 * t430 * t681 - 0.000557865585216 * t343 - 0.02240712 * t544 * t156 + 0.08962848 * t554 * t296 + 0.0373452 * t345 + 0.0003347193511296 * t415 * t362 - 0.001673596755648 * t420 * t681 - 0.001115731170432 * t349;
        let t697 = t695 * t74 * t76;
        let t701 = piecewise3(t3, 0.0, t82 * t371 / 12.0 + t19 * t697 / 4.0);
        let tv3rhotau20 = 2.0 * rho[ip] * t701 + 2.0 * t374;
        v3rhotau2[ip] += tv3rhotau20;
        let t704 = 1.0 / t392;
        let t705 = t121 * t704;
        let t707 = t430 * t704;
        let t709 = t491 * t265;
        let t711 = t420 * t704;
        let t715 = (2.4257213591226012e-06 * t705 - 6.250080555902569e-07 * t707 + 0.0002510395133472 * t709 - 1.5625201389756423e-06 * t711) * t74 * t76;
        let t718 = piecewise3(t3, 0.0, t19 * t715 / 4.0);
        let tv3sigma30 = 2.0 * rho[ip] * t718;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let t720 = 1.0 / t207;
        let t721 = t121 * t720;
        let t723 = t430 * t720;
        let t725 = t554 * t265;
        let t727 = t491 * t342;
        let t729 = t420 * t720;
        let t733 = (1.376905268679541e-05 * t721 - 1.2500161111805137e-06 * t723 + 8.36798377824e-05 * t725 + 0.0003347193511296 * t727 - 3.1250402779512845e-06 * t729) * t74 * t76;
        let t736 = piecewise3(t3, 0.0, t19 * t733 / 4.0);
        let tv3sigma2tau0 = 2.0 * rho[ip] * t736;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let t738 = 1.0 / t102;
        let t739 = t121 * t738;
        let t741 = t430 * t738;
        let t743 = t554 * t342;
        let t747 = t420 * t738;
        let t751 = (-3.748107360743996e-05 * t739 - 2.5000322223610275e-06 * t741 + 0.0003347193511296 * t743 + 0.0003347193511296 * t491 * t362 - 6.250080555902569e-06 * t747) * t74 * t76;
        let t754 = piecewise3(t3, 0.0, t19 * t751 / 4.0);
        let tv3sigmatau20 = 2.0 * rho[ip] * t754;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let t756 = 1.0 / t54;
        let t767 = (-0.0003707093030132039 * t121 * t756 - 5.000064444722055e-06 * t430 * t756 + 0.0010041580533888 * t554 * t362 - 1.2500161111805138e-05 * t420 * t756) * t74 * t76;
        let t770 = piecewise3(t3, 0.0, t19 * t767 / 4.0);
        let tv3tau30 = 2.0 * rho[ip] * t770;
        v3tau3[ip] += tv3tau30;
    }
}
