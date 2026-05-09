//! MGGA_K_LK lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 68 shared lines across all orders.
//! Delta: 164 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_lk_lxc_unpol(
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
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (68 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t40 = t25 * t25;
        let t42 = 1.0 / t27 / t26;
        let t43 = t40 * t42;
        let t44 = lapl[ip] * lapl[ip];
        let t45 = t44 * t31;
        let t46 = t34 * rho[ip];
        let t48 = 1.0 / t22 / t46;
        let t51 = t43 * t45 * t48 / 2916.0;
        let t52 = t43 * sigma[ip];
        let t53 = t34 * t34;
        let t55 = 1.0 / t22 / t53;
        let t56 = t31 * t55;
        let t57 = t56 * lapl[ip];
        let t59 = t52 * t57 / 2592.0;
        let t60 = sigma[ip] * sigma[ip];
        let t61 = t60 * t31;
        let t62 = t53 * rho[ip];
        let t64 = 1.0 / t22 / t62;
        let t67 = t43 * t61 * t64 / 8748.0;
        let t68 = t43 * t60;
        let t69 = t31 * t64;
        let t70 = 1.0 / param_kappa;
        let t71 = t69 * t70;
        let t76 = 1.0 + (5.0 / 648.0 * t30 * t33 * t36 + t51 - t59 + t67 + 25.0 / 209952.0 * t68 * t71) * t70;
        let t78 = t30 * sigma[ip];
        let t79 = t32 * t36;
        let t80 = t51 - t59 + t67;
        let t81 = t80 * t70;
        let t85 = t26 * t26;
        let t86 = 1.0 / t85;
        let t87 = t60 * sigma[ip];
        let t88 = t86 * t87;
        let t89 = t53 * t53;
        let t90 = 1.0 / t89;
        let t91 = param_kappa * param_kappa;
        let t92 = 1.0 / t91;
        let t93 = t90 * t92;
        let t98 = 1.0 + (5.0 / 324.0 * t78 * t79 * t81 + 125.0 / 0.11337408e8 * t88 * t93) * t70;
        let t102 = 1.0 + param_kappa * (2.0 - 1.0 / t76 - 1.0 / t98);
        let t106 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t21 * t23 * t102);
        let tzk0 = 2.0 * t106;
        zk[ip] += tzk0;
        // --- vxc delta (51 lines) ---
        let t107 = 1.0 / t22;
        let t112 = t8 * t21;
        let t113 = t23 * param_kappa;
        let t114 = t76 * t76;
        let t115 = 1.0 / t114;
        let t117 = 1.0 / t23 / t46;
        let t123 = 5.0 / 4374.0 * t43 * t45 * t55;
        let t124 = t69 * lapl[ip];
        let t126 = 13.0 / 7776.0 * t52 * t124;
        let t127 = t53 * t34;
        let t129 = 1.0 / t22 / t127;
        let t132 = 4.0 / 6561.0 * t43 * t61 * t129;
        let t133 = t31 * t129;
        let t134 = t133 * t70;
        let t137 = -5.0 / 243.0 * t30 * t33 * t117 - t123 + t126 - t132 - 25.0 / 39366.0 * t68 * t134;
        let t140 = t98 * t98;
        let t141 = 1.0 / t140;
        let t142 = t32 * t117;
        let t146 = -t123 + t126 - t132;
        let t147 = t146 * t70;
        let t151 = t89 * rho[ip];
        let t152 = 1.0 / t151;
        let t153 = t152 * t92;
        let t156 = -10.0 / 243.0 * t78 * t142 * t81 + 5.0 / 324.0 * t78 * t79 * t147 - 125.0 / 1417176.0 * t88 * t153;
        let t159 = t115 * t137 * t70 + t141 * t156 * t70;
        let t164 = piecewise3(t3, 0.0, t8 * t21 * t107 * t102 / 10.0 + 3.0 / 20.0 * t112 * t113 * t159);
        let tvrho0 = 2.0 * rho[ip] * t164 + 2.0 * t106;
        vrho[ip] += tvrho0;
        let t169 = t43 * t57;
        let t170 = t169 / 2592.0;
        let t171 = sigma[ip] * t31;
        let t173 = t43 * t171 * t64;
        let t174 = t173 / 4374.0;
        let t177 = 5.0 / 648.0 * t30 * t79 - t170 + t174 + 25.0 / 104976.0 * t52 * t71;
        let t180 = t30 * t32;
        let t185 = -t170 + t174;
        let t186 = t185 * t70;
        let t190 = t86 * t60;
        let t193 = 5.0 / 324.0 * t180 * t36 * t80 * t70 + 5.0 / 324.0 * t78 * t79 * t186 + 125.0 / 3779136.0 * t190 * t93;
        let t196 = t115 * t177 * t70 + t141 * t193 * t70;
        let t200 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t196);
        let tvsigma0 = 2.0 * rho[ip] * t200;
        vsigma[ip] += tvsigma0;
        let t209 = t43 * lapl[ip] * t31 * t48 / 1458.0 - t43 * t171 * t55 / 2592.0;
        let t212 = t141 * t25;
        let t213 = t29 * sigma[ip];
        let t214 = t212 * t213;
        let t215 = t209 * t92;
        let t216 = t79 * t215;
        let t219 = t115 * t209 * t70 + 5.0 / 324.0 * t214 * t216;
        let t223 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t219);
        let tvlapl0 = 2.0 * rho[ip] * t223;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        // --- fxc delta (92 lines) ---
        let t227 = 1.0 / t22 / rho[ip];
        let t232 = t107 * param_kappa;
        let t237 = 1.0 / t114 / t76;
        let t238 = t137 * t137;
        let t243 = 1.0 / t23 / t53;
        let t249 = 65.0 / 13122.0 * t43 * t45 * t64;
        let t250 = t133 * lapl[ip];
        let t252 = 13.0 / 1458.0 * t52 * t250;
        let t253 = t53 * t46;
        let t255 = 1.0 / t22 / t253;
        let t258 = 76.0 / 19683.0 * t43 * t61 * t255;
        let t259 = t31 * t255;
        let t260 = t259 * t70;
        let t263 = 55.0 / 729.0 * t30 * t33 * t243 + t249 - t252 + t258 + 475.0 / 118098.0 * t68 * t260;
        let t267 = 1.0 / t140 / t98;
        let t268 = t156 * t156;
        let t272 = t32 * t243;
        let t279 = t249 - t252 + t258;
        let t280 = t279 * t70;
        let t284 = t89 * t34;
        let t285 = 1.0 / t284;
        let t286 = t285 * t92;
        let t289 = 110.0 / 729.0 * t78 * t272 * t81 - 20.0 / 243.0 * t78 * t142 * t147 + 5.0 / 324.0 * t78 * t79 * t280 + 125.0 / 157464.0 * t88 * t286;
        let t292 = t115 * t263 * t70 + t141 * t289 * t70 - 2.0 * t237 * t238 * t92 - 2.0 * t267 * t268 * t92;
        let t297 = piecewise3(t3, 0.0, -t8 * t21 * t227 * t102 / 30.0 + t112 * t232 * t159 / 5.0 + 3.0 / 20.0 * t112 * t113 * t292);
        let tv2rho20 = 2.0 * rho[ip] * t297 + 4.0 * t164;
        v2rho2[ip] += tv2rho20;
        let t303 = t237 * t177;
        let t304 = t92 * t137;
        let t309 = t43 * t124;
        let t310 = 13.0 / 7776.0 * t309;
        let t312 = t43 * t171 * t129;
        let t313 = 8.0 / 6561.0 * t312;
        let t316 = -5.0 / 243.0 * t30 * t142 + t310 - t313 - 25.0 / 19683.0 * t52 * t134;
        let t319 = t267 * t193;
        let t320 = t92 * t156;
        let t334 = t310 - t313;
        let t335 = t334 * t70;
        let t341 = -10.0 / 243.0 * t180 * t117 * t80 * t70 + 5.0 / 324.0 * t180 * t36 * t146 * t70 - 10.0 / 243.0 * t78 * t142 * t186 + 5.0 / 324.0 * t78 * t79 * t335 - 125.0 / 472392.0 * t190 * t153;
        let t344 = t115 * t316 * t70 + t141 * t341 * t70 - 2.0 * t303 * t304 - 2.0 * t319 * t320;
        let t349 = piecewise3(t3, 0.0, t112 * t232 * t196 / 10.0 + 3.0 / 20.0 * t112 * t113 * t344);
        let tv2rhosigma0 = 2.0 * rho[ip] * t349 + 2.0 * t200;
        v2rhosigma[ip] += tv2rhosigma0;
        let t355 = t237 * t209;
        let t360 = -5.0 / 2187.0 * t169 + 13.0 / 7776.0 * t173;
        let t363 = t267 * t25;
        let t364 = t363 * t213;
        let t365 = t91 * param_kappa;
        let t366 = 1.0 / t365;
        let t367 = t209 * t366;
        let t368 = t367 * t156;
        let t372 = t142 * t215;
        let t375 = t360 * t92;
        let t376 = t79 * t375;
        let t379 = -2.0 * t355 * t304 + t115 * t360 * t70 - 5.0 / 162.0 * t364 * t79 * t368 - 10.0 / 243.0 * t214 * t372 + 5.0 / 324.0 * t214 * t376;
        let t384 = piecewise3(t3, 0.0, t112 * t232 * t219 / 10.0 + 3.0 / 20.0 * t112 * t113 * t379);
        let tv2rholapl0 = 2.0 * rho[ip] * t384 + 2.0 * t223;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let t387 = t177 * t177;
        let t391 = t43 * t69;
        let t395 = t391 / 4374.0 + 25.0 / 104976.0 * t43 * t71;
        let t398 = t193 * t193;
        let t406 = t86 * sigma[ip];
        let t412 = 5.0 / 162.0 * t180 * t36 * t185 * t70 + 5.0 / 118098.0 * t406 * t90 * t70 + 125.0 / 1889568.0 * t406 * t93;
        let t415 = t115 * t395 * t70 + t141 * t412 * t70 - 2.0 * t237 * t387 * t92 - 2.0 * t267 * t398 * t92;
        let t419 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t415);
        let tv2sigma20 = 2.0 * rho[ip] * t419;
        v2sigma2[ip] += tv2sigma20;
        let t421 = t92 * t177;
        let t425 = t115 * t40 * t42;
        let t427 = t425 * t56 * t70;
        let t429 = t367 * t193;
        let t433 = t212 * t29;
        let t436 = t141 * t86;
        let t437 = 1.0 / t253;
        let t440 = t436 * sigma[ip] * t437 * t92;
        let t442 = -2.0 * t355 * t421 - t427 / 2592.0 - 5.0 / 162.0 * t364 * t79 * t429 + 5.0 / 324.0 * t433 * t216 - 5.0 / 69984.0 * t440;
        let t446 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t442);
        let tv2sigmalapl0 = 2.0 * rho[ip] * t446;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t448 = t209 * t209;
        let t452 = t31 * t48;
        let t456 = t267 * t40;
        let t457 = t42 * t60;
        let t458 = t456 * t457;
        let t459 = t91 * t91;
        let t460 = 1.0 / t459;
        let t461 = t448 * t460;
        let t462 = t69 * t461;
        let t465 = 1.0 / t127;
        let t470 = -2.0 * t237 * t448 * t92 + t425 * t452 * t70 / 1458.0 - 25.0 / 26244.0 * t458 * t462 + 5.0 / 39366.0 * t436 * sigma[ip] * t465 * t92;
        let t474 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t470);
        let tv2lapl20 = 2.0 * rho[ip] * t474;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        // --- kxc delta (180 lines) ---
        let t478 = 1.0 / t22 / t34;
        let t483 = t227 * param_kappa;
        let t490 = t114 * t114;
        let t491 = 1.0 / t490;
        let t492 = t238 * t137;
        let t496 = t237 * t137;
        let t497 = t92 * t263;
        let t501 = 1.0 / t23 / t62;
        let t507 = 520.0 / 19683.0 * t43 * t45 * t129;
        let t508 = t259 * lapl[ip];
        let t510 = 247.0 / 4374.0 * t52 * t508;
        let t512 = 1.0 / t22 / t89;
        let t515 = 1672.0 / 59049.0 * t43 * t61 * t512;
        let t516 = t31 * t512;
        let t517 = t516 * t70;
        let t520 = -770.0 / 2187.0 * t30 * t33 * t501 - t507 + t510 - t515 - 5225.0 / 177147.0 * t68 * t517;
        let t523 = t140 * t140;
        let t524 = 1.0 / t523;
        let t525 = t268 * t156;
        let t529 = t267 * t156;
        let t530 = t92 * t289;
        let t533 = t32 * t501;
        let t543 = -t507 + t510 - t515;
        let t544 = t543 * t70;
        let t548 = t89 * t46;
        let t550 = 1.0 / t548 * t92;
        let t553 = -1540.0 / 2187.0 * t78 * t533 * t81 + 110.0 / 243.0 * t78 * t272 * t147 - 10.0 / 81.0 * t78 * t142 * t280 + 5.0 / 324.0 * t78 * t79 * t544 - 625.0 / 78732.0 * t88 * t550;
        let t556 = t115 * t520 * t70 + t141 * t553 * t70 + 6.0 * t491 * t492 * t366 + 6.0 * t524 * t525 * t366 - 6.0 * t496 * t497 - 6.0 * t529 * t530;
        let t561 = piecewise3(t3, 0.0, 2.0 / 45.0 * t8 * t21 * t478 * t102 - t112 * t483 * t159 / 10.0 + 3.0 / 10.0 * t112 * t232 * t292 + 3.0 / 20.0 * t112 * t113 * t556);
        let tv3rho30 = 2.0 * rho[ip] * t561 + 6.0 * t297;
        v3rho3[ip] += tv3rho30;
        let t571 = t491 * t177;
        let t572 = t366 * t238;
        let t575 = t237 * t316;
        let t582 = t43 * t250;
        let t583 = 13.0 / 1458.0 * t582;
        let t585 = t43 * t171 * t255;
        let t586 = 152.0 / 19683.0 * t585;
        let t589 = 55.0 / 729.0 * t30 * t272 - t583 + t586 + 475.0 / 59049.0 * t52 * t260;
        let t592 = t524 * t193;
        let t593 = t366 * t268;
        let t596 = t267 * t341;
        let t619 = -t583 + t586;
        let t620 = t619 * t70;
        let t626 = 110.0 / 729.0 * t180 * t243 * t80 * t70 - 20.0 / 243.0 * t180 * t117 * t146 * t70 + 5.0 / 324.0 * t180 * t36 * t279 * t70 + 110.0 / 729.0 * t78 * t272 * t186 - 20.0 / 243.0 * t78 * t142 * t335 + 5.0 / 324.0 * t78 * t79 * t620 + 125.0 / 52488.0 * t190 * t286;
        let t629 = t115 * t589 * t70 + t141 * t626 * t70 - 2.0 * t303 * t497 - 4.0 * t575 * t304 - 2.0 * t319 * t530 - 4.0 * t596 * t320 + 6.0 * t571 * t572 + 6.0 * t592 * t593;
        let t634 = piecewise3(t3, 0.0, -t112 * t483 * t196 / 30.0 + t112 * t232 * t344 / 5.0 + 3.0 / 20.0 * t112 * t113 * t629);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t634 + 4.0 * t349;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t644 = t491 * t209;
        let t647 = t237 * t360;
        let t654 = 65.0 / 6561.0 * t309 - 13.0 / 1458.0 * t312;
        let t657 = t524 * t25;
        let t658 = t657 * t213;
        let t659 = t209 * t460;
        let t660 = t659 * t268;
        let t667 = t360 * t366;
        let t668 = t667 * t156;
        let t672 = t367 * t289;
        let t676 = t272 * t215;
        let t679 = t142 * t375;
        let t682 = t654 * t92;
        let t683 = t79 * t682;
        let t686 = 6.0 * t644 * t572 - 4.0 * t647 * t304 - 2.0 * t355 * t497 + t115 * t654 * t70 + 5.0 / 54.0 * t658 * t79 * t660 + 40.0 / 243.0 * t364 * t142 * t368 - 5.0 / 81.0 * t364 * t79 * t668 - 5.0 / 162.0 * t364 * t79 * t672 + 110.0 / 729.0 * t214 * t676 - 20.0 / 243.0 * t214 * t679 + 5.0 / 324.0 * t214 * t683;
        let t691 = piecewise3(t3, 0.0, -t112 * t483 * t219 / 30.0 + t112 * t232 * t379 / 5.0 + 3.0 / 20.0 * t112 * t113 * t686);
        let tv3rho2lapl0 = 2.0 * rho[ip] * t691 + 4.0 * t384;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let tv3rho2tau0 = 0.0;
        v3rho2tau[ip] += tv3rho2tau0;
        let t697 = t491 * t387;
        let t698 = t366 * t137;
        let t701 = t92 * t316;
        let t704 = t237 * t395;
        let t711 = -8.0 / 6561.0 * t43 * t133 - 25.0 / 19683.0 * t43 * t134;
        let t714 = t524 * t398;
        let t715 = t366 * t156;
        let t721 = t267 * t412;
        let t737 = -20.0 / 243.0 * t180 * t117 * t185 * t70 + 5.0 / 162.0 * t180 * t36 * t334 * t70 - 20.0 / 59049.0 * t406 * t152 * t70 - 125.0 / 236196.0 * t406 * t153;
        let t740 = t115 * t711 * t70 + t141 * t737 * t70 - 4.0 * t319 * t92 * t341 - 4.0 * t303 * t701 - 2.0 * t704 * t304 - 2.0 * t721 * t320 + 6.0 * t697 * t698 + 6.0 * t714 * t715;
        let t745 = piecewise3(t3, 0.0, t112 * t232 * t415 / 10.0 + 3.0 / 20.0 * t112 * t113 * t740);
        let tv3rhosigma20 = 2.0 * rho[ip] * t745 + 2.0 * t419;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t751 = t366 * t177;
        let t752 = t751 * t137;
        let t760 = t237 * t40 * t42;
        let t762 = t760 * t56 * t304;
        let t764 = t425 * t71;
        let t766 = t213 * t32;
        let t767 = t657 * t766;
        let t768 = t36 * t209;
        let t769 = t460 * t193;
        let t770 = t769 * t156;
        let t771 = t768 * t770;
        let t777 = t667 * t193;
        let t781 = t367 * t341;
        let t785 = t29 * t32;
        let t786 = t363 * t785;
        let t794 = t267 * t86;
        let t795 = t794 * sigma[ip];
        let t796 = t437 * t366;
        let t797 = t796 * t156;
        let t798 = t795 * t797;
        let t802 = t436 * sigma[ip] * t90 * t92;
        let t804 = 6.0 * t644 * t752 - 2.0 * t647 * t421 - 2.0 * t355 * t701 + t762 / 1296.0 + 13.0 / 7776.0 * t764 + 5.0 / 54.0 * t767 * t771 + 20.0 / 243.0 * t364 * t142 * t429 - 5.0 / 162.0 * t364 * t79 * t777 - 5.0 / 162.0 * t364 * t79 * t781 - 5.0 / 162.0 * t786 * t768 * t715 - 10.0 / 243.0 * t433 * t372 + 5.0 / 324.0 * t433 * t376 + 5.0 / 34992.0 * t798 + 35.0 / 69984.0 * t802;
        let t809 = piecewise3(t3, 0.0, t112 * t232 * t442 / 10.0 + 3.0 / 20.0 * t112 * t113 * t804);
        let tv3rhosigmalapl0 = 2.0 * rho[ip] * t809 + 2.0 * t446;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t815 = t491 * t448;
        let t824 = t524 * t40;
        let t825 = t824 * t457;
        let t827 = 1.0 / t459 / param_kappa;
        let t828 = t448 * t827;
        let t829 = t828 * t156;
        let t830 = t69 * t829;
        let t833 = t133 * t461;
        let t836 = t659 * t360;
        let t837 = t69 * t836;
        let t840 = t465 * t366;
        let t841 = t840 * t156;
        let t845 = 6.0 * t815 * t698 - 4.0 * t355 * t375 - t760 * t452 * t304 / 729.0 - 5.0 / 2187.0 * t427 + 25.0 / 8748.0 * t825 * t830 + 100.0 / 19683.0 * t458 * t833 - 25.0 / 13122.0 * t458 * t837 - 5.0 / 19683.0 * t795 * t841 - 5.0 / 6561.0 * t440;
        let t850 = piecewise3(t3, 0.0, t112 * t232 * t470 / 10.0 + 3.0 / 20.0 * t112 * t113 * t845);
        let tv3rholapl20 = 2.0 * rho[ip] * t850 + 2.0 * t474;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
        v3rhotau2[ip] += tv3rhotau20;
        let t853 = t387 * t177;
        let t857 = t92 * t395;
        let t860 = t398 * t193;
        let t864 = t92 * t412;
        let t867 = t86 * t90;
        let t872 = 5.0 / 39366.0 * t867 * t70 + 125.0 / 1889568.0 * t867 * t92;
        let t875 = t141 * t872 * t70 + 6.0 * t491 * t853 * t366 + 6.0 * t524 * t860 * t366 - 6.0 * t303 * t857 - 6.0 * t319 * t864;
        let t879 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t875);
        let tv3sigma30 = 2.0 * rho[ip] * t879;
        v3sigma3[ip] += tv3sigma30;
        let t881 = t366 * t387;
        let t885 = t760 * t56 * t421;
        let t889 = t659 * t398;
        let t893 = t366 * t193;
        let t897 = t796 * t193;
        let t898 = t795 * t897;
        let t900 = t367 * t412;
        let t905 = t436 * t437 * t92;
        let t907 = 6.0 * t644 * t881 + t885 / 648.0 - 2.0 * t355 * t857 + 5.0 / 54.0 * t658 * t79 * t889 - 5.0 / 81.0 * t786 * t768 * t893 + 5.0 / 17496.0 * t898 - 5.0 / 162.0 * t364 * t79 * t900 - 5.0 / 34992.0 * t905;
        let t911 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t907);
        let tv3sigma2lapl0 = 2.0 * rho[ip] * t911;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let tv3sigma2tau0 = 0.0;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let t915 = t355 * t92;
        let t916 = t43 * t56;
        let t917 = t915 * t916;
        let t922 = t828 * t193;
        let t923 = t69 * t922;
        let t926 = t42 * sigma[ip];
        let t927 = t456 * t926;
        let t931 = 1.0 / t28 / t85;
        let t932 = t931 * t60;
        let t933 = t363 * t932;
        let t935 = 1.0 / t23 / t151;
        let t936 = t32 * t935;
        let t937 = t936 * t659;
        let t938 = t933 * t937;
        let t940 = t840 * t193;
        let t946 = 6.0 * t815 * t751 + t917 / 648.0 - t760 * t452 * t421 / 729.0 + 25.0 / 8748.0 * t825 * t923 - 25.0 / 13122.0 * t927 * t462 + 25.0 / 5668704.0 * t938 - 5.0 / 19683.0 * t795 * t940 + 5.0 / 39366.0 * t436 * t465 * t92;
        let t950 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t946);
        let tv3sigmalapl20 = 2.0 * rho[ip] * t950;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let tv3sigmatau20 = 0.0;
        v3sigmatau2[ip] += tv3sigmatau20;
        let t952 = t448 * t209;
        let t956 = t43 * t452;
        let t959 = t524 * t86;
        let t960 = t959 * t87;
        let t961 = t90 * t952;
        let t963 = 1.0 / t459 / t91;
        let t964 = t961 * t963;
        let t968 = 1.0 / t23 / t89;
        let t969 = t32 * t968;
        let t970 = t969 * t659;
        let t973 = t60 * t968;
        let t974 = t794 * t973;
        let t975 = t460 * t25;
        let t977 = t975 * t785 * t209;
        let t980 = 6.0 * t491 * t952 * t366 - t915 * t956 / 243.0 + 125.0 / 236196.0 * t960 * t964 - 25.0 / 3188646.0 * t933 * t970 - 25.0 / 6377292.0 * t974 * t977;
        let t984 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t980);
        let tv3lapl30 = 2.0 * rho[ip] * t984;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let tv3tau30 = 0.0;
        v3tau3[ip] += tv3tau30;
        // --- lxc delta (this level) (164 lines) ---
        let t991 = t478 * param_kappa;
        let t1002 = 1.0 / t490 / t76;
        let t1003 = t238 * t238;
        let t1008 = t366 * t263;
        let t1011 = t263 * t263;
        let t1015 = t92 * t520;
        let t1019 = 1.0 / t23 / t127;
        let t1025 = 9880.0 / 59049.0 * t43 * t45 * t255;
        let t1028 = 2717.0 / 6561.0 * t52 * t516 * lapl[ip];
        let t1030 = 1.0 / t22 / t151;
        let t1033 = 41800.0 / 177147.0 * t43 * t61 * t1030;
        let t1042 = 1.0 / t523 / t98;
        let t1043 = t268 * t268;
        let t1048 = t366 * t289;
        let t1051 = t289 * t289;
        let t1055 = t92 * t553;
        let t1058 = t32 * t1019;
        let t1076 = t89 * t53;
        let t1077 = 1.0 / t1076;
        let t1089 = piecewise3(t3, 0.0, -14.0 / 135.0 * t8 * t21 * t48 * t102 + 8.0 / 45.0 * t112 * t991 * t159 - t112 * t483 * t292 / 5.0 + 2.0 / 5.0 * t112 * t232 * t556 + 3.0 / 20.0 * t112 * t113 * (-24.0 * t1002 * t1003 * t460 + 36.0 * t491 * t238 * t1008 - 6.0 * t237 * t1011 * t92 - 8.0 * t496 * t1015 + t115 * (13090.0 / 6561.0 * t30 * t33 * t1019 + t1025 - t1028 + t1033 + 130625.0 / 531441.0 * t68 * t31 * t1030 * t70) * t70 - 24.0 * t1042 * t1043 * t460 + 36.0 * t524 * t268 * t1048 - 6.0 * t267 * t1051 * t92 - 8.0 * t529 * t1055 + t141 * (26180.0 / 6561.0 * t78 * t1058 * t81 - 6160.0 / 2187.0 * t78 * t533 * t147 + 220.0 / 243.0 * t78 * t272 * t280 - 40.0 / 243.0 * t78 * t142 * t544 + 5.0 / 324.0 * t78 * t79 * (t1025 - t1028 + t1033) * t70 + 6875.0 / 78732.0 * t88 * t1077 * t92) * t70));
        let tv4rho40 = 2.0 * rho[ip] * t1089 + 8.0 * t561;
        v4rho4[ip] += tv4rho40;
        let t1103 = t460 * t492;
        let t1109 = t698 * t263;
        let t1122 = 247.0 / 4374.0 * t43 * t508;
        let t1125 = 3344.0 / 59049.0 * t43 * t171 * t512;
        let t1183 = -24.0 * t1002 * t177 * t1103 + 18.0 * t491 * t316 * t572 + 18.0 * t571 * t1109 - 6.0 * t237 * t589 * t304 - 6.0 * t575 * t497 - 2.0 * t303 * t1015 + t115 * (-770.0 / 2187.0 * t30 * t533 + t1122 - t1125 - 10450.0 / 177147.0 * t52 * t517) * t70 - 24.0 * t1042 * t193 * t460 * t525 + 18.0 * t524 * t341 * t593 + 18.0 * t592 * t715 * t289 - 6.0 * t267 * t626 * t320 - 6.0 * t596 * t530 - 2.0 * t319 * t1055 + t141 * (-1540.0 / 2187.0 * t180 * t501 * t80 * t70 + 110.0 / 243.0 * t180 * t243 * t146 * t70 - 10.0 / 81.0 * t180 * t117 * t279 * t70 + 5.0 / 324.0 * t180 * t36 * t543 * t70 - 1540.0 / 2187.0 * t78 * t533 * t186 + 110.0 / 243.0 * t78 * t272 * t335 - 10.0 / 81.0 * t78 * t142 * t620 + 5.0 / 324.0 * t78 * t79 * (t1122 - t1125) * t70 - 625.0 / 26244.0 * t190 * t550) * t70;
        let t1188 = piecewise3(t3, 0.0, 2.0 / 45.0 * t112 * t991 * t196 - t112 * t483 * t344 / 10.0 + 3.0 / 10.0 * t112 * t232 * t629 + 3.0 / 20.0 * t112 * t113 * t1183);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t1188 + 6.0 * t634;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t1205 = t1002 * t209;
        let t1211 = t460 * t156;
        let t1221 = -1040.0 / 19683.0 * t582 + 247.0 / 4374.0 * t585;
        let t1238 = t360 * t460;
        let t1249 = t654 * t366;
        let t1262 = t491 * t360;
        let t1265 = t237 * t654;
        let t1270 = t1042 * t25;
        let t1271 = t1270 * t213;
        let t1272 = t209 * t827;
        let t1277 = -1540.0 / 2187.0 * t214 * t533 * t215 + 5.0 / 18.0 * t658 * t79 * t1238 * t268 + 40.0 / 81.0 * t364 * t142 * t668 + 20.0 / 81.0 * t364 * t142 * t672 - 5.0 / 54.0 * t364 * t79 * t1249 * t156 - 5.0 / 54.0 * t364 * t79 * t667 * t289 - 5.0 / 162.0 * t364 * t79 * t367 * t553 + 18.0 * t1262 * t572 - 6.0 * t1265 * t304 - 6.0 * t647 * t497 - 10.0 / 27.0 * t1271 * t79 * t1272 * t525;
        let t1283 = piecewise3(t3, 0.0, 2.0 / 45.0 * t112 * t991 * t219 - t112 * t483 * t379 / 10.0 + 3.0 / 10.0 * t112 * t232 * t686 + 3.0 / 20.0 * t112 * t113 * (-2.0 * t355 * t1015 + 18.0 * t644 * t1109 - 24.0 * t1205 * t1103 - 20.0 / 27.0 * t658 * t142 * t660 + 5.0 / 18.0 * t767 * t768 * t1211 * t289 - 220.0 / 243.0 * t364 * t272 * t368 + t115 * t1221 * t70 + 5.0 / 324.0 * t214 * t79 * t1221 * t92 + 110.0 / 243.0 * t214 * t272 * t375 - 10.0 / 81.0 * t214 * t142 * t682 + t1277));
        let tv4rho3lapl0 = 2.0 * rho[ip] * t1283 + 6.0 * t691;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let tv4rho3tau0 = 0.0;
        v4rho3tau[ip] += tv4rho3tau0;
        let t1294 = t460 * t238;
        let t1297 = t698 * t316;
        let t1302 = t316 * t316;
        let t1306 = t92 * t589;
        let t1325 = t460 * t268;
        let t1333 = t341 * t341;
        let t1368 = -24.0 * t1002 * t387 * t1294 + 24.0 * t571 * t1297 + 6.0 * t697 * t1008 - 4.0 * t237 * t1302 * t92 - 4.0 * t303 * t1306 + 6.0 * t491 * t395 * t572 - 4.0 * t237 * t711 * t304 - 2.0 * t704 * t497 + t115 * (152.0 / 19683.0 * t43 * t259 + 475.0 / 59049.0 * t43 * t260) * t70 - 24.0 * t1042 * t398 * t1325 + 24.0 * t592 * t715 * t341 + 6.0 * t714 * t1048 - 4.0 * t267 * t1333 * t92 - 4.0 * t319 * t92 * t626 + 6.0 * t524 * t412 * t593 - 4.0 * t267 * t737 * t320 - 2.0 * t721 * t530 + t141 * (220.0 / 729.0 * t180 * t243 * t185 * t70 - 40.0 / 243.0 * t180 * t117 * t334 * t70 + 5.0 / 162.0 * t180 * t36 * t619 * t70 + 20.0 / 6561.0 * t406 * t285 * t70 + 125.0 / 26244.0 * t406 * t286) * t70;
        let t1373 = piecewise3(t3, 0.0, -t112 * t483 * t415 / 30.0 + t112 * t232 * t740 / 5.0 + 3.0 / 20.0 * t112 * t113 * t1368);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t1373 + 4.0 * t745;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t1389 = t491 * t40;
        let t1390 = t1389 * t42;
        let t1406 = t1270 * t766;
        let t1407 = t827 * t193;
        let t1412 = t117 * t209;
        let t1416 = t36 * t360;
        let t1435 = -2.0 * t1265 * t421 - 4.0 * t647 * t701 - 2.0 * t355 * t1306 - t1390 * t56 * t572 / 432.0 + t760 * t56 * t497 / 1296.0 - 20.0 / 243.0 * t433 * t679 + 5.0 / 324.0 * t433 * t683 + 110.0 / 729.0 * t433 * t676 - 13.0 / 1944.0 * t760 * t69 * t304 - 10.0 / 27.0 * t1406 * t768 * t1407 * t268 - 40.0 / 81.0 * t767 * t1412 * t770 + 5.0 / 27.0 * t767 * t1416 * t770 + 5.0 / 27.0 * t767 * t768 * t460 * t341 * t156 + 5.0 / 54.0 * t767 * t768 * t769 * t289 - 220.0 / 729.0 * t364 * t272 * t429 + 40.0 / 243.0 * t364 * t142 * t777;
        let t1451 = t90 * t366;
        let t1468 = t460 * t177;
        let t1475 = t657 * t785;
        let t1479 = t959 * sigma[ip];
        let t1480 = t437 * t460;
        let t1493 = 40.0 / 243.0 * t364 * t142 * t781 - 5.0 / 162.0 * t364 * t79 * t1249 * t193 - 5.0 / 81.0 * t364 * t79 * t667 * t341 - 5.0 / 162.0 * t364 * t79 * t367 * t626 - 35.0 / 17496.0 * t795 * t1451 * t156 - 13.0 / 1458.0 * t425 * t134 - 35.0 / 8748.0 * t436 * sigma[ip] * t152 * t92 + 12.0 * t1262 * t752 + 12.0 * t644 * t1297 + 6.0 * t644 * t751 * t263 - 24.0 * t1205 * t1468 * t238 + 40.0 / 243.0 * t786 * t1412 * t715 + 5.0 / 54.0 * t1475 * t768 * t1325 - 5.0 / 11664.0 * t1479 * t1480 * t268 + 5.0 / 34992.0 * t795 * t796 * t289 - 5.0 / 81.0 * t786 * t1416 * t715 - 5.0 / 162.0 * t786 * t768 * t1048;
        let t1499 = piecewise3(t3, 0.0, -t112 * t483 * t442 / 30.0 + t112 * t232 * t804 / 5.0 + 3.0 / 20.0 * t112 * t113 * (t1435 + t1493));
        let tv4rho2sigmalapl0 = 2.0 * rho[ip] * t1499 + 4.0 * t809;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let tv4rho2sigmatau0 = 0.0;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let t1509 = t465 * t460;
        let t1515 = t360 * t360;
        let t1521 = t1002 * t448;
        let t1533 = t1042 * t40;
        let t1534 = t1533 * t457;
        let t1535 = t448 * t963;
        let t1544 = t457 * t31;
        let t1545 = t824 * t1544;
        let t1546 = t64 * t209;
        let t1577 = 25.0 / 2187.0 * t1545 * t1546 * t827 * t156 * t360 + t1390 * t452 * t572 / 243.0 + 25.0 / 8748.0 * t825 * t69 * t828 * t289 - 1900.0 / 59049.0 * t458 * t259 * t461 + 400.0 / 19683.0 * t458 * t133 * t836 - 25.0 / 13122.0 * t458 * t69 * t1515 * t460 - 25.0 / 13122.0 * t458 * t69 * t659 * t654 + 65.0 / 6561.0 * t764 + 20.0 / 6561.0 * t798 + 35.0 / 6561.0 * t802 + 20.0 / 2187.0 * t762;
        let t1583 = piecewise3(t3, 0.0, -t112 * t483 * t470 / 30.0 + t112 * t232 * t845 / 5.0 + 3.0 / 20.0 * t112 * t113 * (5.0 / 6561.0 * t1479 * t1509 * t268 + 6.0 * t815 * t1008 - 4.0 * t237 * t1515 * t92 - 4.0 * t355 * t682 - 24.0 * t1521 * t1294 - 5.0 / 19683.0 * t795 * t840 * t289 - t760 * t452 * t497 / 729.0 + 24.0 * t644 * t698 * t360 - 25.0 / 2187.0 * t1534 * t69 * t1535 * t268 - 200.0 / 6561.0 * t825 * t133 * t829 + t1577));
        let tv4rho2lapl20 = 2.0 * rho[ip] * t1583 + 4.0 * t850;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let tv4rho2tau20 = 0.0;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t1590 = t460 * t137;
        let t1593 = t366 * t316;
        let t1596 = t366 * t395;
        let t1597 = t1596 * t137;
        let t1602 = t92 * t711;
        let t1608 = t366 * t341;
        let t1611 = t366 * t412;
        let t1623 = t86 * t152;
        let t1631 = -24.0 * t1002 * t853 * t1590 + 18.0 * t697 * t1593 + 18.0 * t571 * t1597 - 6.0 * t575 * t857 - 6.0 * t303 * t1602 - 24.0 * t1042 * t860 * t1211 + 18.0 * t714 * t1608 + 18.0 * t592 * t1611 * t156 - 6.0 * t596 * t864 - 6.0 * t319 * t92 * t737 - 2.0 * t267 * t872 * t320 + t141 * (-20.0 / 19683.0 * t1623 * t70 - 125.0 / 236196.0 * t1623 * t92) * t70;
        let t1636 = piecewise3(t3, 0.0, t112 * t232 * t875 / 10.0 + 3.0 / 20.0 * t112 * t113 * t1631);
        let tv4rhosigma30 = 2.0 * rho[ip] * t1636 + 2.0 * t879;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t1685 = t460 * t387;
        let t1689 = 5.0 / 27.0 * t767 * t768 * t769 * t341 - 10.0 / 27.0 * t1406 * t768 * t827 * t398 * t156 + 5.0 / 54.0 * t767 * t768 * t460 * t412 * t156 - 20.0 / 81.0 * t658 * t142 * t889 + 5.0 / 27.0 * t1475 * t771 + 20.0 / 243.0 * t364 * t142 * t900 - 5.0 / 162.0 * t364 * t79 * t667 * t412 - 5.0 / 162.0 * t364 * t79 * t367 * t737 + 5.0 / 54.0 * t658 * t79 * t1238 * t398 + 5.0 / 17496.0 * t795 * t796 * t341 - 35.0 / 17496.0 * t795 * t1451 * t193 + 12.0 * t644 * t751 * t316 - 24.0 * t1205 * t1685 * t137;
        let t1694 = t42 * t31;
        let t1695 = t1389 * t1694;
        let t1697 = t177 * t137;
        let t1713 = t193 * t156;
        let t1728 = 6.0 * t644 * t1597 + 5.0 / 17496.0 * t794 * t797 - t1695 * t55 * t366 * t1697 / 216.0 + 40.0 / 243.0 * t786 * t1412 * t893 - 5.0 / 81.0 * t786 * t768 * t1608 - 5.0 / 81.0 * t786 * t1416 * t893 + t760 * t56 * t701 / 648.0 - 5.0 / 5832.0 * t1479 * t1480 * t1713 - 13.0 / 1944.0 * t760 * t69 * t421 + 35.0 / 34992.0 * t436 * t93 + 6.0 * t1262 * t881 - 2.0 * t647 * t857 - 2.0 * t355 * t1602;
        let t1734 = piecewise3(t3, 0.0, t112 * t232 * t907 / 10.0 + 3.0 / 20.0 * t112 * t113 * (t1689 + t1728));
        let tv4rhosigma2lapl0 = 2.0 * rho[ip] * t1734 + 2.0 * t911;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let tv4rhosigma2tau0 = 0.0;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let t1749 = t644 * t366 * t40;
        let t1754 = t647 * t92;
        let t1781 = -24.0 * t1521 * t1468 * t137 + 12.0 * t644 * t751 * t360 + 6.0 * t815 * t1593 - t1749 * t1694 * t55 * t137 / 216.0 + t1754 * t916 / 648.0 - 13.0 / 1944.0 * t915 * t391 + t1695 * t48 * t366 * t1697 / 243.0 + 10.0 / 2187.0 * t885 - t760 * t452 * t701 / 729.0 - 25.0 / 2187.0 * t1533 * t1544 * t64 * t448 * t963 * t193 * t156 - 100.0 / 6561.0 * t825 * t133 * t922 + 25.0 / 4374.0 * t1545 * t1546 * t1407 * t360;
        let t1786 = t824 * t926;
        let t1793 = t657 * t932;
        let t1794 = t1272 * t156;
        let t1799 = 1.0 / t23 / t284;
        let t1817 = 25.0 / 8748.0 * t825 * t69 * t828 * t341 + 25.0 / 4374.0 * t1786 * t830 + 200.0 / 19683.0 * t927 * t833 - 25.0 / 6561.0 * t927 * t837 - 25.0 / 1889568.0 * t1793 * t936 * t1794 - 725.0 / 0.17006112e8 * t933 * t32 * t1799 * t659 + 25.0 / 5668704.0 * t933 * t936 * t1238 + 5.0 / 6561.0 * t1479 * t1509 * t1713 + 10.0 / 6561.0 * t898 - 5.0 / 19683.0 * t795 * t840 * t341 - 5.0 / 19683.0 * t794 * t841 - 5.0 / 6561.0 * t905;
        let t1823 = piecewise3(t3, 0.0, t112 * t232 * t946 / 10.0 + 3.0 / 20.0 * t112 * t113 * (t1781 + t1817));
        let tv4rhosigmalapl20 = 2.0 * rho[ip] * t1823 + 2.0 * t950;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let tv4rhosigmatau20 = 0.0;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let t1829 = t1002 * t952;
        let t1841 = t1042 * t86;
        let t1842 = t1841 * t87;
        let t1844 = 1.0 / t459 / t365;
        let t1866 = t959 * t973 * t827;
        let t1867 = t32 * t209;
        let t1880 = -24.0 * t1829 * t1590 + 18.0 * t815 * t667 + t1749 * t1694 * t48 * t137 / 81.0 - t1754 * t956 / 243.0 + 10.0 / 729.0 * t917 - 125.0 / 59049.0 * t1842 * t961 * t1844 * t156 - 250.0 / 59049.0 * t960 * t152 * t952 * t963 + 125.0 / 78732.0 * t960 * t90 * t448 * t963 * t360 + 25.0 / 1062882.0 * t1793 * t969 * t1794 + 325.0 / 4782969.0 * t938 - 25.0 / 3188646.0 * t933 * t969 * t1238 + 25.0 / 2125764.0 * t1866 * t30 * t1867 * t156 + 325.0 / 9565938.0 * t794 * t60 * t935 * t977 - 25.0 / 6377292.0 * t974 * t975 * t785 * t360;
        let t1885 = piecewise3(t3, 0.0, t112 * t232 * t980 / 10.0 + 3.0 / 20.0 * t112 * t113 * t1880);
        let tv4rholapl30 = 2.0 * rho[ip] * t1885 + 2.0 * t984;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let tv4rhotau30 = 0.0;
        v4rhotau3[ip] += tv4rhotau30;
        let t1888 = t387 * t387;
        let t1894 = t395 * t395;
        let t1898 = t398 * t398;
        let t1904 = t412 * t412;
        let t1915 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * (-24.0 * t1002 * t1888 * t460 - 24.0 * t1042 * t1898 * t460 - 6.0 * t237 * t1894 * t92 - 6.0 * t267 * t1904 * t92 - 8.0 * t319 * t92 * t872 + 36.0 * t697 * t1596 + 36.0 * t714 * t1611));
        let tv4sigma40 = 2.0 * rho[ip] * t1915;
        v4sigma4[ip] += tv4sigma40;
        let t1956 = -24.0 * t1205 * t460 * t853 - t1390 * t56 * t881 / 144.0 + 18.0 * t644 * t751 * t395 + t760 * t56 * t857 / 432.0 - 10.0 / 27.0 * t1271 * t79 * t1272 * t860 + 5.0 / 18.0 * t1475 * t768 * t460 * t398 - 5.0 / 3888.0 * t1479 * t1480 * t398 + 5.0 / 18.0 * t767 * t768 * t769 * t412 + 5.0 / 5832.0 * t794 * t897 - 5.0 / 54.0 * t786 * t768 * t1611 + 5.0 / 11664.0 * t795 * t796 * t412 - 5.0 / 162.0 * t364 * t79 * t367 * t872;
        let t1960 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t1956);
        let tv4sigma3lapl0 = 2.0 * rho[ip] * t1960;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let tv4sigma3tau0 = 0.0;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let t1970 = t237 * t25 * t931;
        let t1986 = t1272 * t193;
        let t1998 = t363 * t931 * sigma[ip];
        let t2001 = t85 * t85;
        let t2003 = t267 / t2001;
        let t2018 = -24.0 * t1521 * t1685 - t644 * t751 * t916 / 108.0 + 6.0 * t815 * t1596 - t1970 * t969 * t92 / 279936.0 + t1390 * t452 * t881 / 243.0 - t760 * t452 * t857 / 729.0 - 25.0 / 2187.0 * t1534 * t69 * t1535 * t398 + 25.0 / 2187.0 * t1786 * t923 - 25.0 / 944784.0 * t1793 * t936 * t1986 + 25.0 / 8748.0 * t825 * t69 * t828 * t412 - 25.0 / 13122.0 * t456 * t42 * t462 + 25.0 / 1417176.0 * t1998 * t937 - 25.0 / 0.1224440064e10 * t2003 * t60 / t89 / t127 * t460 + 5.0 / 6561.0 * t1479 * t1509 * t398 - 10.0 / 19683.0 * t794 * t940 - 5.0 / 19683.0 * t795 * t840 * t412;
        let t2022 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t2018);
        let tv4sigma2lapl20 = 2.0 * rho[ip] * t2022;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let tv4sigma2tau20 = 0.0;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let t2026 = t815 * t366;
        let t2051 = t1535 * t43 * t31;
        let t2073 = -24.0 * t1829 * t1468 - t2026 * t916 / 144.0 + t1749 * t1694 * t48 * t177 / 81.0 + t1970 * t32 / t23 / t253 * t92 / 104976.0 - 125.0 / 59049.0 * t1842 * t961 * t1844 * t193 + 125.0 / 78732.0 * t959 * t60 * t964 - 125.0 / 0.204073344e9 * t959 * t87 / t22 / t1076 * t2051 + 25.0 / 1062882.0 * t1793 * t969 * t1986 - 25.0 / 1594323.0 * t1998 * t970 + 25.0 / 0.459165024e9 * t2003 * t60 / t89 / t62 * t460 + 25.0 / 2125764.0 * t1866 * t30 * t1867 * t193 - 25.0 / 3188646.0 * t794 * sigma[ip] * t968 * t977;
        let t2077 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t2073);
        let tv4sigmalapl30 = 2.0 * rho[ip] * t2077;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let tv4sigmatau30 = 0.0;
        v4sigmatau3[ip] += tv4sigmatau30;
        let t2079 = t448 * t448;
        let t2088 = t60 * t60;
        let t2091 = t459 * t459;
        let t2098 = 1.0 / t22 / t548;
        let t2120 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * (-24.0 * t1002 * t2079 * t460 + 2.0 / 81.0 * t2026 * t956 - t1970 * t1058 * t92 / 59049.0 - 625.0 / 0.19131876e8 * t1841 * t2088 * t1799 * t2079 / t2091 * t180 + 125.0 / 0.86093442e8 * t959 * t87 * t2098 * t2051 + 125.0 / 0.172186884e9 * t824 / t27 / t85 / t26 * t87 * t31 * t2098 * t1535 - 25.0 / 0.258280326e9 * t2003 * t60 * t1077 * t460));
        let tv4lapl40 = 2.0 * rho[ip] * t2120;
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
