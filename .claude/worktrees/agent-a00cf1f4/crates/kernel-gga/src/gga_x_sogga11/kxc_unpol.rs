//! GGA_X_SOGGA11 kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sogga11.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_sogga11_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t21 = param_a_1;
        let t22 = M_CBRT6;
        let t23 = param_mu * t22;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t26 = t25 * t25;
        let t27 = 1.0 / t26;
        let t28 = t23 * t27;
        let t29 = 1.0 / param_kappa;
        let t30 = t29 * sigma[ip];
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = rho[ip] * rho[ip];
        let t34 = t18 * t18;
        let t36 = 1.0 / t34 / t33;
        let t37 = t32 * t36;
        let t40 = t28 * t30 * t37 / 24.0;
        let t41 = 1.0 + t40;
        let t43 = 1.0 - 1.0 / t41;
        let t45 = param_a_2;
        let t46 = t43 * t43;
        let t48 = param_a_3;
        let t49 = t46 * t43;
        let t51 = param_a_4;
        let t52 = t46 * t46;
        let t54 = param_a_5;
        let t58 = param_b_1;
        let t59 = f64::exp(-t40);
        let t60 = 1.0 - t59;
        let t62 = param_b_2;
        let t63 = t60 * t60;
        let t65 = param_b_3;
        let t66 = t63 * t60;
        let t68 = param_b_4;
        let t69 = t63 * t63;
        let t71 = param_b_5;
        let t74 = t54 * t52 * t43 + t71 * t69 * t60 + t21 * t43 + t45 * t46 + t48 * t49 + t51 * t52 + t58 * t60 + t62 * t63 + t65 * t66 + t68 * t69 + param_a_0 + param_b_0;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t17 / t34;
        let t84 = t41 * t41;
        let t85 = 1.0 / t84;
        let t87 = t21 * t85 * t23;
        let t88 = t27 * t29;
        let t89 = sigma[ip] * t32;
        let t90 = t33 * rho[ip];
        let t92 = 1.0 / t34 / t90;
        let t94 = t88 * t89 * t92;
        let t97 = t45 * t43;
        let t98 = t85 * param_mu;
        let t99 = t98 * t22;
        let t100 = t97 * t99;
        let t103 = t48 * t46;
        let t104 = t103 * t99;
        let t107 = t51 * t49;
        let t108 = t107 * t99;
        let t111 = t54 * t52;
        let t112 = t111 * t99;
        let t116 = t22 * t27;
        let t117 = t58 * param_mu * t116;
        let t118 = t32 * t92;
        let t119 = t118 * t59;
        let t120 = t30 * t119;
        let t123 = t62 * t60;
        let t124 = t123 * t28;
        let t127 = t65 * t63;
        let t128 = t127 * t28;
        let t131 = t68 * t66;
        let t132 = t131 * t28;
        let t135 = t71 * t69;
        let t136 = t135 * t28;
        let t139 = -t87 * t94 / 9.0 - 2.0 / 9.0 * t100 * t94 - t104 * t94 / 3.0 - 4.0 / 9.0 * t108 * t94 - 5.0 / 9.0 * t112 * t94 - t117 * t120 / 9.0 - 2.0 / 9.0 * t124 * t120 - t128 * t120 / 3.0 - 4.0 / 9.0 * t132 * t120 - 5.0 / 9.0 * t136 * t120;
        let t144 = piecewise3(t2, 0.0, -t6 * t80 * t74 / 8.0 - 3.0 / 8.0 * t6 * t19 * t139);
        let tvrho0 = 2.0 * rho[ip] * t144 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t150 = t97 * t98;
        let t151 = t29 * t32;
        let t153 = t116 * t151 * t36;
        let t156 = t103 * t98;
        let t159 = t107 * t98;
        let t162 = t111 * t98;
        let t169 = t123 * t23;
        let t171 = t88 * t37 * t59;
        let t174 = t127 * t23;
        let t177 = t131 * t23;
        let t180 = t135 * t23;
        let t183 = t87 * t88 * t37 / 24.0 + t150 * t153 / 12.0 + t156 * t153 / 8.0 + t159 * t153 / 6.0 + 5.0 / 24.0 * t162 * t153 + t117 * t151 * t36 * t59 / 24.0 + t169 * t171 / 12.0 + t174 * t171 / 8.0 + t177 * t171 / 6.0 + 5.0 / 24.0 * t180 * t171;
        let t187 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t183);
        let tvsigma0 = 2.0 * rho[ip] * t187;
        vsigma[ip] += tvsigma0;
        let t192 = t17 / t34 / rho[ip];
        let t199 = t84 * t41;
        let t200 = 1.0 / t199;
        let t201 = param_mu * param_mu;
        let t202 = t200 * t201;
        let t203 = t22 * t22;
        let t204 = t202 * t203;
        let t205 = t97 * t204;
        let t207 = 1.0 / t25 / t24;
        let t208 = param_kappa * param_kappa;
        let t209 = 1.0 / t208;
        let t210 = t207 * t209;
        let t211 = sigma[ip] * sigma[ip];
        let t212 = t211 * t31;
        let t213 = t33 * t33;
        let t216 = 1.0 / t18 / t213 / t90;
        let t218 = t210 * t212 * t216;
        let t221 = t48 * t43;
        let t222 = t84 * t84;
        let t223 = 1.0 / t222;
        let t224 = t223 * t201;
        let t225 = t224 * t203;
        let t226 = t221 * t225;
        let t229 = t103 * t204;
        let t232 = t51 * t46;
        let t233 = t232 * t225;
        let t237 = 1.0 / t34 / t213;
        let t238 = t32 * t237;
        let t239 = t238 * t59;
        let t240 = t30 * t239;
        let t248 = t88 * t89 * t237;
        let t259 = t65 * t60;
        let t260 = t201 * t203;
        let t261 = t260 * t207;
        let t262 = t259 * t261;
        let t263 = t209 * t211;
        let t264 = t31 * t216;
        let t265 = t59 * t59;
        let t266 = t264 * t265;
        let t267 = t263 * t266;
        let t270 = t127 * t261;
        let t271 = t264 * t59;
        let t272 = t263 * t271;
        let t275 = -8.0 / 81.0 * t205 * t218 + 4.0 / 27.0 * t226 * t218 - 4.0 / 27.0 * t229 * t218 + 8.0 / 27.0 * t233 * t218 + 11.0 / 9.0 * t128 * t240 + 44.0 / 27.0 * t132 * t240 + 55.0 / 27.0 * t136 * t240 + 55.0 / 27.0 * t112 * t248 + 22.0 / 27.0 * t124 * t240 + 22.0 / 27.0 * t100 * t248 + 11.0 / 9.0 * t104 * t248 + 44.0 / 27.0 * t108 * t248 + 4.0 / 27.0 * t262 * t267 - 2.0 / 27.0 * t270 * t272;
        let t276 = t68 * t63;
        let t277 = t276 * t261;
        let t280 = t131 * t261;
        let t283 = t71 * t66;
        let t284 = t283 * t261;
        let t287 = t135 * t261;
        let t290 = t107 * t204;
        let t293 = t54 * t49;
        let t294 = t293 * t225;
        let t297 = t111 * t204;
        let t300 = t123 * t261;
        let t308 = t21 * t200 * t260;
        let t312 = t45 * t223 * t260;
        let t316 = t203 * t207;
        let t317 = t58 * t201 * t316;
        let t321 = t62 * t201 * t316;
        let t324 = 8.0 / 27.0 * t277 * t267 - 8.0 / 81.0 * t280 * t272 + 40.0 / 81.0 * t284 * t267 - 10.0 / 81.0 * t287 * t272 - 16.0 / 81.0 * t290 * t218 + 40.0 / 81.0 * t294 * t218 - 20.0 / 81.0 * t297 * t218 - 4.0 / 81.0 * t300 * t272 + 11.0 / 27.0 * t117 * t240 + 11.0 / 27.0 * t87 * t248 - 4.0 / 81.0 * t308 * t218 + 4.0 / 81.0 * t312 * t218 - 2.0 / 81.0 * t317 * t272 + 4.0 / 81.0 * t321 * t267;
        let t325 = t275 + t324;
        let t330 = piecewise3(t2, 0.0, t6 * t192 * t74 / 12.0 - t6 * t80 * t139 / 4.0 - 3.0 / 8.0 * t6 * t19 * t325);
        let tv2rho20 = 2.0 * rho[ip] * t330 + 4.0 * t144;
        v2rho2[ip] += tv2rho20;
        let t343 = t209 * t31;
        let t344 = t213 * t33;
        let t346 = 1.0 / t18 / t344;
        let t347 = t346 * t265;
        let t349 = t343 * t347 * sigma[ip];
        let t354 = t343 * t346 * sigma[ip] * t59;
        let t365 = t31 * t346;
        let t367 = t210 * t365 * sigma[ip];
        let t380 = -t87 * t88 * t118 / 9.0 - t117 * t151 * t92 * t59 / 9.0 - 5.0 / 27.0 * t284 * t349 + 5.0 / 108.0 * t287 * t354 - t262 * t349 / 18.0 + t270 * t354 / 36.0 - t277 * t349 / 9.0 + t280 * t354 / 27.0 - t233 * t367 / 9.0 + 2.0 / 27.0 * t290 * t367 - 5.0 / 27.0 * t294 * t367 + 5.0 / 54.0 * t297 * t367 + t300 * t354 / 54.0 + t205 * t367 / 27.0;
        let t385 = t88 * t119;
        let t391 = t116 * t151 * t92;
        let t412 = -t226 * t367 / 18.0 + t229 * t367 / 18.0 - 4.0 / 9.0 * t177 * t385 - 5.0 / 9.0 * t180 * t385 - 4.0 / 9.0 * t159 * t391 - 5.0 / 9.0 * t162 * t391 - 2.0 / 9.0 * t169 * t385 - 2.0 / 9.0 * t150 * t391 - t156 * t391 / 3.0 + t308 * t367 / 54.0 - t312 * t367 / 54.0 + t317 * t354 / 108.0 - t321 * t349 / 54.0 - t174 * t385 / 3.0;
        let t413 = t380 + t412;
        let t418 = piecewise3(t2, 0.0, -t6 * t80 * t183 / 8.0 - 3.0 / 8.0 * t6 * t19 * t413);
        let tv2rhosigma0 = 2.0 * rho[ip] * t418 + 2.0 * t187;
        v2rhosigma[ip] += tv2rhosigma0;
        let t421 = t213 * rho[ip];
        let t423 = 1.0 / t18 / t421;
        let t424 = t31 * t423;
        let t425 = t210 * t424;
        let t430 = t97 * t202;
        let t432 = t316 * t343 * t423;
        let t435 = t221 * t224;
        let t438 = t103 * t202;
        let t441 = t232 * t224;
        let t444 = t107 * t202;
        let t447 = t293 * t224;
        let t450 = t111 * t202;
        let t461 = t123 * t260;
        let t463 = t210 * t424 * t59;
        let t466 = t259 * t260;
        let t468 = t210 * t424 * t265;
        let t471 = t127 * t260;
        let t474 = t276 * t260;
        let t477 = t131 * t260;
        let t480 = t283 * t260;
        let t483 = t135 * t260;
        let t486 = -t308 * t425 / 144.0 + t312 * t425 / 144.0 - t430 * t432 / 72.0 + t435 * t432 / 48.0 - t438 * t432 / 48.0 + t441 * t432 / 24.0 - t444 * t432 / 36.0 + 5.0 / 72.0 * t447 * t432 - 5.0 / 144.0 * t450 * t432 - t317 * t343 * t423 * t59 / 288.0 + t321 * t343 * t423 * t265 / 144.0 - t461 * t463 / 144.0 + t466 * t468 / 48.0 - t471 * t463 / 96.0 + t474 * t468 / 24.0 - t477 * t463 / 72.0 + 5.0 / 72.0 * t480 * t468 - 5.0 / 288.0 * t483 * t463;
        let t490 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t486);
        let tv2sigma20 = 2.0 * rho[ip] * t490;
        v2sigma2[ip] += tv2sigma20;
        let t493 = t17 * t36;
        let t503 = t54 * t46;
        let t505 = 1.0 / t222 / t84;
        let t506 = t201 * param_mu;
        let t507 = t505 * t506;
        let t508 = t503 * t507;
        let t509 = t24 * t24;
        let t510 = 1.0 / t509;
        let t512 = 1.0 / t208 / param_kappa;
        let t513 = t510 * t512;
        let t514 = t211 * sigma[ip];
        let t515 = t213 * t213;
        let t516 = t515 * t90;
        let t517 = 1.0 / t516;
        let t519 = t513 * t514 * t517;
        let t523 = 1.0 / t222 / t41;
        let t524 = t523 * t506;
        let t525 = t293 * t524;
        let t528 = t223 * t506;
        let t529 = t111 * t528;
        let t532 = t506 * t510;
        let t533 = t123 * t532;
        let t534 = t512 * t514;
        let t536 = t534 * t517 * t59;
        let t539 = t97 * t528;
        let t542 = t221 * t524;
        let t545 = t103 * t528;
        let t548 = t51 * t43;
        let t549 = t548 * t507;
        let t553 = 1.0 / t34 / t421;
        let t555 = t88 * t89 * t553;
        let t558 = t32 * t553;
        let t559 = t558 * t59;
        let t560 = t30 * t559;
        let t564 = 1.0 / t18 / t515;
        let t565 = t31 * t564;
        let t567 = t263 * t565 * t59;
        let t571 = t210 * t212 * t564;
        let t574 = t565 * t265;
        let t575 = t263 * t574;
        let t578 = -160.0 / 81.0 * t508 * t519 + 320.0 / 81.0 * t525 * t519 - 80.0 / 81.0 * t529 * t519 - 16.0 / 243.0 * t533 * t536 - 32.0 / 81.0 * t539 * t519 + 32.0 / 27.0 * t542 * t519 - 16.0 / 27.0 * t545 * t519 - 64.0 / 81.0 * t549 * t519 - 154.0 / 81.0 * t87 * t555 - 154.0 / 81.0 * t117 * t560 + 22.0 / 81.0 * t317 * t567 + 44.0 / 81.0 * t308 * t571 - 44.0 / 81.0 * t321 * t575;
        let t581 = t259 * t532;
        let t583 = t534 * t517 * t265;
        let t586 = t127 * t532;
        let t589 = t68 * t60;
        let t590 = t589 * t532;
        let t591 = t265 * t59;
        let t593 = t534 * t517 * t591;
        let t596 = t276 * t532;
        let t599 = t131 * t532;
        let t602 = t71 * t63;
        let t603 = t602 * t532;
        let t606 = t283 * t532;
        let t609 = t135 * t532;
        let t612 = t232 * t524;
        let t615 = t107 * t528;
        let t619 = t65 * t506 * t510;
        let t623 = t62 * t506 * t510;
        let t626 = -44.0 / 81.0 * t312 * t571 + 16.0 / 27.0 * t581 * t583 - 8.0 / 81.0 * t586 * t536 - 64.0 / 81.0 * t590 * t593 + 32.0 / 27.0 * t596 * t583 - 32.0 / 243.0 * t599 * t536 - 160.0 / 81.0 * t603 * t593 + 160.0 / 81.0 * t606 * t583 - 40.0 / 243.0 * t609 * t536 + 64.0 / 27.0 * t612 * t519 - 64.0 / 81.0 * t615 * t519 - 16.0 / 81.0 * t619 * t593 + 16.0 / 81.0 * t623 * t583;
        let t629 = t45 * t523 * t506;
        let t633 = t48 * t505 * t506;
        let t637 = t21 * t223 * t506;
        let t641 = t58 * t506 * t510;
        let t662 = 32.0 / 81.0 * t629 * t519 - 16.0 / 81.0 * t633 * t519 - 16.0 / 81.0 * t637 * t519 - 8.0 / 243.0 * t641 * t536 - 440.0 / 81.0 * t294 * t571 + 220.0 / 81.0 * t297 * t571 - 440.0 / 81.0 * t284 * t575 + 110.0 / 81.0 * t287 * t567 - 88.0 / 27.0 * t277 * t575 + 88.0 / 81.0 * t280 * t567 - 44.0 / 27.0 * t262 * t575 + 22.0 / 27.0 * t270 * t567 - 308.0 / 81.0 * t100 * t555;
        let t689 = -154.0 / 27.0 * t104 * t555 - 616.0 / 81.0 * t108 * t555 - 770.0 / 81.0 * t136 * t560 - 770.0 / 81.0 * t112 * t555 - 308.0 / 81.0 * t124 * t560 - 88.0 / 27.0 * t233 * t571 - 154.0 / 27.0 * t128 * t560 - 616.0 / 81.0 * t132 * t560 - 44.0 / 27.0 * t226 * t571 + 44.0 / 27.0 * t229 * t571 + 44.0 / 81.0 * t300 * t567 + 88.0 / 81.0 * t205 * t571 + 176.0 / 81.0 * t290 * t571;
        let t691 = t578 + t626 + t662 + t689;
        let t696 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t493 * t74 + t6 * t192 * t139 / 4.0 - 3.0 / 8.0 * t6 * t80 * t325 - 3.0 / 8.0 * t6 * t19 * t691);
        let tv3rho30 = 2.0 * rho[ip] * t696 + 6.0 * t330;
        v3rho3[ip] += tv3rho30;
        let t713 = t515 * t33;
        let t714 = 1.0 / t713;
        let t715 = t512 * t714;
        let t716 = t265 * t211;
        let t717 = t715 * t716;
        let t720 = t211 * t59;
        let t721 = t715 * t720;
        let t728 = t591 * t211;
        let t729 = t715 * t728;
        let t735 = t513 * t714 * t211;
        let t746 = 11.0 / 27.0 * t87 * t88 * t238 + 11.0 / 27.0 * t117 * t151 * t237 * t59 - 20.0 / 27.0 * t606 * t717 + 5.0 / 81.0 * t609 * t721 - 2.0 / 9.0 * t581 * t717 + t586 * t721 / 27.0 + 8.0 / 27.0 * t590 * t729 + 4.0 / 81.0 * t599 * t721 + 8.0 / 27.0 * t549 * t735 - 8.0 / 9.0 * t612 * t735 + 8.0 / 27.0 * t615 * t735 + 20.0 / 27.0 * t508 * t735 - 40.0 / 27.0 * t525 * t735;
        let t761 = t88 * t239;
        let t765 = t116 * t151 * t237;
        let t776 = 10.0 / 27.0 * t529 * t735 + 2.0 / 81.0 * t533 * t721 + 4.0 / 27.0 * t539 * t735 - 4.0 / 9.0 * t542 * t735 + 2.0 / 9.0 * t545 * t735 + 20.0 / 27.0 * t603 * t729 - 4.0 / 9.0 * t596 * t717 + 22.0 / 27.0 * t169 * t761 + 22.0 / 27.0 * t150 * t765 + 55.0 / 27.0 * t162 * t765 + 55.0 / 27.0 * t180 * t761 + 44.0 / 27.0 * t159 * t765 + 11.0 / 9.0 * t174 * t761;
        let t780 = t209 * sigma[ip];
        let t781 = t780 * t266;
        let t786 = t210 * sigma[ip] * t31 * t216;
        let t793 = t343 * t216 * sigma[ip] * t59;
        let t812 = 44.0 / 27.0 * t177 * t761 + t321 * t781 / 6.0 + t312 * t786 / 6.0 - t308 * t786 / 6.0 - t317 * t793 / 12.0 + 11.0 / 9.0 * t156 * t765 + 2.0 / 27.0 * t633 * t735 + 2.0 / 27.0 * t637 * t735 + t641 * t721 / 81.0 + 2.0 / 27.0 * t619 * t729 - 2.0 / 27.0 * t623 * t717 - 4.0 / 27.0 * t629 * t735 + t226 * t786 / 2.0;
        let t837 = -t229 * t786 / 2.0 - t300 * t793 / 6.0 - t205 * t786 / 3.0 - 2.0 / 3.0 * t290 * t786 + 5.0 / 3.0 * t294 * t786 - 5.0 / 6.0 * t297 * t786 + t233 * t786 + t262 * t781 / 2.0 - t270 * t793 / 4.0 + t277 * t781 - t280 * t793 / 3.0 - 5.0 / 12.0 * t287 * t793 + 5.0 / 3.0 * t284 * t781;
        let t839 = t746 + t776 + t812 + t837;
        let t844 = piecewise3(t2, 0.0, t6 * t192 * t183 / 12.0 - t6 * t80 * t413 / 4.0 - 3.0 / 8.0 * t6 * t19 * t839);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t844 + 4.0 * t418;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t850 = t515 * rho[ip];
        let t851 = 1.0 / t850;
        let t853 = t513 * t851 * sigma[ip];
        let t864 = t210 * t365;
        let t879 = t512 * t851;
        let t880 = sigma[ip] * t59;
        let t881 = t879 * t880;
        let t884 = t265 * sigma[ip];
        let t885 = t879 * t884;
        let t890 = t591 * sigma[ip];
        let t891 = t879 * t890;
        let t908 = -t533 * t881 / 108.0 + t581 * t885 / 12.0 - t586 * t881 / 72.0 - t590 * t891 / 9.0 + t596 * t885 / 6.0 - t599 * t881 / 54.0 - 5.0 / 18.0 * t603 * t891 + 5.0 / 18.0 * t606 * t885 - 5.0 / 216.0 * t609 * t881 - t539 * t853 / 18.0 + t542 * t853 / 6.0;
        let t913 = t210 * t365 * t59;
        let t917 = t210 * t365 * t265;
        let t929 = t316 * t343 * t346;
        let t959 = 4.0 / 27.0 * t444 * t929 - 10.0 / 27.0 * t447 * t929 + 2.0 / 27.0 * t430 * t929 - t435 * t929 / 9.0 + t438 * t929 / 9.0 - t633 * t853 / 36.0 - t641 * t881 / 216.0 + t623 * t885 / 36.0 - t619 * t891 / 36.0 - t637 * t853 / 36.0 + t629 * t853 / 18.0;
        let t961 = -t549 * t853 / 9.0 + t612 * t853 / 3.0 - t615 * t853 / 9.0 - 5.0 / 18.0 * t508 * t853 + 5.0 / 9.0 * t525 * t853 + t308 * t864 / 27.0 - t312 * t864 / 27.0 + t317 * t343 * t346 * t59 / 54.0 - t321 * t343 * t347 / 27.0 - 5.0 / 36.0 * t529 * t853 + t908 - t545 * t853 / 12.0 + 2.0 / 27.0 * t477 * t913 - 10.0 / 27.0 * t480 * t917 + 5.0 / 54.0 * t483 * t913 - t466 * t917 / 9.0 + t471 * t913 / 18.0 - 2.0 / 9.0 * t474 * t917 + 5.0 / 27.0 * t450 * t929 + t461 * t913 / 27.0 - 2.0 / 9.0 * t441 * t929 + t959;
        let t966 = piecewise3(t2, 0.0, -t6 * t80 * t486 / 8.0 - 3.0 / 8.0 * t6 * t19 * t961);
        let tv3rhosigma20 = 2.0 * rho[ip] * t966 + 2.0 * t490;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t969 = t97 * t223;
        let t970 = 1.0 / t515;
        let t971 = t512 * t970;
        let t972 = t532 * t971;
        let t975 = t221 * t523;
        let t978 = t103 * t223;
        let t981 = t548 * t505;
        let t984 = t232 * t523;
        let t987 = t107 * t223;
        let t990 = t503 * t505;
        let t993 = t293 * t523;
        let t996 = t111 * t223;
        let t999 = t123 * t506;
        let t1001 = t513 * t970 * t59;
        let t1004 = t259 * t506;
        let t1006 = t513 * t970 * t265;
        let t1009 = t127 * t506;
        let t1012 = t969 * t972 / 48.0 - t975 * t972 / 16.0 + t978 * t972 / 32.0 + t981 * t972 / 24.0 - t984 * t972 / 8.0 + t987 * t972 / 24.0 + 5.0 / 48.0 * t990 * t972 - 5.0 / 24.0 * t993 * t972 + 5.0 / 96.0 * t996 * t972 + t999 * t1001 / 288.0 - t1004 * t1006 / 32.0 + t1009 * t1001 / 192.0;
        let t1013 = t589 * t506;
        let t1015 = t513 * t970 * t591;
        let t1018 = t276 * t506;
        let t1021 = t131 * t506;
        let t1024 = t602 * t506;
        let t1027 = t283 * t506;
        let t1030 = t135 * t506;
        let t1033 = t513 * t970;
        let t1049 = t1013 * t1015 / 24.0 - t1018 * t1006 / 16.0 + t1021 * t1001 / 144.0 + 5.0 / 48.0 * t1024 * t1015 - 5.0 / 48.0 * t1027 * t1006 + 5.0 / 576.0 * t1030 * t1001 + t637 * t1033 / 96.0 - t629 * t1033 / 48.0 + t633 * t1033 / 96.0 + t641 * t971 * t59 / 576.0 - t623 * t971 * t265 / 96.0 + t619 * t971 * t591 / 96.0;
        let t1050 = t1012 + t1049;
        let t1054 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t1050);
        let tv3sigma30 = 2.0 * rho[ip] * t1054;
        v3sigma3[ip] += tv3sigma30;
    }
}
