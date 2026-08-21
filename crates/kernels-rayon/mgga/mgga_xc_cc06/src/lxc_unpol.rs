//! MGGA_XC_CC06 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_cc06.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_cc06_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho3lapl: &mut [f64],
    v4rho3tau: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rho2sigmalapl: &mut [f64],
    v4rho2sigmatau: &mut [f64],
    v4rho2lapl2: &mut [f64],
    v4rho2lapltau: &mut [f64],
    v4rho2tau2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4rhosigma2lapl: &mut [f64],
    v4rhosigma2tau: &mut [f64],
    v4rhosigmalapl2: &mut [f64],
    v4rhosigmalapltau: &mut [f64],
    v4rhosigmatau2: &mut [f64],
    v4rholapl3: &mut [f64],
    v4rholapl2tau: &mut [f64],
    v4rholapltau2: &mut [f64],
    v4rhotau3: &mut [f64],
    v4sigma4: &mut [f64],
    v4sigma3lapl: &mut [f64],
    v4sigma3tau: &mut [f64],
    v4sigma2lapl2: &mut [f64],
    v4sigma2lapltau: &mut [f64],
    v4sigma2tau2: &mut [f64],
    v4sigmalapl3: &mut [f64],
    v4sigmalapl2tau: &mut [f64],
    v4sigmalapltau2: &mut [f64],
    v4sigmatau3: &mut [f64],
    v4lapl4: &mut [f64],
    v4lapl3tau: &mut [f64],
    v4lapl2tau2: &mut [f64],
    v4lapltau3: &mut [f64],
    v4tau4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t9 = pow_1_3(zeta_threshold);
        let t11 = piecewise3(1.0 <= zeta_threshold, t9 * zeta_threshold, 1.0);
        let t12 = pow_1_3(rho[ip]);
        let t16 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t11 * t12);
        let t18 = 1.0 / M_PI;
        let t19 = pow_1_3(t18);
        let t20 = t4 * t19;
        let t21 = M_CBRT4;
        let t22 = t21 * t21;
        let t25 = t20 * t22 / t12;
        let t27 = 1.0 + 0.053425 * t25;
        let t28 = rmath::sqrt(t25);
        let t31 = pow_3_2(t25);
        let t33 = t4 * t4;
        let t34 = t19 * t19;
        let t35 = t33 * t34;
        let t36 = t12 * t12;
        let t37 = 1.0 / t36;
        let t39 = t35 * t21 * t37;
        let t41 = 3.79785 * t28 + 0.8969 * t25 + 0.204775 * t31 + 0.123235 * t39;
        let t44 = 1.0 + 16.081824322151103 / t41;
        let t45 = rmath::ln(t44);
        let t50 = M_CBRT2;
        let t54 = (2.0 * t11 - 2.0) / (2.0 * t50 - 2.0);
        let t56 = 1.0 + 0.0278125 * t25;
        let t61 = 5.1785 * t28 + 0.905775 * t25 + 0.1100325 * t31 + 0.1241775 * t39;
        let t64 = 1.0 + 29.608574643216677 / t61;
        let t65 = rmath::ln(t64);
        let t69 = 2.0 * t16 - 0.062182 * t27 * t45 + 0.019751789702565206 * t54 * t56 * t65;
        let t70 = t33 * t21;
        let t71 = t34 * lapl[ip];
        let t73 = 1.0 / t36 / rho[ip];
        let t75 = t70 * t71 * t73;
        let t77 = -0.0007 + 0.002 * t75;
        let t79 = 1.0 + 0.0065 * t75;
        let t80 = 1.0 / t79;
        let t82 = t77 * t80 + 1.0;
        let tzk0 = t69 * t82;
        zk[ip] += tzk0;
        let t86 = piecewise3(t3, 0.0, -t7 * t11 * t37 / 8.0);
        let t89 = 1.0 / t12 / rho[ip];
        let t90 = t22 * t89;
        let t94 = t41 * t41;
        let t95 = 1.0 / t94;
        let t96 = t27 * t95;
        let t98 = 1.0 / t28 * t4;
        let t99 = t19 * t22;
        let t100 = t99 * t89;
        let t101 = t98 * t100;
        let t103 = t20 * t90;
        let t105 = rmath::sqrt(t25);
        let t106 = t105 * t4;
        let t107 = t106 * t100;
        let t109 = t21 * t73;
        let t110 = t35 * t109;
        let t112 = -0.632975 * t101 - 0.29896666666666666 * t103 - 0.1023875 * t107 - 0.08215666666666667 * t110;
        let t113 = 1.0 / t44;
        let t114 = t112 * t113;
        let t117 = t54 * t4;
        let t122 = t54 * t56;
        let t123 = t61 * t61;
        let t124 = 1.0 / t123;
        let t129 = -0.8630833333333333 * t101 - 0.301925 * t103 - 0.05501625 * t107 - 0.082785 * t110;
        let t131 = 1.0 / t64;
        let t132 = t124 * t129 * t131;
        let t135 = 2.0 * t86 + 0.0011073577833333333 * t20 * t90 * t45 + 1.0 * t96 * t114 - 0.0001831155503675316 * t117 * t99 * t89 * t65 - 0.5848223397455204 * t122 * t132;
        let t136 = rho[ip] * t135;
        let t138 = rho[ip] * t69;
        let t139 = t70 * t34;
        let t140 = rho[ip] * rho[ip];
        let t142 = 1.0 / t36 / t140;
        let t143 = lapl[ip] * t142;
        let t147 = t79 * t79;
        let t148 = 1.0 / t147;
        let t150 = t77 * t148 * t33;
        let t151 = t21 * t34;
        let t155 = -0.0033333333333333335 * t139 * t143 * t80 + 0.010833333333333334 * t150 * t151 * t143;
        let tvrho0 = t136 * t82 + t138 * t155 + tzk0;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t163 = 0.002 * t35 * t109 * t80 - 0.0065 * t150 * t151 * t73;
        let tvlapl0 = t138 * t163;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t171 = piecewise3(t3, 0.0, t7 * t11 * t73 / 12.0);
        let t174 = 1.0 / t12 / t140;
        let t175 = t22 * t174;
        let t179 = t20 * t22;
        let t180 = t89 * t95;
        let t184 = t94 * t41;
        let t185 = 1.0 / t184;
        let t186 = t27 * t185;
        let t187 = t112 * t112;
        let t188 = t187 * t113;
        let t193 = 1.0 / t28 / t25 * t33;
        let t194 = t151 * t142;
        let t195 = t193 * t194;
        let t197 = t99 * t174;
        let t198 = t98 * t197;
        let t200 = t20 * t175;
        let t202 = 1.0/rmath::sqrt(t25);
        let t203 = t202 * t33;
        let t204 = t203 * t194;
        let t206 = t106 * t197;
        let t208 = t21 * t142;
        let t209 = t35 * t208;
        let t211 = -0.4219833333333333 * t195 + 0.8439666666666666 * t198 + 0.3986222222222222 * t200 + 0.06825833333333334 * t204 + 0.13651666666666668 * t206 + 0.1369277777777778 * t209;
        let t212 = t211 * t113;
        let t215 = t94 * t94;
        let t216 = 1.0 / t215;
        let t217 = t27 * t216;
        let t218 = t44 * t44;
        let t219 = 1.0 / t218;
        let t220 = t187 * t219;
        let t227 = t54 * t20;
        let t231 = t123 * t61;
        let t232 = 1.0 / t231;
        let t233 = t129 * t129;
        let t235 = t232 * t233 * t131;
        let t244 = -0.5753888888888888 * t195 + 1.1507777777777777 * t198 + 0.4025666666666667 * t200 + 0.0366775 * t204 + 0.073355 * t206 + 0.137975 * t209;
        let t246 = t124 * t244 * t131;
        let t249 = t123 * t123;
        let t250 = 1.0 / t249;
        let t251 = t250 * t233;
        let t252 = t64 * t64;
        let t253 = 1.0 / t252;
        let t254 = t251 * t253;
        let t257 = 2.0 * t171 - 0.0014764770444444443 * t20 * t175 * t45 - 0.035616666666666665 * t179 * t180 * t114 - 2.0 * t186 * t188 + 1.0 * t96 * t212 + 16.081824322151103 * t217 * t220 + 0.0002441540671567088 * t117 * t99 * t174 * t65 + 0.010843580882781523 * t227 * t90 * t132 + 1.169644679491041 * t122 * t235 - 0.5848223397455204 * t122 * t246 - 17.315755899375862 * t122 * t254;
        let t258 = rho[ip] * t257;
        let t262 = t140 * rho[ip];
        let t264 = 1.0 / t36 / t262;
        let t265 = lapl[ip] * t264;
        let t270 = t19 * t18;
        let t271 = t4 * t22 * t270;
        let t272 = lapl[ip] * lapl[ip];
        let t273 = t140 * t140;
        let t274 = t273 * rho[ip];
        let t276 = 1.0 / t12 / t274;
        let t277 = t272 * t276;
        let t282 = 1.0 / t147 / t79;
        let t284 = t77 * t282 * t4;
        let t285 = t22 * t270;
        let t292 = 0.008888888888888889 * t139 * t265 * t80 - 0.00021666666666666666 * t271 * t277 * t148 + 0.0007041666666666666 * t284 * t285 * t277 - 0.028888888888888888 * t150 * t151 * t265;
        let tv2rho20 = 2.0 * t135 * t82 + 2.0 * t136 * t155 + t138 * t292 + 2.0 * t69 * t155 + t258 * t82;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t300 = 1.0 / t12 / t273;
        let t311 = -0.0033333333333333335 * t35 * t208 * t80 + 0.00013 * t271 * t300 * t148 * lapl[ip] - 0.0004225 * t284 * t285 * t300 * lapl[ip] + 0.010833333333333334 * t150 * t194;
        let tv2rholapl0 = t136 * t163 + t138 * t311 + t69 * t163;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t313 = t4 * t270;
        let t315 = 1.0 / t12 / t262;
        let t316 = t22 * t315;
        let t323 = -7.8e-05 * t313 * t316 * t148 + 0.0002535 * t284 * t285 * t315;
        let tv2lapl20 = t138 * t323;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        let t331 = 1.0 / t215 / t94;
        let t332 = t27 * t331;
        let t333 = t187 * t112;
        let t335 = 1.0 / t218 / t44;
        let t336 = t333 * t335;
        let t340 = 1.0 / t249 / t123;
        let t341 = t233 * t129;
        let t344 = 1.0 / t252 / t64;
        let t345 = t340 * t341 * t344;
        let t349 = 1.0 / t249 / t61;
        let t351 = t349 * t341 * t253;
        let t357 = 1.0 / t28 / t39 * t18 / 4.0;
        let t358 = 1.0 / t273;
        let t359 = t357 * t358;
        let t361 = t151 * t264;
        let t362 = t193 * t361;
        let t364 = t99 * t315;
        let t365 = t98 * t364;
        let t367 = t20 * t316;
        let t369 = 1.0/pow_3_2(t25);
        let t370 = t369 * t18;
        let t371 = t370 * t358;
        let t373 = t203 * t361;
        let t375 = t106 * t364;
        let t377 = t21 * t264;
        let t378 = t35 * t377;
        let t380 = -3.4523333333333333 * t359 + 2.3015555555555554 * t362 - 2.6851481481481483 * t365 - 0.9393222222222222 * t367 + 0.073355 * t371 - 0.14671 * t373 - 0.17116166666666666 * t375 - 0.36793333333333333 * t378;
        let t382 = t124 * t380 * t131;
        let t385 = t211 * t219;
        let t392 = t333 * t113;
        let t398 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t11 * t142);
        let t401 = 1.0 / t215 / t41;
        let t402 = t27 * t401;
        let t403 = t333 * t219;
        let t414 = -2.5319 * t359 + 1.6879333333333333 * t362 - 1.9692555555555555 * t365 - 0.9301185185185186 * t367 + 0.13651666666666668 * t371 - 0.27303333333333335 * t373 - 0.31853888888888887 * t375 - 0.36514074074074077 * t378;
        let t415 = t414 * t113;
        let t421 = 517.2501470570617 * t332 * t336 - 1025.3897021007795 * t122 * t345 + 103.89453539625518 * t122 * t351 - 0.5848223397455204 * t122 * t382 + 48.24547296645331 * t217 * t385 * t112 - 6.0 * t186 * t114 * t211 + 6.0 * t217 * t392 + 2.0 * t398 - 96.49094593290663 * t402 * t403 + 1.0 * t96 * t415 + 0.0034451131037037037 * t20 * t316 * t45;
        let t432 = t253 * t129;
        let t437 = t250 * t341 * t131;
        let t441 = t131 * t244;
        let t445 = t89 * t216;
        let t449 = t174 * t95;
        let t467 = -0.021687161765563047 * t227 * t175 * t132 + 0.016265371324172287 * t227 * t90 * t246 + 0.4815944609513912 * t227 * t90 * t254 - 51.94726769812759 * t122 * t250 * t244 * t432 - 3.5089340384731225 * t122 * t437 + 3.5089340384731225 * t122 * t232 * t129 * t441 - 0.8591714644109227 * t179 * t445 * t220 + 0.07123333333333333 * t179 * t449 * t114 - 0.053425 * t179 * t180 * t212 - 0.032530742648344574 * t227 * t90 * t235 - 0.0005696928233656539 * t117 * t99 * t315 * t65 + 0.10685 * t179 * t89 * t185 * t188;
        let t468 = t421 + t467;
        let t469 = rho[ip] * t468;
        let t476 = 1.0 / t36 / t273;
        let t477 = lapl[ip] * t476;
        let t481 = t273 * t140;
        let t483 = 1.0 / t12 / t481;
        let t484 = t272 * t483;
        let t488 = t272 * lapl[ip];
        let t489 = t273 * t273;
        let t490 = 1.0 / t489;
        let t491 = t488 * t490;
        let t494 = t147 * t147;
        let t495 = 1.0 / t494;
        let t496 = t77 * t495;
        let t505 = -0.03259259259259259 * t139 * t477 * t80 + 0.0017333333333333333 * t271 * t484 * t148 - 8.561640017777542e-06 * t491 * t282 + 2.782533005777701e-05 * t496 * t491 - 0.005633333333333333 * t284 * t285 * t484 + 0.10592592592592592 * t150 * t151 * t477;
        let tv3rho30 = 6.0 * t135 * t155 + 3.0 * t136 * t292 + t138 * t505 + 3.0 * t258 * t155 + 3.0 * t257 * t82 + 3.0 * t69 * t292 + t469 * t82;
        v3rho3[ip] += tv3rho30;
        let tv3rho2sigma0 = 0.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t521 = t273 * t262;
        let t522 = 1.0 / t521;
        let t523 = t522 * t282;
        let t535 = 0.008888888888888889 * t35 * t377 * t80 - 0.00078 * t271 * t276 * t148 * lapl[ip] + 5.136984010666525e-06 * t523 * t272 - 1.6695198034666206e-05 * t496 * t522 * t272 + 0.002535 * t284 * t285 * t276 * lapl[ip] - 0.028888888888888888 * t150 * t361;
        let tv3rho2lapl0 = 2.0 * t135 * t163 + 2.0 * t136 * t311 + t138 * t535 + t258 * t163 + 2.0 * t69 * t311;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let tv3rho2tau0 = 0.0;
        v3rho2tau[ip] += tv3rho2tau0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t539 = t22 * t300;
        let t543 = 1.0 / t481;
        let t544 = t543 * t282;
        let t553 = 0.00026 * t313 * t539 * t148 - 3.082190406399915e-06 * t544 * lapl[ip] + 1.0017118820799724e-05 * t496 * t543 * lapl[ip] - 0.000845 * t284 * t285 * t300;
        let tv3rholapl20 = t136 * t323 + t138 * t553 + t69 * t323;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
        v3rhotau2[ip] += tv3rhotau20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let tv3sigma2tau0 = 0.0;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let tv3sigmatau20 = 0.0;
        v3sigmatau2[ip] += tv3sigmatau20;
        let t555 = 1.0 / t274;
        let t560 = 1.849314243839949e-06 * t555 * t282 - 6.010271292479834e-06 * t496 * t555;
        let tv3lapl30 = t138 * t560;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let tv3tau30 = 0.0;
        v3tau3[ip] += tv3tau30;
        let t572 = piecewise3(t3, 0.0, 10.0 / 27.0 * t7 * t11 * t264);
        let t591 = 1.0 / t28 * rho[ip] * t276 * t179 / 48.0;
        let t593 = t357 * t555;
        let t595 = t151 * t476;
        let t596 = t193 * t595;
        let t598 = t99 * t300;
        let t599 = t98 * t598;
        let t601 = t20 * t539;
        let t603 = rmath::pow(t25, -2.5);
        let t606 = t603 * t18 * t276 * t179;
        let t608 = t370 * t555;
        let t610 = t203 * t595;
        let t612 = t106 * t598;
        let t614 = t21 * t476;
        let t615 = t35 * t614;
        let t621 = t215 * t215;
        let t624 = t187 * t187;
        let t625 = t218 * t218;
        let t639 = t211 * t211;
        let t649 = 2.0 * t572 + 3103.50088234237 * t332 * t187 * t335 * t211 + 36.0 * t217 * t188 * t211 - 578.9456755974397 * t402 * t385 * t187 + 1.0 * t96 * (-2.109916666666667 * t591 + 20.2552 * t593 - 7.501925925925926 * t596 + 6.564185185185186 * t599 + 3.100395061728395 * t601 + 0.06825833333333334 * t606 - 1.0921333333333334 * t608 + 1.2134814814814814 * t610 + 1.0617962962962963 * t612 + 1.3388493827160495 * t615) * t113 + 24954.97798673547 * t27 / t621 * t624 / t625 + 578.9456755974397 * t332 * t624 * t219 - 6207.00176468474 * t27 / t215 / t184 * t624 * t335 + 48.24547296645331 * t217 * t639 * t219 - 6.0 * t186 * t639 * t113 - 24.0 * t402 * t624 * t113;
        let t652 = t233 * t233;
        let t661 = t249 * t249;
        let t664 = t252 * t252;
        let t684 = t244 * t244;
        let t716 = 12304.676425209354 * t122 / t249 / t231 * t652 * t344 - 623.3672123775311 * t122 * t340 * t652 * t253 - 91080.98259910992 * t122 / t661 * t652 / t664 - 0.5848223397455204 * t122 * t124 * (-2.8769444444444443 * t591 + 27.618666666666666 * t593 - 10.229135802469136 * t596 + 8.950493827160495 * t599 + 3.131074074074074 * t601 + 0.0366775 * t606 - 0.58684 * t608 + 0.6520444444444444 * t610 + 0.5705388888888889 * t612 + 1.3490888888888888 * t615) * t131 + 3.5089340384731225 * t122 * t232 * t684 * t131 - 51.94726769812759 * t122 * t250 * t684 * t253 + 14.03573615389249 * t122 * t349 * t652 * t131 - 8.0 * t186 * t114 * t414 + 64.32729728860441 * t217 * t414 * t219 * t112 - 0.011483710345679013 * t20 * t539 * t45 - 3.436685857643691 * t103 * t216 * t211 * t219 * t112 + 0.4274 * t103 * t185 * t112 * t212;
        let t742 = t54 * t179;
        let t744 = t244 * t253;
        let t762 = -0.043374323531126094 * t227 * t175 * t246 - 1.2842518958703766 * t227 * t175 * t254 + 0.1301229705933783 * t227 * t90 * t437 + 38.02486811957057 * t227 * t90 * t345 - 3.8527556876111295 * t227 * t90 * t351 + 0.021687161765563047 * t227 * t90 * t382 + 0.06747116993730726 * t227 * t316 * t132 + 0.08674864706225219 * t227 * t175 * t235 + 1.9263778438055648 * t742 * t89 * t250 * t744 * t129 - 0.1301229705933783 * t742 * t89 * t232 * t441 * t129 + 2.291123905095794 * t179 * t174 * t216 * t220 - 0.2849333333333333 * t179 * t174 * t185 * t188;
        let t809 = 0.14246666666666666 * t179 * t449 * t212 - 21.053604230838733 * t122 * t251 * t441 - 0.07123333333333333 * t179 * t180 * t415 - 0.22161481481481482 * t179 * t315 * t95 * t114 - 0.4274 * t179 * t445 * t392 + 6.873371715287382 * t179 * t89 * t401 * t403 + 4.678578717964164 * t122 * t232 * t380 * t131 * t129 - 69.26302359750345 * t122 * t250 * t380 * t432 - 36.84545214203136 * t179 * t89 * t331 * t336 - 6152.338212604677 * t122 * t340 * t233 * t344 * t244 + 623.3672123775311 * t122 * t349 * t233 * t744 + 0.0018989760778855128 * t117 * t99 * t300 * t65;
        let t822 = lapl[ip] / t36 / t274;
        let t828 = t272 / t12 / t521;
        let t832 = t489 * rho[ip];
        let t834 = t488 / t832;
        let t837 = t272 * t272;
        let t840 = 1.0 / t36 / t489 / t140;
        let t847 = t77 / t494 / t79;
        let tv4rho40 = 4.0 * t468 * t82 + 12.0 * t257 * t155 + 12.0 * t135 * t292 + 4.0 * t69 * t505 + rho[ip] * (t649 + t716 + t762 + t809) * t82 + 4.0 * t469 * t155 + 6.0 * t258 * t292 + 4.0 * t136 * t505 + t138 * (0.15209876543209877 * t139 * t822 * t80 - 0.013096296296296297 * t271 * t828 * t148 + 0.00013698624028444067 * t834 * t282 - 3.7100440077036016e-07 * t837 * t840 * t495 * t139 + 1.2057643025036704e-06 * t847 * t837 * t840 * t33 * t151 - 0.00044520528092443216 * t496 * t834 + 0.04256296296296296 * t284 * t285 * t828 - 0.494320987654321 * t150 * t151 * t822);
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = 0.0;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t885 = 1.0 / t36 / t832;
        let tv4rho3lapl0 = 3.0 * t257 * t163 + 6.0 * t135 * t311 + 3.0 * t69 * t535 + t469 * t163 + 3.0 * t258 * t311 + 3.0 * t136 * t535 + t138 * (-0.03259259259259259 * t35 * t614 * t80 + 0.004737777777777777 * t271 * t483 * t148 * lapl[ip] - 6.678079213866482e-05 * t490 * t282 * t272 + 2.2260264046221607e-07 * t885 * t495 * t488 * t139 - 7.234585815022023e-07 * t847 * t885 * t488 * t33 * t151 + 0.00021703757445066068 * t496 * t490 * t272 - 0.015397777777777778 * t284 * t285 * t483 * lapl[ip] + 0.10592592592592592 * t150 * t595);
        v4rho3lapl[ip] += tv4rho3lapl0;
        let tv4rho3tau0 = 0.0;
        v4rho3tau[ip] += tv4rho3tau0;
        let tv4rho2sigma20 = 0.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let tv4rho2sigmatau0 = 0.0;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let t920 = 1.0 / t36 / t489;
        let tv4rho2lapl20 = 2.0 * t135 * t323 + 2.0 * t69 * t553 + t258 * t323 + 2.0 * t136 * t553 + t138 * (-0.0011266666666666667 * t313 * t22 * t276 * t148 + 2.876711045973254e-05 * t523 * lapl[ip] - 1.3356158427732964e-07 * t920 * t495 * t272 * t139 + 4.3407514890132136e-07 * t847 * t920 * t272 * t33 * t151 - 9.349310899413076e-05 * t496 * t522 * lapl[ip] + 0.003661666666666667 * t284 * t285 * t276);
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let tv4rho2tau20 = 0.0;
        v4rho2tau2[ip] += tv4rho2tau20;
        let tv4rhosigma30 = 0.0;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4rhosigma2lapl0 = 0.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let tv4rhosigma2tau0 = 0.0;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let tv4rhosigmalapl20 = 0.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let tv4rhosigmatau20 = 0.0;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let t942 = 1.0 / t36 / t521;
        let tv4rholapl30 = t69 * t560 + t136 * t560 + t138 * (-9.246571219199744e-06 * t544 + 8.013695056639779e-08 * t942 * t495 * t33 * t151 * lapl[ip] - 2.604450893407928e-07 * t847 * t942 * t70 * t71 + 3.005135646239917e-05 * t496 * t543);
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let tv4rhotau30 = 0.0;
        v4rhotau3[ip] += tv4rhotau30;
        let tv4sigma40 = 0.0;
        v4sigma4[ip] += tv4sigma40;
        let tv4sigma3lapl0 = 0.0;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let tv4sigma3tau0 = 0.0;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let tv4sigma2lapl20 = 0.0;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let tv4sigma2tau20 = 0.0;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let tv4sigmalapl30 = 0.0;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let tv4sigmatau30 = 0.0;
        v4sigmatau3[ip] += tv4sigmatau30;
        let t957 = 1.0 / t36 / t481;
        let tv4lapl40 = t138 * (-4.808217033983867e-08 * t957 * t495 * t139 + 1.5626705360447568e-07 * t847 * t957 * t139);
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let tv4tau40 = 0.0;
        v4tau4[ip] += tv4tau40;
    }
}
