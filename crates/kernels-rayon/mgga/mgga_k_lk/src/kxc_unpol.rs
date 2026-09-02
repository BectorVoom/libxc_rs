//! MGGA_K_LK kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_lk.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_lk_kxc_unpol(
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
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t98 = 1.0 + (5.0 / 324.0 * t78 * t79 * t81 + 125.0 / 11337408.0 * t88 * t93) * t70;
        let t102 = 1.0 + param_kappa * (2.0 - 1.0 / t76 - 1.0 / t98);
        let t106 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t21 * t23 * t102);
        let tzk0 = 2.0 * t106;
        zk[ip] += tzk0;
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
    }
}
