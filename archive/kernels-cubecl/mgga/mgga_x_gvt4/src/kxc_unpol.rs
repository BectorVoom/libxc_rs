//! MGGA_X_GVT4 kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gvt4.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_gvt4_kxc_unpol(
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
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRTPI;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5::<f64>(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3::<f64>(zeta_threshold);
        let t14 = pow_1_3::<f64>(t10);
        let t16 = piecewise3::<f64>(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = 1.0 / t4 * t16;
        let t18 = pow_1_3::<f64>(rho[ip]);
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
        let t37 = pow_1_3::<f64>(t36);
        let t38 = t37 * t37;
        let t39 = t35 * t38;
        let t41 = 1.0 + 0.186726e-2 * t27 + 0.373452e-2 * t32 - 0.1120356e-2 * t39;
        let t47 = -0.3556788e-2 * t27 + 0.12500652e-1 * t32 - 0.37501956e-2 * t39;
        let t48 = t41 * t41;
        let t49 = 1.0 / t48;
        let t51 = sigma[ip] * sigma[ip];
        let t52 = t51 * t20;
        let t53 = t23 * t23;
        let t54 = t53 * rho[ip];
        let t56 = 1.0 / t18 / t54;
        let t61 = 2.0 * t32 - 3.0 / 5.0 * t39;
        let t65 = t61 * t61;
        let t67 = -0.4709036e-4 * t52 * t56 - 0.1282732e-3 * t22 * t26 * t61 + 0.3574822e-3 * t65;
        let t68 = t48 * t41;
        let t69 = 1.0 / t68;
        let t73 = pow_1_3::<f64>(1.0 / M_PI);
        let t74 = 1.0 / t73;
        let t76 = M_CBRT4;
        let t77 = (-0.9800683e0 / t41 + t47 * t49 + t67 * t69) * t74 * t76;
        let t80 = piecewise3::<f64>(t3, 0.0, t19 * t77 / 4.0);
        let tzk0 = 2.0 * t80;
        zk[ip] += tzk0;
        let t82 = t17 / t24;
        let t85 = t23 * rho[ip];
        let t87 = 1.0 / t24 / t85;
        let t88 = t22 * t87;
        let t90 = t29 * t26;
        let t92 = -0.497936e-2 * t88 - 0.62242e-2 * t90;
        let t97 = 0.9484768e-2 * t88 - 0.2083442e-1 * t90;
        let t99 = t47 * t69;
        let t102 = t53 * t23;
        let t104 = 1.0 / t18 / t102;
        let t110 = sigma[ip] * t20;
        let t114 = t61 * tau[ip];
        let t115 = t21 * t26;
        let t118 = 0.25114858666666666667e-3 * t52 * t104 + 0.34206186666666666667e-3 * t22 * t87 * t61 + 0.85515466666666666667e-3 * t110 * t56 * tau[ip] - 0.23832146666666666667e-2 * t114 * t115;
        let t120 = t48 * t48;
        let t121 = 1.0 / t120;
        let t122 = t67 * t121;
        let t127 = (0.9800683e0 * t49 * t92 + t97 * t49 - 2.0 * t99 * t92 + t118 * t69 - 3.0 * t122 * t92) * t74 * t76;
        let t131 = piecewise3::<f64>(t3, 0.0, t82 * t77 / 12.0 + t19 * t127 / 4.0);
        let tvrho0 = 2.0 * rho[ip] * t131 + 2.0 * t80;
        vrho[ip] += tvrho0;
        let t134 = t49 * t21;
        let t135 = t134 * t26;
        let t137 = t99 * t115;
        let t139 = t110 * t56;
        let t141 = t115 * t61;
        let t143 = -0.9418072e-4 * t139 - 0.1282732e-3 * t141;
        let t145 = t122 * t115;
        let t149 = (-0.1726745666142e-2 * t135 - 0.373452e-2 * t137 + t143 * t69 - 0.560178e-2 * t145) * t74 * t76;
        let t152 = piecewise3::<f64>(t3, 0.0, t19 * t149 / 4.0);
        let tvsigma0 = 2.0 * rho[ip] * t152;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t156 = t21 * t31;
        let t160 = 1.0 / t18 / t53;
        let t166 = -0.5130928e-3 * t110 * t160 + 0.14299288e-2 * t61 * t21 * t31;
        let t172 = (0.16160736667716e-1 * t134 * t31 - 0.746904e-2 * t99 * t156 + t166 * t69 - 0.1120356e-1 * t122 * t156) * t74 * t76;
        let t175 = piecewise3::<f64>(t3, 0.0, t19 * t172 / 4.0);
        let tvtau0 = 2.0 * rho[ip] * t175;
        vtau[ip] += tvtau0;
        let t178 = t17 * t31;
        let t183 = t92 * t92;
        let t187 = 1.0 / t24 / t53;
        let t188 = t22 * t187;
        let t190 = t29 * t87;
        let t192 = 0.18257653333333333333e-1 * t188 + 0.16597866666666666667e-1 * t190;
        let t197 = -0.34777482666666666667e-1 * t188 + 0.55558453333333333333e-1 * t190;
        let t199 = t97 * t69;
        let t202 = t47 * t121;
        let t207 = t53 * t85;
        let t209 = 1.0 / t18 / t207;
        let t218 = tau[ip] * tau[ip];
        let t219 = t218 * t20;
        let t222 = t21 * t87;
        let t225 = -0.15906077155555555556e-2 * t52 * t209 - 0.12542268444444444445e-2 * t22 * t187 * t61 - 0.68412373333333333334e-2 * t110 * t104 * tau[ip] + 0.15888097777777777778e-1 * t219 * t56 + 0.63552391111111111112e-2 * t114 * t222;
        let t227 = t118 * t121;
        let t231 = 1.0 / t120 / t41;
        let t232 = t67 * t231;
        let t239 = (-0.19601366e1 * t69 * t183 + 0.9800683e0 * t49 * t192 + t197 * t49 - 4.0 * t199 * t92 + 6.0 * t202 * t183 - 2.0 * t99 * t192 + t225 * t69 - 6.0 * t227 * t92 + 12.0 * t232 * t183 - 3.0 * t122 * t192) * t74 * t76;
        let t243 = piecewise3::<f64>(t3, 0.0, -t178 * t77 / 18.0 + t82 * t127 / 6.0 + t19 * t239 / 4.0);
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
        let t268 = 0.50229717333333333333e-3 * t261 + 0.34206186666666666667e-3 * t263 + 0.85515466666666666667e-3 * t266;
        let t270 = t143 * t121;
        let t273 = t227 * t115;
        let t275 = t232 * t256;
        let t277 = t122 * t222;
        let t281 = (0.3453491332284e-2 * t250 + 0.4604655109712e-2 * t252 - 0.373452e-2 * t254 + 0.1120356e-1 * t257 + 0.995872e-2 * t259 + t268 * t69 - 3.0 * t270 * t92 - 0.560178e-2 * t273 + 0.2240712e-1 * t275 + 0.1493808e-1 * t277) * t74 * t76;
        let t285 = piecewise3::<f64>(t3, 0.0, t82 * t149 / 12.0 + t19 * t281 / 4.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t285 + 2.0 * t152;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t290 = t31 * t92;
        let t296 = t156 * t92;
        let t305 = 0.22234021333333333333e-2 * t139 - 0.95328586666666666667e-2 * tau[ip] * t20 * t160 - 0.23832146666666666667e-2 * t141;
        let t307 = t166 * t121;
        let t317 = (-0.32321473335432e-1 * t248 * t290 - 0.2693456111286e-1 * t135 - 0.746904e-2 * t199 * t156 + 0.2240712e-1 * t202 * t296 + 0.124484e-1 * t137 + t305 * t69 - 3.0 * t307 * t92 - 0.1120356e-1 * t227 * t156 + 0.4481424e-1 * t232 * t296 + 0.186726e-1 * t145) * t74 * t76;
        let t321 = piecewise3::<f64>(t3, 0.0, t82 * t172 / 12.0 + t19 * t317 / 4.0);
        let tv2rhotau0 = 2.0 * rho[ip] * t321 + 2.0 * t175;
        v2rhotau[ip] += tv2rhotau0;
        let t324 = t69 * t20;
        let t325 = t324 * t56;
        let t327 = t202 * t265;
        let t329 = t270 * t115;
        let t331 = t232 * t265;
        let t335 = (-0.5471779570623875632e-4 * t325 + 0.418399188912e-4 * t327 - 0.1120356e-1 * t329 + 0.836798377824e-4 * t331) * t74 * t76;
        let t338 = piecewise3::<f64>(t3, 0.0, t19 * t335 / 4.0);
        let tv2sigma20 = 2.0 * rho[ip] * t338;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t340 = t324 * t160;
        let t342 = t20 * t160;
        let t343 = t202 * t342;
        let t345 = t307 * t115;
        let t349 = t232 * t342;
        let t353 = (-0.58066640491359751264e-3 * t340 + 0.836798377824e-4 * t343 - 0.560178e-2 * t345 - 0.1120356e-1 * t270 * t156 + 0.1673596755648e-3 * t349) * t74 * t76;
        let t356 = piecewise3::<f64>(t3, 0.0, t19 * t353 / 4.0);
        let tv2sigmatau0 = 2.0 * rho[ip] * t356;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t359 = 1.0 / t18 / t85;
        let t362 = t20 * t359;
        let t371 = (0.52915690831705649747e-2 * t324 * t359 + 0.1673596755648e-3 * t202 * t362 - 0.2240712e-1 * t307 * t156 + 0.3347193511296e-3 * t232 * t362) * t74 * t76;
        let t374 = piecewise3::<f64>(t3, 0.0, t19 * t371 / 4.0);
        let tv2tau20 = 2.0 * rho[ip] * t374;
        v2tau2[ip] += tv2tau20;
        let t377 = t17 * t26;
        let t385 = 1.0 / t24 / t54;
        let t386 = t22 * t385;
        let t388 = t29 * t187;
        let t390 = 0.16229491911111111111e0 * t386 - 0.20371432888888888889e0 * t388;
        let t392 = t53 * t53;
        let t394 = 1.0 / t18 / t392;
        let t405 = t21 * t187;
        let t408 = 0.11664456580740740741e-1 * t52 * t394 + 0.58530586074074074077e-2 * t22 * t385 * t61 + 0.51689348740740740742e-1 * t110 * t209 * tau[ip] - 0.12710478222222222222e0 * t219 * t104 - 0.23302543407407407408e-1 * t114 * t405;
        let t412 = -0.85202382222222222221e-1 * t386 - 0.60858844444444444446e-1 * t388;
        let t415 = t118 * t231;
        let t419 = 1.0 / t120 / t48;
        let t420 = t67 * t419;
        let t421 = t183 * t92;
        let t424 = t92 * t192;
        let t427 = t97 * t121;
        let t430 = t47 * t231;
        let t444 = t197 * t69;
        let t451 = t225 * t121;
        let t454 = t390 * t49 + t408 * t69 + 0.9800683e0 * t49 * t412 + 36.0 * t415 * t183 - 60.0 * t420 * t421 + 36.0 * t232 * t424 + 18.0 * t427 * t183 - 24.0 * t430 * t421 + 18.0 * t202 * t424 - 9.0 * t227 * t192 - 3.0 * t122 * t412 + 0.58804098e1 * t121 * t421 - 0.58804098e1 * t69 * t92 * t192 - 6.0 * t444 * t92 - 6.0 * t199 * t192 - 2.0 * t99 * t412 - 9.0 * t451 * t92;
        let t456 = t454 * t74 * t76;
        let t460 = piecewise3::<f64>(t3, 0.0, 5.0 / 54.0 * t377 * t77 - t178 * t127 / 6.0 + t82 * t239 / 4.0 + t19 * t456 / 4.0);
        let tv3rho30 = 2.0 * rho[ip] * t460 + 6.0 * t243;
        v3rho3[ip] += tv3rho30;
        let t468 = t268 * t121;
        let t473 = t110 * t209;
        let t475 = t405 * t61;
        let t477 = t20 * t104;
        let t478 = t477 * tau[ip];
        let t480 = -0.31812154311111111111e-2 * t473 - 0.12542268444444444445e-2 * t475 - 0.68412373333333333334e-2 * t478;
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
        let t501 = -6.0 * t468 * t92 - 3.0 * t270 * t192 + t480 * t69 - 0.560178e-2 * t482 + 0.3453491332284e-2 * t485 - 0.16883735402277333333e-1 * t487 - 0.373452e-2 * t489 + 12.0 * t491 * t183 + 0.1120356e-1 * t495 - 0.36515306666666666667e-1 * t497 + 0.4481424e-1 * t499;
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
        let t529 = 0.2987616e-1 * t502 + 0.2240712e-1 * t504 - 0.5477296e-1 * t506 - 0.10360473996852e-1 * t510 - 0.18418620438848e-1 * t513 + 0.2240712e-1 * t515 + 0.1991744e-1 * t517 - 0.4481424e-1 * t520 - 0.5975232e-1 * t523 - 0.1120356e0 * t525 - 0.11950464e0 * t527;
        let t532 = (t501 + t529) * t74 * t76;
        let t536 = piecewise3::<f64>(t3, 0.0, -t178 * t149 / 18.0 + t82 * t281 / 6.0 + t19 * t532 / 4.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t536 + 4.0 * t285;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t544 = t305 * t121;
        let t554 = t166 * t231;
        let t562 = -0.11858144711111111111e-1 * t261 + 0.57197152e-1 * t266 + 0.63552391111111111112e-2 * t263;
        let t564 = t156 * t183;
        let t571 = t156 * t192;
        let t574 = -6.0 * t544 * t92 - 3.0 * t307 * t192 - 0.32321473335432e-1 * t248 * t31 * t192 - 0.746904e-2 * t444 * t156 + 12.0 * t554 * t183 - 0.1120356e-1 * t451 * t156 + t562 * t69 - 0.8962848e-1 * t430 * t564 - 0.2240712e0 * t420 * t564 + 0.8962848e-1 * t415 * t296 + 0.4481424e-1 * t232 * t571;
        let t590 = 0.96964420006296e-1 * t508 * t31 * t183 + 0.4481424e-1 * t427 * t296 + 0.2240712e-1 * t202 * t571 - 0.746904e-1 * t257 - 0.33195733333333333333e-1 * t259 - 0.1493808e0 * t275 - 0.497936e-1 * t277 + 0.10773824445144e0 * t250 + 0.7182549630096e-1 * t252 + 0.248968e-1 * t254 + 0.373452e-1 * t273;
        let t593 = (t574 + t590) * t74 * t76;
        let t597 = piecewise3::<f64>(t3, 0.0, -t178 * t172 / 18.0 + t82 * t317 / 6.0 + t19 * t593 / 4.0);
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
        let t627 = 0.16415338711871626896e-3 * t604 + 0.29182824376660670037e-3 * t606 + 0.418399188912e-4 * t608 - 0.1673596755648e-3 * t611 - 0.2231462340864e-3 * t613 - 0.1120356e-1 * t615 + 0.4481424e-1 * t617 + 0.2987616e-1 * t619 + 0.836798377824e-4 * t621 - 0.418399188912e-3 * t623 - 0.4462924681728e-3 * t625;
        let t629 = t627 * t74 * t76;
        let t633 = piecewise3::<f64>(t3, 0.0, t82 * t335 / 12.0 + t19 * t629 / 4.0);
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
        let t664 = 0.17419992147407925379e-2 * t639 + 0.25162210879589225548e-2 * t325 + 0.836798377824e-4 * t642 - 0.3347193511296e-3 * t645 - 0.3626126303904e-3 * t327 - 0.560178e-2 * t648 + 0.2240712e-1 * t650 + 0.1493808e-1 * t652 - 0.1120356e-1 * t468 * t156 + 0.4481424e-1 * t491 * t296 + 0.186726e-1 * t329 + 0.1673596755648e-3 * t659 - 0.836798377824e-3 * t661 - 0.7252252607808e-3 * t331;
        let t666 = t664 * t74 * t76;
        let t670 = piecewise3::<f64>(t3, 0.0, t82 * t353 / 12.0 + t19 * t666 / 4.0);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t670 + 2.0 * t356;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t681 = t362 * t92;
        let t695 = -0.15874707249511694924e-1 * t602 * t359 * t92 - 0.17638563610568549916e-1 * t340 + 0.1673596755648e-3 * t427 * t362 - 0.6694387022592e-3 * t430 * t681 - 0.557865585216e-3 * t343 - 0.2240712e-1 * t544 * t156 + 0.8962848e-1 * t554 * t296 + 0.373452e-1 * t345 + 0.3347193511296e-3 * t415 * t362 - 0.1673596755648e-2 * t420 * t681 - 0.1115731170432e-2 * t349;
        let t697 = t695 * t74 * t76;
        let t701 = piecewise3::<f64>(t3, 0.0, t82 * t371 / 12.0 + t19 * t697 / 4.0);
        let tv3rhotau20 = 2.0 * rho[ip] * t701 + 2.0 * t374;
        v3rhotau2[ip] += tv3rhotau20;
        let t704 = 1.0 / t392;
        let t705 = t121 * t704;
        let t707 = t430 * t704;
        let t709 = t491 * t265;
        let t711 = t420 * t704;
        let t715 = (0.24257213591226013496e-5 * t705 - 0.625008055590256896e-6 * t707 + 0.2510395133472e-3 * t709 - 0.156252013897564224e-5 * t711) * t74 * t76;
        let t718 = piecewise3::<f64>(t3, 0.0, t19 * t715 / 4.0);
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
        let t733 = (0.13769052686795410687e-4 * t721 - 0.1250016111180513792e-5 * t723 + 0.836798377824e-4 * t725 + 0.3347193511296e-3 * t727 - 0.312504027795128448e-5 * t729) * t74 * t76;
        let t736 = piecewise3::<f64>(t3, 0.0, t19 * t733 / 4.0);
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
        let t751 = (-0.37481073607439962653e-4 * t739 - 0.2500032222361027584e-5 * t741 + 0.3347193511296e-3 * t743 + 0.3347193511296e-3 * t491 * t362 - 0.625008055590256896e-5 * t747) * t74 * t76;
        let t754 = piecewise3::<f64>(t3, 0.0, t19 * t751 / 4.0);
        let tv3sigmatau20 = 2.0 * rho[ip] * t754;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let t756 = 1.0 / t54;
        let t767 = (-0.37070930301320389336e-3 * t121 * t756 - 0.5000064444722055168e-5 * t430 * t756 + 0.10041580533888e-2 * t554 * t362 - 0.1250016111180513792e-4 * t420 * t756) * t74 * t76;
        let t770 = piecewise3::<f64>(t3, 0.0, t19 * t767 / 4.0);
        let tv3tau30 = 2.0 * rho[ip] * t770;
        v3tau3[ip] += tv3tau30;
    }
}
