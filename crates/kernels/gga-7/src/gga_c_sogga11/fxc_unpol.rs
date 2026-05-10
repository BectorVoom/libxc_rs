//! GGA_C_SOGGA11 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 64 shared lines across all orders.
//! Delta: 169 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_sogga11_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_sogga11_a_0: f64,
    param_sogga11_a_1: f64,
    param_sogga11_a_2: f64,
    param_sogga11_a_3: f64,
    param_sogga11_a_4: f64,
    param_sogga11_a_5: f64,
    param_sogga11_b_0: f64,
    param_sogga11_b_1: f64,
    param_sogga11_b_2: f64,
    param_sogga11_b_3: f64,
    param_sogga11_b_4: f64,
    param_sogga11_b_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (64 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 0.379785e1 * t13 + 0.8969e0 * t10 + 0.204775e0 * t16 + 0.123235e0 * t24;
        let t29 = 1.0 + 0.16081979498692535067e2 / t26;
        let t30 = f64::ln(t29);
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608749977793437516e2 / t50;
        let t54 = f64::ln(t53);
        let t58 = -0.621814e-1 * t12 * t30 + 0.19751673498613801407e-1 * t43 * t45 * t54;
        let t60 = param_sogga11_a_1;
        let t61 = t34 * t34;
        let t62 = piecewise3(t33, t61, 1.0);
        let t63 = t39 * t62;
        let t64 = rho[ip] * rho[ip];
        let t66 = 1.0 / t7 / t64;
        let t67 = sigma[ip] * t66;
        let t68 = t63 * t67;
        let t69 = 1.0 / t3;
        let t70 = t18 * t69;
        let t71 = 1.0 / t58;
        let t72 = t5 * t71;
        let t73 = t70 * t72;
        let t75 = 0.69506584583333333332e-3 * t68 * t73;
        let t76 = 1.0 - t75;
        let t78 = 1.0 - 1.0 / t76;
        let t80 = param_sogga11_a_2;
        let t81 = t78 * t78;
        let t83 = param_sogga11_a_3;
        let t84 = t81 * t78;
        let t86 = param_sogga11_a_4;
        let t87 = t81 * t81;
        let t89 = param_sogga11_a_5;
        let t93 = param_sogga11_b_1;
        let t94 = f64::exp(t75);
        let t95 = 1.0 - t94;
        let t97 = param_sogga11_b_2;
        let t98 = t95 * t95;
        let t100 = param_sogga11_b_3;
        let t101 = t98 * t95;
        let t103 = param_sogga11_b_4;
        let t104 = t98 * t98;
        let t106 = param_sogga11_b_5;
        let t109 = t106 * t104 * t95 + t89 * t87 * t78 + t100 * t101 + t103 * t104 + t60 * t78 + t80 * t81 + t83 * t84 + t86 * t87 + t93 * t95 + t97 * t98 + param_sogga11_a_0 + param_sogga11_b_0;
        let tzk0 = t58 * t109;
        zk[ip] += tzk0;
        // --- vxc delta (74 lines) ---
        let t111 = 1.0 / t7 / rho[ip];
        let t112 = t6 * t111;
        let t116 = t26 * t26;
        let t117 = 1.0 / t116;
        let t118 = t12 * t117;
        let t120 = 1.0 / t13 * t1;
        let t121 = t3 * t6;
        let t122 = t121 * t111;
        let t123 = t120 * t122;
        let t125 = t4 * t112;
        let t127 = f64::sqrt(t10);
        let t128 = t127 * t1;
        let t129 = t128 * t122;
        let t134 = t20 * t5 / t21 / rho[ip];
        let t136 = -0.632975e0 * t123 - 0.29896666666666666667e0 * t125 - 0.1023875e0 * t129 - 0.82156666666666666667e-1 * t134;
        let t137 = 1.0 / t29;
        let t138 = t136 * t137;
        let t141 = t43 * t1;
        let t146 = t43 * t45;
        let t147 = t50 * t50;
        let t148 = 1.0 / t147;
        let t153 = -0.86308333333333333334e0 * t123 - 0.301925e0 * t125 - 0.5501625e-1 * t129 - 0.82785e-1 * t134;
        let t155 = 1.0 / t53;
        let t156 = t148 * t153 * t155;
        let t159 = 0.11073470983333333333e-2 * t4 * t112 * t30 + 1.0 * t118 * t138 - 0.18311447306006545054e-3 * t141 * t121 * t111 * t54 - 0.5848223622634646207e0 * t146 * t156;
        let t160 = rho[ip] * t159;
        let t162 = rho[ip] * t58;
        let t163 = t76 * t76;
        let t164 = 1.0 / t163;
        let t165 = t60 * t164;
        let t166 = t64 * rho[ip];
        let t168 = 1.0 / t7 / t166;
        let t169 = sigma[ip] * t168;
        let t170 = t63 * t169;
        let t173 = t58 * t58;
        let t174 = 1.0 / t173;
        let t175 = t5 * t174;
        let t176 = t175 * t159;
        let t177 = t70 * t176;
        let t180 = 0.16218203069444444444e-2 * t170 * t73 + 0.69506584583333333332e-3 * t68 * t177;
        let t182 = t80 * t78;
        let t183 = t164 * t180;
        let t186 = t83 * t81;
        let t189 = t86 * t84;
        let t192 = t89 * t87;
        let t195 = -t180;
        let t196 = t93 * t195;
        let t198 = t97 * t95;
        let t199 = t195 * t94;
        let t202 = t100 * t98;
        let t205 = t103 * t101;
        let t208 = t106 * t104;
        let t211 = t165 * t180 + 2.0 * t182 * t183 + 3.0 * t186 * t183 + 4.0 * t189 * t183 + 5.0 * t192 * t183 - t196 * t94 - 2.0 * t198 * t199 - 3.0 * t202 * t199 - 4.0 * t205 * t199 - 5.0 * t208 * t199;
        let tvrho0 = t160 * t109 + t162 * t211 + tzk0;
        vrho[ip] += tvrho0;
        let t213 = t165 * t63;
        let t214 = t66 * t18;
        let t215 = t69 * t5;
        let t216 = t215 * t71;
        let t217 = t214 * t216;
        let t221 = t164 * t39 * t62;
        let t222 = t182 * t221;
        let t225 = t186 * t221;
        let t228 = t189 * t221;
        let t231 = t192 * t221;
        let t234 = t93 * t39;
        let t235 = t62 * t66;
        let t238 = t70 * t72 * t94;
        let t241 = t63 * t66;
        let t242 = t198 * t241;
        let t245 = t202 * t241;
        let t248 = t205 * t241;
        let t251 = t208 * t241;
        let t254 = -0.69506584583333333332e-3 * t213 * t217 - 0.13901316916666666666e-2 * t222 * t217 - 0.20851975375e-2 * t225 * t217 - 0.27802633833333333333e-2 * t228 * t217 - 0.34753292291666666666e-2 * t231 * t217 - 0.69506584583333333332e-3 * t234 * t235 * t238 - 0.13901316916666666666e-2 * t242 * t238 - 0.20851975375e-2 * t245 * t238 - 0.27802633833333333333e-2 * t248 * t238 - 0.34753292291666666666e-2 * t251 * t238;
        let tvsigma0 = t162 * t254;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (169 lines) ---
        let t259 = t6 * t66;
        let t263 = t4 * t6;
        let t264 = t111 * t117;
        let t268 = t116 * t26;
        let t269 = 1.0 / t268;
        let t270 = t12 * t269;
        let t271 = t136 * t136;
        let t272 = t271 * t137;
        let t277 = 1.0 / t13 / t10 * t18;
        let t278 = t19 * t5;
        let t280 = 1.0 / t21 / t64;
        let t281 = t278 * t280;
        let t282 = t277 * t281;
        let t284 = t121 * t66;
        let t285 = t120 * t284;
        let t287 = t4 * t259;
        let t289 = 1.0/f64::sqrt(t10);
        let t290 = t289 * t18;
        let t291 = t290 * t281;
        let t293 = t128 * t284;
        let t296 = t20 * t5 * t280;
        let t298 = -0.42198333333333333333e0 * t282 + 0.84396666666666666666e0 * t285 + 0.39862222222222222223e0 * t287 + 0.68258333333333333333e-1 * t291 + 0.13651666666666666667e0 * t293 + 0.13692777777777777778e0 * t296;
        let t299 = t298 * t137;
        let t302 = t116 * t116;
        let t303 = 1.0 / t302;
        let t304 = t12 * t303;
        let t305 = t29 * t29;
        let t306 = 1.0 / t305;
        let t307 = t271 * t306;
        let t314 = t43 * t4;
        let t318 = t147 * t50;
        let t319 = 1.0 / t318;
        let t320 = t153 * t153;
        let t322 = t319 * t320 * t155;
        let t331 = -0.57538888888888888889e0 * t282 + 0.11507777777777777778e1 * t285 + 0.40256666666666666667e0 * t287 + 0.366775e-1 * t291 + 0.73355e-1 * t293 + 0.137975e0 * t296;
        let t333 = t148 * t331 * t155;
        let t336 = t147 * t147;
        let t337 = 1.0 / t336;
        let t338 = t337 * t320;
        let t339 = t53 * t53;
        let t340 = 1.0 / t339;
        let t341 = t338 * t340;
        let t344 = -0.14764627977777777777e-2 * t4 * t259 * t30 - 0.35616666666666666666e-1 * t263 * t264 * t138 - 2.0 * t270 * t272 + 1.0 * t118 * t299 + 0.16081979498692535067e2 * t304 * t307 + 0.24415263074675393405e-3 * t141 * t121 * t66 * t54 + 0.10843581300301739842e-1 * t314 * t112 * t156 + 0.11696447245269292414e1 * t146 * t322 - 0.5848223622634646207e0 * t146 * t333 - 0.17315859105681463759e2 * t146 * t341;
        let t345 = rho[ip] * t344;
        let t349 = t83 * t78;
        let t350 = t163 * t163;
        let t351 = 1.0 / t350;
        let t352 = t180 * t180;
        let t353 = t351 * t352;
        let t356 = t195 * t195;
        let t357 = t356 * t94;
        let t360 = t163 * t76;
        let t361 = 1.0 / t360;
        let t362 = t361 * t352;
        let t367 = t64 * t64;
        let t369 = 1.0 / t7 / t367;
        let t371 = t63 * sigma[ip] * t369;
        let t377 = 1.0 / t173 / t58;
        let t378 = t5 * t377;
        let t379 = t159 * t159;
        let t380 = t378 * t379;
        let t381 = t70 * t380;
        let t384 = t175 * t344;
        let t385 = t70 * t384;
        let t388 = 0.54060676898148148147e-2 * t371 * t73 + 0.32436406138888888888e-2 * t170 * t177 + 0.13901316916666666666e-2 * t68 * t381 - 0.69506584583333333332e-3 * t68 * t385;
        let t389 = t388 * t94;
        let t392 = t103 * t98;
        let t393 = t94 * t94;
        let t394 = t356 * t393;
        let t397 = t100 * t95;
        let t402 = t106 * t101;
        let t413 = -t388;
        let t414 = t164 * t413;
        let t417 = -4.0 * t182 * t362 + 2.0 * t182 * t414 - 6.0 * t186 * t362 - 8.0 * t189 * t362 - 10.0 * t192 * t362 - 3.0 * t202 * t389 - 4.0 * t205 * t357 - 4.0 * t205 * t389 - 5.0 * t208 * t357 - 5.0 * t208 * t389 + 6.0 * t349 * t353 + 12.0 * t392 * t394 + 6.0 * t397 * t394 + 20.0 * t402 * t394;
        let t418 = t93 * t356;
        let t424 = t80 * t351;
        let t427 = t97 * t356;
        let t433 = t60 * t361;
        let t438 = t86 * t81;
        let t441 = t89 * t84;
        let t450 = -t93 * t388 * t94 + t165 * t413 + 3.0 * t186 * t414 + 4.0 * t189 * t414 + 5.0 * t192 * t414 - 2.0 * t198 * t357 - 2.0 * t198 * t389 - 3.0 * t202 * t357 + 2.0 * t424 * t352 - 2.0 * t433 * t352 + 12.0 * t438 * t353 + 20.0 * t441 * t353 + 2.0 * t427 * t393 - t418 * t94;
        let t451 = t417 + t450;
        let tv2rho20 = 2.0 * t159 * t109 + t345 * t109 + 2.0 * t160 * t211 + t162 * t451 + 2.0 * t58 * t211;
        v2rho2[ip] += tv2rho20;
        let t455 = t433 * t241;
        let t456 = t72 * t180;
        let t457 = t70 * t456;
        let t460 = t165 * t241;
        let t463 = t63 * t168;
        let t464 = t198 * t463;
        let t467 = t202 * t463;
        let t470 = t205 * t463;
        let t473 = t168 * t18;
        let t474 = t473 * t216;
        let t482 = t180 * t39 * t62;
        let t483 = t424 * t482;
        let t488 = t235 * t18;
        let t489 = t234 * t488;
        let t490 = t174 * t94;
        let t491 = t490 * t159;
        let t492 = t215 * t491;
        let t496 = t71 * t195 * t94;
        let t497 = t215 * t496;
        let t500 = t97 * t195;
        let t501 = t393 * t39;
        let t502 = t501 * t62;
        let t503 = t500 * t502;
        let t506 = t208 * t463;
        let t509 = t62 * t168;
        let t516 = t361 * t39 * t62;
        let t517 = t182 * t516;
        let t518 = t214 * t69;
        let t519 = t518 * t456;
        let t522 = t518 * t176;
        let t525 = t186 * t516;
        let t528 = 0.13901316916666666666e-2 * t455 * t457 + 0.69506584583333333332e-3 * t460 * t177 + 0.32436406138888888887e-2 * t464 * t238 + 0.48654609208333333333e-2 * t467 * t238 + 0.64872812277777777777e-2 * t470 * t238 + 0.64872812277777777777e-2 * t228 * t474 + 0.81091015347222222221e-2 * t231 * t474 + 0.32436406138888888887e-2 * t222 * t474 - 0.13901316916666666666e-2 * t483 * t217 + 0.48654609208333333333e-2 * t225 * t474 + 0.69506584583333333332e-3 * t489 * t492 - 0.69506584583333333332e-3 * t489 * t497 + 0.13901316916666666666e-2 * t503 * t217 + 0.81091015347222222221e-2 * t506 * t238 + 0.16218203069444444444e-2 * t234 * t509 * t238 + 0.16218203069444444444e-2 * t213 * t474 + 0.27802633833333333332e-2 * t517 * t519 + 0.13901316916666666666e-2 * t222 * t522 + 0.4170395075e-2 * t525 * t519;
        let t529 = t70 * t5;
        let t530 = t529 * t496;
        let t533 = t529 * t491;
        let t540 = t397 * t241;
        let t541 = t71 * t393;
        let t543 = t529 * t541 * t195;
        let t550 = t189 * t516;
        let t555 = t192 * t516;
        let t561 = t351 * t39 * t62;
        let t562 = t349 * t561;
        let t565 = t438 * t561;
        let t568 = t441 * t561;
        let t573 = t392 * t241;
        let t580 = t402 * t241;
        let t583 = -0.34753292291666666666e-2 * t251 * t530 + 0.13901316916666666666e-2 * t242 * t533 - 0.13901316916666666666e-2 * t242 * t530 + 0.20851975375e-2 * t245 * t533 + 0.4170395075e-2 * t540 * t543 - 0.20851975375e-2 * t245 * t530 + 0.20851975375e-2 * t225 * t522 + 0.55605267666666666666e-2 * t550 * t519 + 0.27802633833333333333e-2 * t228 * t522 + 0.69506584583333333332e-2 * t555 * t519 + 0.34753292291666666666e-2 * t231 * t522 - 0.4170395075e-2 * t562 * t519 - 0.83407901499999999999e-2 * t565 * t519 - 0.13901316916666666666e-1 * t568 * t519 + 0.27802633833333333333e-2 * t248 * t533 + 0.83407901499999999999e-2 * t573 * t543 - 0.27802633833333333333e-2 * t248 * t530 + 0.34753292291666666666e-2 * t251 * t533 + 0.13901316916666666666e-1 * t580 * t543;
        let t584 = t528 + t583;
        let tv2rhosigma0 = t160 * t254 + t162 * t584 + t58 * t254;
        v2rhosigma[ip] += tv2rhosigma0;
        let t586 = t39 * t39;
        let t587 = t62 * t62;
        let t588 = t586 * t587;
        let t589 = t433 * t588;
        let t591 = 1.0 / t21 / t367;
        let t592 = t591 * t1;
        let t593 = 1.0 / t19;
        let t594 = t593 * t6;
        let t595 = t594 * t174;
        let t596 = t592 * t595;
        let t599 = t424 * t588;
        let t603 = t361 * t586 * t587;
        let t604 = t182 * t603;
        let t608 = t351 * t586 * t587;
        let t609 = t349 * t608;
        let t612 = t186 * t603;
        let t615 = t438 * t608;
        let t618 = t189 * t603;
        let t621 = t441 * t608;
        let t624 = t192 * t603;
        let t627 = t93 * t586;
        let t628 = t587 * t591;
        let t630 = t1 * t593;
        let t631 = t6 * t174;
        let t633 = t630 * t631 * t94;
        let t636 = t97 * t586;
        let t639 = t630 * t631 * t393;
        let t642 = t588 * t591;
        let t643 = t198 * t642;
        let t646 = t397 * t642;
        let t649 = t202 * t642;
        let t652 = t392 * t642;
        let t655 = t205 * t642;
        let t658 = t402 * t642;
        let t661 = t208 * t642;
        let t664 = -0.28986991802640426041e-5 * t589 * t596 + 0.2898699180264042604e-5 * t599 * t596 - 0.57973983605280852081e-5 * t604 * t596 + 0.86960975407921278125e-5 * t609 * t596 - 0.86960975407921278125e-5 * t612 * t596 + 0.17392195081584255625e-4 * t615 * t596 - 0.11594796721056170417e-4 * t618 * t596 + 0.28986991802640426041e-4 * t621 * t596 - 0.14493495901320213021e-4 * t624 * t596 - 0.14493495901320213021e-5 * t627 * t628 * t633 + 0.2898699180264042604e-5 * t636 * t628 * t639 - 0.2898699180264042604e-5 * t643 * t633 + 0.86960975407921278125e-5 * t646 * t639 - 0.43480487703960639062e-5 * t649 * t633 + 0.17392195081584255625e-4 * t652 * t639 - 0.57973983605280852083e-5 * t655 * t633 + 0.28986991802640426041e-4 * t658 * t639 - 0.72467479506601065103e-5 * t661 * t633;
        let tv2sigma20 = t162 * t664;
        v2sigma2[ip] += tv2sigma20;
    }
}
