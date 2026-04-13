//! MGGA_X_GVT4 lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gvt4.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_gvt4_lxc_unpol(
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
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho3lapl: &mut Array<f64>,
    v4rho3tau: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rho2sigmalapl: &mut Array<f64>,
    v4rho2sigmatau: &mut Array<f64>,
    v4rho2lapl2: &mut Array<f64>,
    v4rho2lapltau: &mut Array<f64>,
    v4rho2tau2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4rhosigma2lapl: &mut Array<f64>,
    v4rhosigma2tau: &mut Array<f64>,
    v4rhosigmalapl2: &mut Array<f64>,
    v4rhosigmalapltau: &mut Array<f64>,
    v4rhosigmatau2: &mut Array<f64>,
    v4rholapl3: &mut Array<f64>,
    v4rholapl2tau: &mut Array<f64>,
    v4rholapltau2: &mut Array<f64>,
    v4rhotau3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    v4sigma3lapl: &mut Array<f64>,
    v4sigma3tau: &mut Array<f64>,
    v4sigma2lapl2: &mut Array<f64>,
    v4sigma2lapltau: &mut Array<f64>,
    v4sigma2tau2: &mut Array<f64>,
    v4sigmalapl3: &mut Array<f64>,
    v4sigmalapl2tau: &mut Array<f64>,
    v4sigmalapltau2: &mut Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t73 = pow_1_3(1.0 / M_PI);
        let t74 = 1.0 / t73;
        let t76 = M_CBRT4;
        let t77 = (-0.9800683e0 / t41 + t47 * t49 + t67 * t69) * t74 * t76;
        let t80 = piecewise3(t3, 0.0, t19 * t77 / 4.0);
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
        let t131 = piecewise3(t3, 0.0, t82 * t77 / 12.0 + t19 * t127 / 4.0);
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
        let t152 = piecewise3(t3, 0.0, t19 * t149 / 4.0);
        let tvsigma0 = 2.0 * rho[ip] * t152;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t156 = t21 * t31;
        let t160 = 1.0 / t18 / t53;
        let t166 = -0.5130928e-3 * t110 * t160 + 0.14299288e-2 * t61 * t21 * t31;
        let t172 = (0.16160736667716e-1 * t134 * t31 - 0.746904e-2 * t99 * t156 + t166 * t69 - 0.1120356e-1 * t122 * t156) * t74 * t76;
        let t175 = piecewise3(t3, 0.0, t19 * t172 / 4.0);
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
        let t268 = 0.50229717333333333333e-3 * t261 + 0.34206186666666666667e-3 * t263 + 0.85515466666666666667e-3 * t266;
        let t270 = t143 * t121;
        let t273 = t227 * t115;
        let t275 = t232 * t256;
        let t277 = t122 * t222;
        let t281 = (0.3453491332284e-2 * t250 + 0.4604655109712e-2 * t252 - 0.373452e-2 * t254 + 0.1120356e-1 * t257 + 0.995872e-2 * t259 + t268 * t69 - 3.0 * t270 * t92 - 0.560178e-2 * t273 + 0.2240712e-1 * t275 + 0.1493808e-1 * t277) * t74 * t76;
        let t285 = piecewise3(t3, 0.0, t82 * t149 / 12.0 + t19 * t281 / 4.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t285 + 2.0 * t152;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t290 = t31 * t92;
        let t296 = t156 * t92;
        let t305 = 0.22234021333333333333e-2 * t139 - 0.95328586666666666667e-2 * tau[ip] * t20 * t160 - 0.23832146666666666667e-2 * t141;
        let t307 = t166 * t121;
        let t317 = (-0.32321473335432e-1 * t248 * t290 - 0.2693456111286e-1 * t135 - 0.746904e-2 * t199 * t156 + 0.2240712e-1 * t202 * t296 + 0.124484e-1 * t137 + t305 * t69 - 3.0 * t307 * t92 - 0.1120356e-1 * t227 * t156 + 0.4481424e-1 * t232 * t296 + 0.186726e-1 * t145) * t74 * t76;
        let t321 = piecewise3(t3, 0.0, t82 * t172 / 12.0 + t19 * t317 / 4.0);
        let tv2rhotau0 = 2.0 * rho[ip] * t321 + 2.0 * t175;
        v2rhotau[ip] += tv2rhotau0;
        let t324 = t69 * t20;
        let t325 = t324 * t56;
        let t327 = t202 * t265;
        let t329 = t270 * t115;
        let t331 = t232 * t265;
        let t335 = (-0.5471779570623875632e-4 * t325 + 0.418399188912e-4 * t327 - 0.1120356e-1 * t329 + 0.836798377824e-4 * t331) * t74 * t76;
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
        let t353 = (-0.58066640491359751264e-3 * t340 + 0.836798377824e-4 * t343 - 0.560178e-2 * t345 - 0.1120356e-1 * t270 * t156 + 0.1673596755648e-3 * t349) * t74 * t76;
        let t356 = piecewise3(t3, 0.0, t19 * t353 / 4.0);
        let tv2sigmatau0 = 2.0 * rho[ip] * t356;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t359 = 1.0 / t18 / t85;
        let t362 = t20 * t359;
        let t371 = (0.52915690831705649747e-2 * t324 * t359 + 0.1673596755648e-3 * t202 * t362 - 0.2240712e-1 * t307 * t156 + 0.3347193511296e-3 * t232 * t362) * t74 * t76;
        let t374 = piecewise3(t3, 0.0, t19 * t371 / 4.0);
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
        let t460 = piecewise3(t3, 0.0, 5.0 / 54.0 * t377 * t77 - t178 * t127 / 6.0 + t82 * t239 / 4.0 + t19 * t456 / 4.0);
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
        let t536 = piecewise3(t3, 0.0, -t178 * t149 / 18.0 + t82 * t281 / 6.0 + t19 * t532 / 4.0);
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
        let t627 = 0.16415338711871626896e-3 * t604 + 0.29182824376660670037e-3 * t606 + 0.418399188912e-4 * t608 - 0.1673596755648e-3 * t611 - 0.2231462340864e-3 * t613 - 0.1120356e-1 * t615 + 0.4481424e-1 * t617 + 0.2987616e-1 * t619 + 0.836798377824e-4 * t621 - 0.418399188912e-3 * t623 - 0.4462924681728e-3 * t625;
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
        let t664 = 0.17419992147407925379e-2 * t639 + 0.25162210879589225548e-2 * t325 + 0.836798377824e-4 * t642 - 0.3347193511296e-3 * t645 - 0.3626126303904e-3 * t327 - 0.560178e-2 * t648 + 0.2240712e-1 * t650 + 0.1493808e-1 * t652 - 0.1120356e-1 * t468 * t156 + 0.4481424e-1 * t491 * t296 + 0.186726e-1 * t329 + 0.1673596755648e-3 * t659 - 0.836798377824e-3 * t661 - 0.7252252607808e-3 * t331;
        let t666 = t664 * t74 * t76;
        let t670 = piecewise3(t3, 0.0, t82 * t353 / 12.0 + t19 * t666 / 4.0);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t670 + 2.0 * t356;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t681 = t362 * t92;
        let t695 = -0.15874707249511694924e-1 * t602 * t359 * t92 - 0.17638563610568549916e-1 * t340 + 0.1673596755648e-3 * t427 * t362 - 0.6694387022592e-3 * t430 * t681 - 0.557865585216e-3 * t343 - 0.2240712e-1 * t544 * t156 + 0.8962848e-1 * t554 * t296 + 0.373452e-1 * t345 + 0.3347193511296e-3 * t415 * t362 - 0.1673596755648e-2 * t420 * t681 - 0.1115731170432e-2 * t349;
        let t697 = t695 * t74 * t76;
        let t701 = piecewise3(t3, 0.0, t82 * t371 / 12.0 + t19 * t697 / 4.0);
        let tv3rhotau20 = 2.0 * rho[ip] * t701 + 2.0 * t374;
        v3rhotau2[ip] += tv3rhotau20;
        let t704 = 1.0 / t392;
        let t705 = t121 * t704;
        let t707 = t430 * t704;
        let t709 = t491 * t265;
        let t711 = t420 * t704;
        let t715 = (0.24257213591226013496e-5 * t705 - 0.625008055590256896e-6 * t707 + 0.2510395133472e-3 * t709 - 0.156252013897564224e-5 * t711) * t74 * t76;
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
        let t733 = (0.13769052686795410687e-4 * t721 - 0.1250016111180513792e-5 * t723 + 0.836798377824e-4 * t725 + 0.3347193511296e-3 * t727 - 0.312504027795128448e-5 * t729) * t74 * t76;
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
        let t751 = (-0.37481073607439962653e-4 * t739 - 0.2500032222361027584e-5 * t741 + 0.3347193511296e-3 * t743 + 0.3347193511296e-3 * t491 * t362 - 0.625008055590256896e-5 * t747) * t74 * t76;
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
        let t767 = (-0.37070930301320389336e-3 * t121 * t756 - 0.5000064444722055168e-5 * t430 * t756 + 0.10041580533888e-2 * t554 * t362 - 0.1250016111180513792e-4 * t420 * t756) * t74 * t76;
        let t770 = piecewise3(t3, 0.0, t19 * t767 / 4.0);
        let tv3tau30 = 2.0 * rho[ip] * t770;
        v3tau3[ip] += tv3tau30;
        let t782 = t183 * t192;
        let t785 = t92 * t412;
        let t796 = t118 * t419;
        let t801 = t67 / t120 / t68;
        let t802 = t183 * t183;
        let t805 = t97 * t231;
        let t808 = t47 * t419;
        let t811 = t192 * t192;
        let t814 = t390 * t69;
        let t817 = t408 * t121;
        let t823 = -360.0 * t420 * t782 + 24.0 * t202 * t785 + 144.0 * t415 * t424 + 48.0 * t232 * t785 + 72.0 * t427 * t424 - 144.0 * t430 * t782 - 240.0 * t796 * t421 + 360.0 * t801 * t802 - 96.0 * t805 * t421 + 120.0 * t808 * t802 + 18.0 * t202 * t811 - 8.0 * t814 * t92 - 12.0 * t817 * t92 - 0.78405464e1 * t69 * t412 * t92;
        let t824 = t225 * t231;
        let t837 = 1.0 / t24 / t102;
        let t838 = t22 * t837;
        let t840 = t29 * t385;
        let t842 = 0.48281349925925925925e0 * t838 + 0.28400794074074074075e0 * t840;
        let t845 = t197 * t121;
        let t854 = t392 * rho[ip];
        let t867 = t21 * t385;
        let t882 = 72.0 * t824 * t183 + 36.0 * t232 * t811 + 0.352824588e2 * t121 * t183 * t192 - 12.0 * t444 * t192 - 8.0 * t199 * t412 - 2.0 * t99 * t842 + 36.0 * t845 * t183 - 18.0 * t451 * t192 - 12.0 * t227 * t412 - 3.0 * t122 * t842 + (-0.97203804839506172842e-1 * t52 / t18 / t854 - 0.3316733210864197531e-1 * t22 * t837 * t61 - 0.41807561481481481483e0 * t110 * t394 * tau[ip] + 0.96034724345679012345e0 * t219 * t209 + 0.10874520256790123457e0 * t114 * t867) * t69 + 0.9800683e0 * t49 * t842 + (-0.91967120829629629629e0 * t838 + 0.95066686814814814815e0 * t840) * t49 - 0.235216392e2 * t231 * t802 - 0.58804098e1 * t69 * t811;
        let t889 = piecewise3(t3, 0.0, -20.0 / 81.0 * t17 * t87 * t77 + 10.0 / 27.0 * t377 * t127 - t178 * t239 / 3.0 + t82 * t456 / 3.0 + t19 * (t823 + t882) * t74 * t76 / 4.0);
        let tv4rho40 = 2.0 * rho[ip] * t889 + 8.0 * t460;
        v4rho4[ip] += tv4rho40;
        let t925 = t231 * t21;
        let t934 = t143 * t419;
        let t937 = t480 * t121;
        let t944 = t268 * t231;
        let t947 = t430 * t21;
        let t948 = t484 * t92;
        let t951 = t420 * t21;
        let t954 = 0.25560714666666666667e0 * t122 * t867 + 0.41441895987408e-1 * t925 * t26 * t421 + 0.82883791974816e-1 * t508 * t87 * t183 + 0.78790765210627555554e-1 * t134 * t385 - 60.0 * t934 * t421 - 9.0 * t937 * t92 - 9.0 * t468 * t192 - 3.0 * t270 * t412 + 36.0 * t944 * t183 - 0.13444272e0 * t947 * t948 - 0.3361068e0 * t951 * t948;
        let t960 = t20 * t209;
        let t971 = t222 * t192;
        let t978 = t115 * t421;
        let t981 = t222 * t183;
        let t993 = t115 * t412;
        let t996 = t405 * t92;
        let t1011 = -0.31081421990556e-1 * t508 * t948 + 0.3361068e-1 * t845 * t256 + 0.3361068e-1 * t427 * t494 + 0.1120356e-1 * t202 * t993 + 0.32863776e0 * t202 * t996 + 0.6722136e-1 * t415 * t494 + 0.2240712e-1 * t232 * t993 + 0.65727552e0 * t232 * t996 + 0.6722136e0 * t801 * t978 + 0.8962848e0 * t420 * t981 - 0.8962848e-1 * t202 * t971;
        let t1019 = piecewise3(t3, 0.0, 5.0 / 54.0 * t377 * t149 - t178 * t281 / 6.0 + t82 * t532 / 4.0 + t19 * (0.2987616e-1 * t444 * t222 + 0.17040476444444444445e0 * t99 * t867 + 0.4481424e-1 * t451 * t222 - 0.27627930658272e-1 * t248 * t87 * t192 - 0.373452e-2 * t814 * t115 - 0.10954592e0 * t199 * t405 + 36.0 * t491 * t424 - 0.560178e-2 * t817 * t115 + 0.3453491332284e-2 * t248 * t26 * t412 + 0.101302412413664e0 * t248 * t187 * t92 + t954 + (0.23328913161481481481e-1 * t110 * t394 + 0.58530586074074074077e-2 * t867 * t61 + 0.51689348740740740742e-1 * t960 * tau[ip]) * t69 - 0.16431888e0 * t227 * t405 - 0.3361068e0 * t796 * t519 - 0.35851392e0 * t415 * t522 - 0.17925696e0 * t232 * t971 - 0.13444272e0 * t805 * t519 - 0.17925696e0 * t427 * t522 + 0.2240712e0 * t808 * t978 + 0.35851392e0 * t430 * t981 + 0.6722136e-1 * t824 * t256 + t1011) * t74 * t76 / 4.0);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t1019 + 6.0 * t536;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = 0.0;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t1029 = t166 * t419;
        let t1032 = t305 * t231;
        let t1035 = t562 * t121;
        let t1042 = t290 * t192;
        let t1073 = -0.32321473335432e-1 * t248 * t31 * t412 - 0.746904e-2 * t814 * t156 - 0.1120356e-1 * t817 * t156 + 36.0 * t554 * t424 - 0.48482210003148e0 * t510 - 0.43095297780576e0 * t513 - 0.995872e-1 * t517 + 0.12171768888888888889e0 * t497 - 0.1493808e0 * t502 + 0.18257653333333333333e0 * t506 + 0.560178e-1 * t482;
        let t1077 = t156 * t421;
        let t1084 = t156 * t412;
        let t1110 = -0.6722136e0 * t796 * t564 + 0.13444272e0 * t415 * t571 + 0.4481424e-1 * t232 * t1084 - 0.1120356e0 * t495 - 0.4481424e0 * t499 - 0.2240712e0 * t504 - 0.2240712e0 * t515 + 0.4481424e0 * t520 + 0.2987616e0 * t523 + 0.1120356e1 * t525 + 0.5975232e0 * t527;
        let t1118 = piecewise3(t3, 0.0, 5.0 / 54.0 * t377 * t172 - t178 * t317 / 6.0 + t82 * t593 / 4.0 + t19 * (-60.0 * t1029 * t421 + 36.0 * t1032 * t183 - 9.0 * t1035 * t92 - 9.0 * t544 * t192 - 3.0 * t307 * t412 - 0.26888544e0 * t947 * t1042 - 0.6722136e0 * t951 * t1042 + (0.7510158317037037037e-1 * t473 - 0.34741973807407407407e0 * t478 - 0.23302543407407407408e-1 * t475) * t69 - 0.26336015310352e0 * t487 - 0.387857680025184e0 * t925 * t31 * t421 + t1073 + 0.16160736667716e0 * t485 + 0.373452e-1 * t489 + 0.4481424e0 * t808 * t1077 + 0.13444272e1 * t801 * t1077 + 0.6722136e-1 * t427 * t571 + 0.2240712e-1 * t202 * t1084 + 0.290893260018888e0 * t508 * t1042 + 0.6722136e-1 * t845 * t296 + 0.13444272e0 * t824 * t296 - 0.26888544e0 * t805 * t564 + t1110) * t74 * t76 / 4.0);
        let tv4rho3tau0 = 2.0 * rho[ip] * t1118 + 6.0 * t597;
        v4rho3tau[ip] += tv4rho3tau0;
        let t1134 = t231 * t20;
        let t1154 = -0.1848245543855175769e-2 * t324 * t209 + 0.5975232e-1 * t468 * t222 - 0.10954592e0 * t270 * t405 + 0.28265189650944e-2 * t232 * t960 - 0.65661354847486507584e-3 * t1134 * t56 * t183 - 0.17509694625996402022e-2 * t602 * t104 * t92 - 0.4462924681728e-3 * t427 * t477 + 0.14132594825472e-2 * t202 * t960 - 0.8925849363456e-3 * t415 * t477 + 0.16415338711871626896e-3 * t602 * t56 * t192 + 0.418399188912e-4 * t845 * t265 - 0.1120356e-1 * t937 * t115;
        let t1163 = t265 * t183;
        let t1166 = t477 * t92;
        let t1173 = t265 * t192;
        let t1184 = 0.836798377824e-4 * t824 * t265 - 0.836798377824e-3 * t796 * t610 - 0.2240712e0 * t934 * t519 - 0.23900928e0 * t491 * t522 + 0.2510395133472e-2 * t801 * t1163 + 0.4462924681728e-2 * t420 * t1166 + 0.836798377824e-3 * t808 * t1163 + 0.17851698726912e-2 * t430 * t1166 - 0.418399188912e-3 * t420 * t1173 - 0.3347193511296e-3 * t805 * t610 - 0.1673596755648e-3 * t430 * t1173 + 0.8962848e-1 * t944 * t256 + 0.4481424e-1 * t491 * t494;
        let t1191 = piecewise3(t3, 0.0, -t178 * t335 / 18.0 + t82 * t629 / 6.0 + t19 * (t1154 + t1184) * t74 * t76 / 4.0);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t1191 + 4.0 * t633;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let t1225 = -0.69679968589631701516e-2 * t1134 * t160 * t183 + 0.2987616e-1 * t544 * t222 - 0.5477296e-1 * t307 * t405 + 0.1673596755648e-3 * t824 * t342 + 0.17419992147407925379e-2 * t602 * t160 * t192 + 0.836798377824e-4 * t845 * t342 - 0.560178e-2 * t1035 * t115 - 0.1120356e-1 * t937 * t156 - 0.497936e-1 * t619 - 0.14504505215616e-2 * t621 + 0.38678680574976e-2 * t625 - 0.15097326527753535329e-1 * t604 - 0.7252252607808e-3 * t608 + 0.19339340287488e-2 * t613 + 0.373452e-1 * t615 + 0.29009010431232e-2 * t611;
        let t1228 = t342 * t192;
        let t1231 = t342 * t183;
        let t1257 = -0.1493808e0 * t617 + 0.7252252607808e-2 * t623 - 0.3347193511296e-3 * t430 * t1228 + 0.5020790266944e-2 * t801 * t1231 + 0.1673596755648e-2 * t808 * t1231 - 0.1120356e0 * t1029 * t519 - 0.11950464e0 * t554 * t522 - 0.2240712e0 * t934 * t564 + 0.4481424e-1 * t1032 * t256 + 0.2240712e-1 * t554 * t494 + 0.8962848e-1 * t944 * t296 + 0.4481424e-1 * t491 * t571 - 0.1673596755648e-2 * t796 * t644 - 0.836798377824e-3 * t420 * t1228 - 0.6694387022592e-3 * t805 * t644 - 0.13419845802447586959e-1 * t606;
        let t1264 = piecewise3(t3, 0.0, -t178 * t353 / 18.0 + t82 * t666 / 6.0 + t19 * (t1225 + t1257) * t74 * t76 / 4.0);
        let tv4rho2sigmatau0 = 2.0 * rho[ip] * t1264 + 4.0 * t670;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let tv4rho2lapl20 = 0.0;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let t1291 = -0.15874707249511694924e-1 * t602 * t359 * t192 + 0.1673596755648e-3 * t845 * t362 + 0.63498828998046779696e-1 * t1134 * t359 * t183 - 0.2240712e-1 * t1035 * t156 + 0.3347193511296e-3 * t824 * t362 - 0.995872e-1 * t652 - 0.2231462340864e-2 * t659 + 0.1058313816634112995e0 * t639 - 0.1115731170432e-2 * t642 + 0.746904e-1 * t648 + 0.76433775645797049636e-1 * t325 + 0.2417417535936e-2 * t327;
        let t1296 = t362 * t192;
        let t1303 = t362 * t183;
        let t1316 = 0.4834835071872e-2 * t331 + 0.4462924681728e-2 * t645 - 0.2987616e0 * t650 + 0.1115731170432e-1 * t661 - 0.6694387022592e-3 * t430 * t1296 + 0.17925696e0 * t1032 * t296 + 0.8962848e-1 * t554 * t571 + 0.3347193511296e-2 * t808 * t1303 - 0.4481424e0 * t1029 * t564 + 0.10041580533888e-1 * t801 * t1303 - 0.3347193511296e-2 * t796 * t681 - 0.1673596755648e-2 * t420 * t1296 - 0.13388774045184e-2 * t805 * t681;
        let t1323 = piecewise3(t3, 0.0, -t178 * t371 / 18.0 + t82 * t697 / 6.0 + t19 * (t1291 + t1316) * t74 * t76 / 4.0);
        let tv4rho2tau20 = 2.0 * rho[ip] * t1323 + 4.0 * t701;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t1331 = 1.0 / t854;
        let t1336 = t704 * t92;
        let t1353 = -0.97028854364904053984e-5 * t231 * t704 * t92 - 0.19405770872980810797e-4 * t121 * t1331 - 0.625008055590256896e-6 * t805 * t704 + 0.312504027795128448e-5 * t808 * t1336 + 0.5000064444722055168e-5 * t430 * t1331 + 0.2510395133472e-3 * t944 * t265 - 0.1255197566736e-2 * t934 * t610 - 0.13388774045184e-2 * t491 * t477 - 0.156252013897564224e-5 * t796 * t704 + 0.937512083385385344e-5 * t801 * t1336 + 0.1250016111180513792e-4 * t420 * t1331;
        let t1359 = piecewise3(t3, 0.0, t82 * t715 / 12.0 + t19 * t1353 * t74 * t76 / 4.0);
        let tv4rhosigma30 = 2.0 * rho[ip] * t1359 + 2.0 * t718;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4rhosigma2lapl0 = 0.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let t1370 = t720 * t92;
        let t1390 = -0.55076210747181642748e-4 * t231 * t720 * t92 - 0.96383368807567874809e-4 * t705 - 0.1250016111180513792e-5 * t805 * t720 + 0.625008055590256896e-5 * t808 * t1370 + 0.8750112778263596544e-5 * t707 + 0.836798377824e-4 * t1032 * t265 - 0.418399188912e-3 * t1029 * t610 - 0.4462924681728e-3 * t554 * t477 + 0.3347193511296e-3 * t944 * t342 - 0.1673596755648e-2 * t934 * t644 - 0.14504505215616e-2 * t709 - 0.312504027795128448e-5 * t796 * t720 + 0.1875024166770770688e-4 * t801 * t1370 + 0.2187528194565899136e-4 * t711;
        let t1396 = piecewise3(t3, 0.0, t82 * t733 / 12.0 + t19 * t1390 * t74 * t76 / 4.0);
        let tv4rhosigma2tau0 = 2.0 * rho[ip] * t1396 + 2.0 * t736;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let tv4rhosigmalapl20 = 0.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let t1407 = t738 * t92;
        let t1426 = 0.14992429442975985061e-3 * t231 * t738 * t92 + 0.22488644164463977592e-3 * t721 - 0.2500032222361027584e-5 * t805 * t738 + 0.1250016111180513792e-4 * t808 * t1407 + 0.15000193334166165504e-4 * t723 + 0.3347193511296e-3 * t1032 * t342 - 0.1673596755648e-2 * t1029 * t644 - 0.14504505215616e-2 * t725 + 0.3347193511296e-3 * t944 * t362 - 0.1673596755648e-2 * t934 * t681 - 0.1115731170432e-2 * t727 - 0.625008055590256896e-5 * t796 * t738 + 0.3750048333541541376e-4 * t801 * t1407 + 0.3750048333541541376e-4 * t729;
        let t1432 = piecewise3(t3, 0.0, t82 * t751 / 12.0 + t19 * t1426 * t74 * t76 / 4.0);
        let tv4rhosigmatau20 = 2.0 * rho[ip] * t1432 + 2.0 * t754;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let t1443 = t756 * t92;
        let t1457 = 0.14828372120528155734e-2 * t231 * t756 * t92 + 0.18535465150660194668e-2 * t739 - 0.5000064444722055168e-5 * t805 * t756 + 0.2500032222361027584e-4 * t808 * t1443 + 0.2500032222361027584e-4 * t741 + 0.10041580533888e-2 * t1032 * t362 - 0.5020790266944e-2 * t1029 * t681 - 0.3347193511296e-2 * t743 - 0.1250016111180513792e-4 * t796 * t756 + 0.7500096667083082752e-4 * t801 * t1443 + 0.625008055590256896e-4 * t747;
        let t1463 = piecewise3(t3, 0.0, t82 * t767 / 12.0 + t19 * t1457 * t74 * t76 / 4.0);
        let tv4rhotau30 = 2.0 * rho[ip] * t1463 + 2.0 * t770;
        v4rhotau3[ip] += tv4rhotau30;
        let t1468 = 1.0 / t24 / t392 / t23;
        let t1472 = t1468 * t21;
        let t1484 = piecewise3(t3, 0.0, t19 * (-0.39537870823603221723e-7 * t231 * t1468 * t21 + 0.58352627094073154581e-8 * t808 * t1472 - 0.625008055590256896e-5 * t934 * t704 + 0.17505788128221946374e-7 * t801 * t1472) * t74 * t76 / 4.0);
        let tv4sigma40 = 2.0 * rho[ip] * t1484;
        v4sigma4[ip] += tv4sigma40;
        let tv4sigma3lapl0 = 0.0;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let t1487 = 1.0 / t24 / t854;
        let t1491 = t1487 * t21;
        let t1505 = piecewise3(t3, 0.0, t19 * (-0.17285519473436482498e-6 * t231 * t1487 * t21 + 0.11670525418814630916e-7 * t808 * t1491 - 0.156252013897564224e-5 * t1029 * t704 - 0.937512083385385344e-5 * t934 * t720 + 0.35011576256443892749e-7 * t801 * t1491) * t74 * t76 / 4.0);
        let tv4sigma3tau0 = 2.0 * rho[ip] * t1505;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let tv4sigma2lapl20 = 0.0;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let t1508 = 1.0 / t24 / t392;
        let t1512 = t1508 * t21;
        let t1526 = piecewise3(t3, 0.0, t19 * (0.85573524052432245038e-7 * t231 * t1508 * t21 + 0.23341050837629261832e-7 * t808 * t1512 - 0.625008055590256896e-5 * t1029 * t720 - 0.1250016111180513792e-4 * t934 * t738 + 0.70023152512887785497e-7 * t801 * t1512) * t74 * t76 / 4.0);
        let tv4sigma2tau20 = 2.0 * rho[ip] * t1526;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let tv4sigmalapl30 = 0.0;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let t1529 = 1.0 / t24 / t207;
        let t1533 = t1529 * t21;
        let t1547 = piecewise3(t3, 0.0, t19 * (0.22714005145381455963e-5 * t231 * t1529 * t21 + 0.46682101675258523665e-7 * t808 * t1533 - 0.1875024166770770688e-4 * t1029 * t738 - 0.1250016111180513792e-4 * t934 * t756 + 0.14004630502577557099e-6 * t801 * t1533) * t74 * t76 / 4.0);
        let tv4sigmatau30 = 2.0 * rho[ip] * t1547;
        v4sigmatau3[ip] += tv4sigmatau30;
        let tv4lapl40 = 0.0;
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let t1552 = t837 * t21;
        let t1564 = piecewise3(t3, 0.0, t19 * (0.11218679240724768037e-4 * t231 * t837 * t21 + 0.9336420335051704733e-7 * t808 * t1552 - 0.5000064444722055168e-4 * t1029 * t756 + 0.28009261005155114199e-6 * t801 * t1552) * t74 * t76 / 4.0);
        let tv4tau40 = 2.0 * rho[ip] * t1564;
        v4tau4[ip] += tv4tau40;
    }
}
