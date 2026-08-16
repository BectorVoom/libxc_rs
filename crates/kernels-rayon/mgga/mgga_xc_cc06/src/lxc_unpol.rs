//! MGGA_XC_CC06 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_cc06.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

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
        let t27 = 1.0 + 0.53425e-1 * t25;
        let t28 = f64::sqrt(t25);
        let t31 = pow_3_2(t25);
        let t33 = t4 * t4;
        let t34 = t19 * t19;
        let t35 = t33 * t34;
        let t36 = t12 * t12;
        let t37 = 1.0 / t36;
        let t39 = t35 * t21 * t37;
        let t41 = 0.379785e1 * t28 + 0.8969e0 * t25 + 0.204775e0 * t31 + 0.123235e0 * t39;
        let t44 = 1.0 + 0.16081824322151104822e2 / t41;
        let t45 = f64::ln(t44);
        let t50 = M_CBRT2;
        let t54 = (2.0 * t11 - 2.0) / (2.0 * t50 - 2.0);
        let t56 = 1.0 + 0.278125e-1 * t25;
        let t61 = 0.51785e1 * t28 + 0.905775e0 * t25 + 0.1100325e0 * t31 + 0.1241775e0 * t39;
        let t64 = 1.0 + 0.29608574643216675549e2 / t61;
        let t65 = f64::ln(t64);
        let t69 = 2.0 * t16 - 0.62182e-1 * t27 * t45 + 0.19751789702565206229e-1 * t54 * t56 * t65;
        let t70 = t33 * t21;
        let t71 = t34 * lapl[ip];
        let t73 = 1.0 / t36 / rho[ip];
        let t75 = t70 * t71 * t73;
        let t77 = -0.7e-3 + 0.2e-2 * t75;
        let t79 = 1.0 + 0.65e-2 * t75;
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
        let t105 = f64::sqrt(t25);
        let t106 = t105 * t4;
        let t107 = t106 * t100;
        let t109 = t21 * t73;
        let t110 = t35 * t109;
        let t112 = -0.632975e0 * t101 - 0.29896666666666666667e0 * t103 - 0.1023875e0 * t107 - 0.82156666666666666667e-1 * t110;
        let t113 = 1.0 / t44;
        let t114 = t112 * t113;
        let t117 = t54 * t4;
        let t122 = t54 * t56;
        let t123 = t61 * t61;
        let t124 = 1.0 / t123;
        let t129 = -0.86308333333333333334e0 * t101 - 0.301925e0 * t103 - 0.5501625e-1 * t107 - 0.82785e-1 * t110;
        let t131 = 1.0 / t64;
        let t132 = t124 * t129 * t131;
        let t135 = 2.0 * t86 + 0.11073577833333333333e-2 * t20 * t90 * t45 + 1.0 * t96 * t114 - 0.18311555036753159941e-3 * t117 * t99 * t89 * t65 - 0.58482233974552040708e0 * t122 * t132;
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
        let t155 = -0.33333333333333333333e-2 * t139 * t143 * t80 + 0.10833333333333333333e-1 * t150 * t151 * t143;
        let tvrho0 = t136 * t82 + t138 * t155 + tzk0;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t163 = 0.2e-2 * t35 * t109 * t80 - 0.65e-2 * t150 * t151 * t73;
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
        let t202 = 1.0/f64::sqrt(t25);
        let t203 = t202 * t33;
        let t204 = t203 * t194;
        let t206 = t106 * t197;
        let t208 = t21 * t142;
        let t209 = t35 * t208;
        let t211 = -0.42198333333333333333e0 * t195 + 0.84396666666666666666e0 * t198 + 0.39862222222222222223e0 * t200 + 0.68258333333333333333e-1 * t204 + 0.13651666666666666667e0 * t206 + 0.13692777777777777778e0 * t209;
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
        let t244 = -0.57538888888888888889e0 * t195 + 0.11507777777777777778e1 * t198 + 0.40256666666666666667e0 * t200 + 0.366775e-1 * t204 + 0.73355e-1 * t206 + 0.137975e0 * t209;
        let t246 = t124 * t244 * t131;
        let t249 = t123 * t123;
        let t250 = 1.0 / t249;
        let t251 = t250 * t233;
        let t252 = t64 * t64;
        let t253 = 1.0 / t252;
        let t254 = t251 * t253;
        let t257 = 2.0 * t171 - 0.14764770444444444444e-2 * t20 * t175 * t45 - 0.35616666666666666667e-1 * t179 * t180 * t114 - 2.0 * t186 * t188 + 1.0 * t96 * t212 + 0.16081824322151104822e2 * t217 * t220 + 0.24415406715670879921e-3 * t117 * t99 * t174 * t65 + 0.10843580882781524214e-1 * t227 * t90 * t132 + 0.11696446794910408142e1 * t122 * t235 - 0.58482233974552040708e0 * t122 * t246 - 0.17315755899375863299e2 * t122 * t254;
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
        let t292 = 0.88888888888888888888e-2 * t139 * t265 * t80 - 0.21666666666666666666e-3 * t271 * t277 * t148 + 0.70416666666666666662e-3 * t284 * t285 * t277 - 0.28888888888888888888e-1 * t150 * t151 * t265;
        let tv2rho20 = 2.0 * t135 * t82 + 2.0 * t136 * t155 + t138 * t292 + 2.0 * t69 * t155 + t258 * t82;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t300 = 1.0 / t12 / t273;
        let t311 = -0.33333333333333333333e-2 * t35 * t208 * t80 + 0.13e-3 * t271 * t300 * t148 * lapl[ip] - 0.42249999999999999999e-3 * t284 * t285 * t300 * lapl[ip] + 0.10833333333333333333e-1 * t150 * t194;
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
        let t323 = -0.78e-4 * t313 * t316 * t148 + 0.2535e-3 * t284 * t285 * t315;
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
        let t380 = -0.34523333333333333333e1 * t359 + 0.23015555555555555556e1 * t362 - 0.26851481481481481482e1 * t365 - 0.93932222222222222223e0 * t367 + 0.73355e-1 * t371 - 0.14671e0 * t373 - 0.17116166666666666667e0 * t375 - 0.36793333333333333333e0 * t378;
        let t382 = t124 * t380 * t131;
        let t385 = t211 * t219;
        let t392 = t333 * t113;
        let t398 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t11 * t142);
        let t401 = 1.0 / t215 / t41;
        let t402 = t27 * t401;
        let t403 = t333 * t219;
        let t414 = -0.25319e1 * t359 + 0.16879333333333333333e1 * t362 - 0.19692555555555555555e1 * t365 - 0.93011851851851851854e0 * t367 + 0.13651666666666666667e0 * t371 - 0.27303333333333333333e0 * t373 - 0.3185388888888888889e0 * t375 - 0.36514074074074074075e0 * t378;
        let t415 = t414 * t113;
        let t421 = 0.51725014705706168417e3 * t332 * t336 - 0.1025389702100779493e4 * t122 * t345 + 0.1038945353962551798e3 * t122 * t351 - 0.58482233974552040708e0 * t122 * t382 + 0.48245472966453314466e2 * t217 * t385 * t112 - 6.0 * t186 * t114 * t211 + 6.0 * t217 * t392 + 2.0 * t398 - 0.96490945932906628932e2 * t402 * t403 + 1.0 * t96 * t415 + 0.34451131037037037036e-2 * t20 * t316 * t45;
        let t432 = t253 * t129;
        let t437 = t250 * t341 * t131;
        let t441 = t131 * t244;
        let t445 = t89 * t216;
        let t449 = t174 * t95;
        let t467 = -0.21687161765563048428e-1 * t227 * t175 * t132 + 0.16265371324172286321e-1 * t227 * t90 * t246 + 0.48159446095139119799e0 * t227 * t90 * t254 - 0.51947267698127589897e2 * t122 * t250 * t244 * t432 - 0.35089340384731224426e1 * t122 * t437 + 0.35089340384731224426e1 * t122 * t232 * t129 * t441 - 0.85917146441092277512e0 * t179 * t445 * t220 + 0.71233333333333333334e-1 * t179 * t449 * t114 - 0.53425e-1 * t179 * t180 * t212 - 0.32530742648344572643e-1 * t227 * t90 * t235 - 0.56969282336565386482e-3 * t117 * t99 * t315 * t65 + 0.10685e0 * t179 * t89 * t185 * t188;
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
        let t505 = -0.32592592592592592592e-1 * t139 * t477 * t80 + 0.17333333333333333333e-2 * t271 * t484 * t148 - 0.85616400177775416862e-5 * t491 * t282 + 0.27825330057777010479e-4 * t496 * t491 - 0.5633333333333333333e-2 * t284 * t285 * t484 + 0.10592592592592592592e0 * t150 * t151 * t477;
        let tv3rho30 = 6.0 * t135 * t155 + 3.0 * t136 * t292 + t138 * t505 + 3.0 * t258 * t155 + 3.0 * t257 * t82 + 3.0 * t69 * t292 + t469 * t82;
        v3rho3[ip] += tv3rho30;
        let tv3rho2sigma0 = 0.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t521 = t273 * t262;
        let t522 = 1.0 / t521;
        let t523 = t522 * t282;
        let t535 = 0.88888888888888888888e-2 * t35 * t377 * t80 - 0.77999999999999999999e-3 * t271 * t276 * t148 * lapl[ip] + 0.51369840106665250119e-5 * t523 * t272 - 0.16695198034666206288e-4 * t496 * t522 * t272 + 0.25349999999999999999e-2 * t284 * t285 * t276 * lapl[ip] - 0.28888888888888888888e-1 * t150 * t361;
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
        let t553 = 0.26e-3 * t313 * t539 * t148 - 0.30821904063999150072e-5 * t544 * lapl[ip] + 0.10017118820799723773e-4 * t496 * t543 * lapl[ip] - 0.845e-3 * t284 * t285 * t300;
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
        let t560 = 0.18493142438399490044e-5 * t555 * t282 - 0.60102712924798342641e-5 * t496 * t555;
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
        let t603 = f64::powf(t25, -0.25e1);
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
        let t649 = 2.0 * t572 + 0.3103500882342370105e4 * t332 * t187 * t335 * t211 + 36.0 * t217 * t188 * t211 - 0.57894567559743977359e3 * t402 * t385 * t187 + 1.0 * t96 * (-0.21099166666666666667e1 * t591 + 0.202552e2 * t593 - 0.75019259259259259258e1 * t596 + 0.6564185185185185185e1 * t599 + 0.31003950617283950618e1 * t601 + 0.68258333333333333335e-1 * t606 - 0.10921333333333333333e1 * t608 + 0.12134814814814814815e1 * t610 + 0.10617962962962962963e1 * t612 + 0.13388493827160493828e1 * t615) * t113 + 0.24954977986735470917e5 * t27 / t621 * t624 / t625 + 0.57894567559743977359e3 * t332 * t624 * t219 - 0.620700176468474021e4 * t27 / t215 / t184 * t624 * t335 + 0.48245472966453314466e2 * t217 * t639 * t219 - 6.0 * t186 * t639 * t113 - 24.0 * t402 * t624 * t113;
        let t652 = t233 * t233;
        let t661 = t249 * t249;
        let t664 = t252 * t252;
        let t684 = t244 * t244;
        let t716 = 0.12304676425209353917e5 * t122 / t249 / t231 * t652 * t344 - 0.6233672123775310788e3 * t122 * t340 * t652 * t253 - 0.91080982599109921211e5 * t122 / t661 * t652 / t664 - 0.58482233974552040708e0 * t122 * t124 * (-0.28769444444444444444e1 * t591 + 0.27618666666666666667e2 * t593 - 0.10229135802469135803e2 * t596 + 0.89504938271604938273e1 * t599 + 0.31310740740740740741e1 * t601 + 0.366775e-1 * t606 - 0.58684e0 * t608 + 0.65204444444444444445e0 * t610 + 0.5705388888888888889e0 * t612 + 0.13490888888888888889e1 * t615) * t131 + 0.35089340384731224426e1 * t122 * t232 * t684 * t131 - 0.51947267698127589897e2 * t122 * t250 * t684 * t253 + 0.1403573615389248977e2 * t122 * t349 * t652 * t131 - 8.0 * t186 * t114 * t414 + 0.64327297288604419288e2 * t217 * t414 * t219 * t112 - 0.11483710345679012345e-1 * t20 * t539 * t45 - 0.34366858576436911004e1 * t103 * t216 * t211 * t219 * t112 + 0.4274e0 * t103 * t185 * t112 * t212;
        let t742 = t54 * t179;
        let t744 = t244 * t253;
        let t762 = -0.43374323531126096856e-1 * t227 * t175 * t246 - 0.1284251895870376528e1 * t227 * t175 * t254 + 0.13012297059337829057e0 * t227 * t90 * t437 + 0.38024868119570572865e2 * t227 * t90 * t345 - 0.38527556876111295841e1 * t227 * t90 * t351 + 0.21687161765563048428e-1 * t227 * t90 * t382 + 0.67471169937307261776e-1 * t227 * t316 * t132 + 0.86748647062252193713e-1 * t227 * t175 * t235 + 0.1926377843805564792e1 * t742 * t89 * t250 * t744 * t129 - 0.13012297059337829057e0 * t742 * t89 * t232 * t441 * t129 + 0.2291123905095794067e1 * t179 * t174 * t216 * t220 - 0.28493333333333333334e0 * t179 * t174 * t185 * t188;
        let t809 = 0.14246666666666666667e0 * t179 * t449 * t212 - 0.21053604230838734656e2 * t122 * t251 * t441 - 0.71233333333333333333e-1 * t179 * t180 * t415 - 0.22161481481481481481e0 * t179 * t315 * t95 * t114 - 0.4274e0 * t179 * t445 * t392 + 0.68733717152873822009e1 * t179 * t89 * t401 * t403 + 0.46785787179641632568e1 * t122 * t232 * t380 * t131 * t129 - 0.69263023597503453196e2 * t122 * t250 * t380 * t432 - 0.36845452142031360636e2 * t179 * t89 * t331 * t336 - 0.61523382126046769581e4 * t122 * t340 * t233 * t344 * t244 + 0.62336721237753107879e3 * t122 * t349 * t233 * t744 + 0.18989760778855128827e-2 * t117 * t99 * t300 * t65;
        let t822 = lapl[ip] / t36 / t274;
        let t828 = t272 / t12 / t521;
        let t832 = t489 * rho[ip];
        let t834 = t488 / t832;
        let t837 = t272 * t272;
        let t840 = 1.0 / t36 / t489 / t140;
        let t847 = t77 / t494 / t79;
        let tv4rho40 = 4.0 * t468 * t82 + 12.0 * t257 * t155 + 12.0 * t135 * t292 + 4.0 * t69 * t505 + rho[ip] * (t649 + t716 + t762 + t809) * t82 + 4.0 * t469 * t155 + 6.0 * t258 * t292 + 4.0 * t136 * t505 + t138 * (0.15209876543209876543e0 * t139 * t822 * t80 - 0.13096296296296296296e-1 * t271 * t828 * t148 + 0.13698624028444066698e-3 * t834 * t282 - 0.37100440077036013972e-6 * t837 * t840 * t495 * t139 + 0.12057643025036704541e-5 * t847 * t837 * t840 * t33 * t151 - 0.44520528092443216767e-3 * t496 * t834 + 0.42562962962962962961e-1 * t284 * t285 * t828 - 0.49432098765432098763e0 * t150 * t151 * t822);
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = 0.0;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t885 = 1.0 / t36 / t832;
        let tv4rho3lapl0 = 3.0 * t257 * t163 + 6.0 * t135 * t311 + 3.0 * t69 * t535 + t469 * t163 + 3.0 * t258 * t311 + 3.0 * t136 * t535 + t138 * (-0.32592592592592592592e-1 * t35 * t614 * t80 + 0.47377777777777777777e-2 * t271 * t483 * t148 * lapl[ip] - 0.66780792138664825154e-4 * t490 * t282 * t272 + 0.22260264046221608384e-6 * t885 * t495 * t488 * t139 - 0.72345858150220227246e-6 * t847 * t885 * t488 * t33 * t151 + 0.21703757445066068175e-3 * t496 * t490 * t272 - 0.15397777777777777777e-1 * t284 * t285 * t483 * lapl[ip] + 0.10592592592592592592e0 * t150 * t595);
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
        let tv4rho2lapl20 = 2.0 * t135 * t323 + 2.0 * t69 * t553 + t258 * t323 + 2.0 * t136 * t553 + t138 * (-0.11266666666666666667e-2 * t313 * t22 * t276 * t148 + 0.28767110459732540067e-4 * t523 * lapl[ip] - 0.13356158427732965031e-6 * t920 * t495 * t272 * t139 + 0.43407514890132136348e-6 * t847 * t920 * t272 * t33 * t151 - 0.93493108994130755216e-4 * t496 * t522 * lapl[ip] + 0.36616666666666666667e-2 * t284 * t285 * t276);
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
        let tv4rholapl30 = t69 * t560 + t136 * t560 + t138 * (-0.9246571219199745022e-5 * t544 + 0.80136950566397790188e-7 * t942 * t495 * t33 * t151 * lapl[ip] - 0.2604450893407928181e-6 * t847 * t942 * t70 * t71 + 0.3005135646239917132e-4 * t496 * t543);
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
        let tv4lapl40 = t138 * (-0.48082170339838674114e-7 * t957 * t495 * t139 + 0.15626705360447569087e-6 * t847 * t957 * t139);
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
