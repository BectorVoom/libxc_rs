//! LDA_XC_KSDT vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_ksdt.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_ksdt_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_T: f64,
    param_b_0_1: f64,
    param_b_0_2: f64,
    param_b_0_0: f64,
    param_b_0_3: f64,
    param_b_0_4: f64,
    param_c_0_1: f64,
    param_c_0_2: f64,
    param_c_0_0: f64,
    param_e_0_1: f64,
    param_e_0_2: f64,
    param_e_0_0: f64,
    param_e_0_3: f64,
    param_e_0_4: f64,
    param_d_0_1: f64,
    param_d_0_2: f64,
    param_d_0_0: f64,
    param_d_0_3: f64,
    param_d_0_4: f64,
    param_b_1_1: f64,
    param_b_1_2: f64,
    param_b_1_0: f64,
    param_b_1_3: f64,
    param_b_1_4: f64,
    param_c_1_1: f64,
    param_c_1_2: f64,
    param_c_1_0: f64,
    param_e_1_1: f64,
    param_e_1_2: f64,
    param_e_1_0: f64,
    param_e_1_3: f64,
    param_e_1_4: f64,
    param_d_1_1: f64,
    param_d_1_2: f64,
    param_d_1_0: f64,
    param_d_1_3: f64,
    param_d_1_4: f64,
    param_thetaParam: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = 1.0 / M_PI;
        let t2 = M_CBRT4;
        let t3 = t2 * t2;
        let t4 = t1 * t3;
        let t5 = pow_1_3(9.0);
        let t6 = t4 * t5;
        let t7 = pow_1_3(t1);
        let t8 = 1.0 / t7;
        let t9 = t5 * t5;
        let t10 = t7 * t1;
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t13 = 1.0 / param_T;
        let t14 = M_CBRT3;
        let t15 = t13 * t14;
        let t16 = pow_1_3(rho[ip]);
        let t17 = t16 * t16;
        let t18 = t15 * t17;
        let t21 = f64::tanh(t12 * t18 / 6.0);
        let t22 = t8 * t21;
        let t23 = M_PI * M_PI;
        let t24 = 1.0 / t23;
        let t25 = t7 * t7;
        let t26 = t25 * t24;
        let t27 = t9 * t26;
        let t28 = param_T * param_T;
        let t29 = t28 * t14;
        let t31 = 1.0 / t16 / rho[ip];
        let t32 = t29 * t31;
        let t33 = t27 * t32;
        let t35 = t28 * param_T;
        let t36 = rho[ip] * rho[ip];
        let t37 = 1.0 / t36;
        let t38 = t35 * t37;
        let t40 = t23 * t23;
        let t41 = t40 * M_PI;
        let t43 = t7 / t41;
        let t44 = t5 * t43;
        let t45 = t28 * t28;
        let t46 = t14 * t14;
        let t47 = t45 * t46;
        let t49 = 1.0 / t17 / t36;
        let t50 = t47 * t49;
        let t51 = t44 * t50;
        let t53 = 0.75 + 0.45090814814814817 * t33 - 0.0008419930512353099 * t38 + 0.3364938271604938 * t51;
        let t56 = 1.0 + 1.2311866666666667 * t33 + 1.0094814814814814 * t51;
        let t57 = 1.0 / t56;
        let t58 = t53 * t57;
        let t62 = M_SQRT2;
        let t63 = t5 * t10;
        let t64 = param_T * t46;
        let t65 = 1.0 / t17;
        let t67 = t63 * t64 * t65;
        let t68 = f64::sqrt(t67);
        let t72 = f64::tanh(3.0 / 2.0 * t62 / t68);
        let t76 = param_b_0_1 * t9 * t26;
        let t81 = param_b_0_2 * t5 * t43;
        let t84 = param_b_0_0 + 4.0 / 27.0 * t76 * t32 + 16.0 / 81.0 * t81 * t50;
        let t85 = t72 * t84;
        let t88 = param_b_0_3 * t9 * t26;
        let t93 = param_b_0_4 * t5 * t43;
        let t96 = 1.0 + 4.0 / 27.0 * t88 * t32 + 16.0 / 81.0 * t93 * t50;
        let t97 = 1.0 / t96;
        let t98 = t14 * t7;
        let t99 = 1.0 / t16;
        let t100 = t3 * t99;
        let t101 = t98 * t100;
        let t102 = f64::sqrt(t101);
        let t103 = t97 * t102;
        let t107 = param_c_0_1;
        let t108 = param_c_0_2;
        let t113 = f64::exp(-t108 * t9 * t11 * t18 / 6.0);
        let t115 = t107 * t113 + param_c_0_0;
        let t116 = t115 * t21;
        let t120 = param_e_0_1 * t9 * t26;
        let t125 = param_e_0_2 * t5 * t43;
        let t128 = param_e_0_0 + 4.0 / 27.0 * t120 * t32 + 16.0 / 81.0 * t125 * t50;
        let t131 = param_e_0_3 * t9 * t26;
        let t136 = param_e_0_4 * t5 * t43;
        let t139 = 1.0 + 4.0 / 27.0 * t131 * t32 + 16.0 / 81.0 * t136 * t50;
        let t140 = 1.0 / t139;
        let t141 = t128 * t140;
        let t142 = t116 * t141;
        let t146 = (t6 * t22 * t58 / 4.0 + t85 * t103 / 2.0 + t142 * t101 / 4.0) * t46;
        let t147 = t146 * t8;
        let t148 = t2 * t16;
        let t152 = param_d_0_1 * t9 * t26;
        let t157 = param_d_0_2 * t5 * t43;
        let t160 = param_d_0_0 + 4.0 / 27.0 * t152 * t32 + 16.0 / 81.0 * t157 * t50;
        let t161 = t72 * t160;
        let t164 = param_d_0_3 * t9 * t26;
        let t169 = param_d_0_4 * t5 * t43;
        let t172 = 1.0 + 4.0 / 27.0 * t164 * t32 + 16.0 / 81.0 * t169 * t50;
        let t173 = 1.0 / t172;
        let t174 = t173 * t102;
        let t177 = t21 * t128;
        let t178 = t177 * t140;
        let t181 = 1.0 + t161 * t174 / 2.0 + t178 * t101 / 4.0;
        let t182 = 1.0 / t181;
        let t183 = 1.0 <= zeta_threshold;
        let t185 = 2.0 / 3.0 - 0.003481525 * t101;
        let t187 = 1.0 + 0.045802 * t101;
        let t188 = 1.0 / t187;
        let t189 = t185 * t188;
        let t190 = t63 * param_T;
        let t191 = t46 * t65;
        let t195 = 1.064009 + 0.06361833333333333 * t190 * t191 * t102;
        let t199 = f64::exp(-2.0 / 9.0 * t190 * t191 * t195);
        let t201 = -t189 * t199 + 2.0;
        let t202 = f64::powf(zeta_threshold, t201);
        let t203 = piecewise3(t183, t202, 1.0);
        let t205 = 2.0 * t203 - 2.0;
        let t206 = f64::powf(2.0, t201);
        let t207 = t206 - 2.0;
        let t208 = 1.0 / t207;
        let t209 = t205 * t208;
        let t210 = 1.0 - t209;
        let t211 = t182 * t210;
        let t212 = t148 * t211;
        let t213 = t147 * t212;
        let t214 = M_CBRT2;
        let t215 = t214 * t1;
        let t216 = t3 * t5;
        let t217 = t215 * t216;
        let t220 = t214 * t214;
        let t224 = f64::tanh(t12 * t13 * t14 * t17 * t220 / 6.0);
        let t225 = t8 * t224;
        let t226 = t27 * t28;
        let t227 = t14 * t31;
        let t229 = t226 * t227 * t220;
        let t232 = t44 * t45;
        let t233 = t46 * t49;
        let t235 = t232 * t233 * t214;
        let t237 = 0.75 + 0.11272703703703704 * t229 - 0.00021049826280882748 * t38 + 0.042061728395061726 * t235;
        let t240 = 1.0 + 0.30779666666666666 * t229 + 0.12618518518518518 * t235;
        let t241 = 1.0 / t240;
        let t247 = t190 * t191 * t214;
        let t248 = f64::sqrt(t247);
        let t251 = f64::tanh(3.0 / t248);
        let t255 = param_b_1_1 * t9 * t26;
        let t257 = t29 * t31 * t220;
        let t262 = param_b_1_2 * t5 * t43;
        let t263 = t49 * t214;
        let t264 = t47 * t263;
        let t267 = param_b_1_0 + t255 * t257 / 27.0 + 2.0 / 81.0 * t262 * t264;
        let t268 = t251 * t267;
        let t271 = param_b_1_3 * t9 * t26;
        let t276 = param_b_1_4 * t5 * t43;
        let t279 = 1.0 + t271 * t257 / 27.0 + 2.0 / 81.0 * t276 * t264;
        let t280 = 1.0 / t279;
        let t281 = t280 * t102;
        let t285 = param_c_1_1;
        let t286 = param_c_1_2;
        let t293 = f64::exp(-t286 * t9 * t11 * t15 * t17 * t220 / 6.0);
        let t295 = t285 * t293 + param_c_1_0;
        let t296 = t295 * t224;
        let t300 = param_e_1_1 * t9 * t26;
        let t305 = param_e_1_2 * t5 * t43;
        let t308 = param_e_1_0 + t300 * t257 / 27.0 + 2.0 / 81.0 * t305 * t264;
        let t311 = param_e_1_3 * t9 * t26;
        let t316 = param_e_1_4 * t5 * t43;
        let t319 = 1.0 + t311 * t257 / 27.0 + 2.0 / 81.0 * t316 * t264;
        let t320 = 1.0 / t319;
        let t321 = t308 * t320;
        let t322 = t296 * t321;
        let t326 = (t217 * t225 * t237 * t241 / 4.0 + t268 * t281 / 2.0 + t322 * t101 / 4.0) * t46;
        let t327 = t8 * t2;
        let t328 = t326 * t327;
        let t332 = param_d_1_1 * t9 * t26;
        let t337 = param_d_1_2 * t5 * t43;
        let t340 = param_d_1_0 + t332 * t257 / 27.0 + 2.0 / 81.0 * t337 * t264;
        let t341 = t251 * t340;
        let t344 = param_d_1_3 * t9 * t26;
        let t349 = param_d_1_4 * t5 * t43;
        let t352 = 1.0 + t344 * t257 / 27.0 + 2.0 / 81.0 * t349 * t264;
        let t353 = 1.0 / t352;
        let t354 = t353 * t102;
        let t357 = t224 * t308;
        let t358 = t357 * t320;
        let t361 = 1.0 + t341 * t354 / 2.0 + t358 * t101 / 4.0;
        let t362 = 1.0 / t361;
        let t363 = t16 * t362;
        let t364 = t363 * t209;
        let t365 = t328 * t364;
        let tzk0 = -t213 / 3.0 - t365 / 3.0;
        zk[ip] += tzk0;
        let t369 = t25 * t1;
        let t371 = 1.0 / t369 * t13;
        let t372 = t4 * t371;
        let t373 = t14 * t99;
        let t374 = t21 * t21;
        let t375 = 1.0 - t374;
        let t377 = t375 * t53 * t57;
        let t382 = 1.0 / t16 / t36;
        let t383 = t29 * t382;
        let t384 = t27 * t383;
        let t386 = t36 * rho[ip];
        let t387 = 1.0 / t386;
        let t388 = t35 * t387;
        let t391 = 1.0 / t17 / t386;
        let t392 = t47 * t391;
        let t393 = t44 * t392;
        let t395 = -0.6012108641975309 * t384 + 0.0016839861024706198 * t388 - 0.8973168724279835 * t393;
        let t401 = t4 * t5 * t8;
        let t402 = t21 * t53;
        let t403 = t56 * t56;
        let t404 = 1.0 / t403;
        let t407 = -1.6415822222222223 * t384 - 2.6919506172839505 * t393;
        let t408 = t404 * t407;
        let t413 = 1.0 / t68 / t67;
        let t414 = t62 * t413;
        let t415 = t414 * t190;
        let t417 = 1.0 / t17 / rho[ip];
        let t418 = t46 * t417;
        let t419 = t72 * t72;
        let t420 = 1.0 - t419;
        let t421 = t418 * t420;
        let t422 = t84 * t97;
        let t423 = t422 * t102;
        let t431 = -16.0 / 81.0 * t76 * t383 - 128.0 / 243.0 * t81 * t392;
        let t432 = t72 * t431;
        let t435 = t96 * t96;
        let t436 = 1.0 / t435;
        let t437 = t436 * t102;
        let t442 = -16.0 / 81.0 * t88 * t383 - 128.0 / 243.0 * t93 * t392;
        let t443 = t437 * t442;
        let t446 = 1.0 / t102;
        let t447 = t97 * t446;
        let t448 = t85 * t447;
        let t449 = t3 * t31;
        let t450 = t98 * t449;
        let t453 = t107 * t108;
        let t454 = t453 * t9;
        let t455 = M_PI * t13;
        let t456 = t455 * t46;
        let t457 = t454 * t456;
        let t459 = t65 * t113 * t21;
        let t460 = t141 * t3;
        let t464 = t115 * t9;
        let t465 = t464 * t456;
        let t466 = t65 * t375;
        let t467 = t466 * t460;
        let t474 = -16.0 / 81.0 * t120 * t383 - 128.0 / 243.0 * t125 * t392;
        let t475 = t474 * t140;
        let t476 = t116 * t475;
        let t479 = t139 * t139;
        let t480 = 1.0 / t479;
        let t481 = t128 * t480;
        let t482 = t116 * t481;
        let t487 = -16.0 / 81.0 * t131 * t383 - 128.0 / 243.0 * t136 * t392;
        let t489 = t98 * t100 * t487;
        let t494 = t372 * t373 * t377 / 4.0 + t6 * t22 * t395 * t57 / 4.0 - t401 * t402 * t408 / 4.0 + t415 * t421 * t423 / 4.0 + t432 * t103 / 2.0 - t85 * t443 / 2.0 - t448 * t450 / 12.0 - t457 * t459 * t460 / 36.0 + t465 * t467 / 36.0 + t476 * t101 / 4.0 - t482 * t489 / 4.0 - t142 * t450 / 12.0;
        let t495 = t494 * t46;
        let t496 = t495 * t8;
        let t497 = t496 * t212;
        let t499 = t2 * t65;
        let t500 = t499 * t211;
        let t501 = t147 * t500;
        let t503 = t146 * t327;
        let t504 = t181 * t181;
        let t505 = 1.0 / t504;
        let t506 = t16 * t505;
        let t507 = t160 * t173;
        let t508 = t507 * t102;
        let t516 = -16.0 / 81.0 * t152 * t383 - 128.0 / 243.0 * t157 * t392;
        let t517 = t72 * t516;
        let t520 = t172 * t172;
        let t521 = 1.0 / t520;
        let t522 = t521 * t102;
        let t527 = -16.0 / 81.0 * t164 * t383 - 128.0 / 243.0 * t169 * t392;
        let t528 = t522 * t527;
        let t531 = t173 * t446;
        let t532 = t161 * t531;
        let t535 = t9 * M_PI;
        let t536 = t13 * t46;
        let t537 = t535 * t536;
        let t540 = t21 * t474;
        let t541 = t540 * t140;
        let t544 = t480 * t14;
        let t545 = t177 * t544;
        let t546 = t7 * t3;
        let t548 = t546 * t99 * t487;
        let t553 = t415 * t421 * t508 / 4.0 + t517 * t174 / 2.0 - t161 * t528 / 2.0 - t532 * t450 / 12.0 + t537 * t467 / 36.0 + t541 * t101 / 4.0 - t545 * t548 / 4.0 - t178 * t450 / 12.0;
        let t554 = t210 * t553;
        let t555 = t506 * t554;
        let t556 = t503 * t555;
        let t558 = t98 * t3;
        let t559 = t31 * t188;
        let t563 = t187 * t187;
        let t564 = 1.0 / t563;
        let t565 = t185 * t564;
        let t566 = t565 * t199;
        let t576 = t5 * t369 * param_T;
        let t581 = -0.04241222222222222 * t190 * t418 * t102 - 0.031809166666666666 * t576 * t37 * t446 * t3;
        let t585 = 4.0 / 27.0 * t190 * t418 * t195 - 2.0 / 9.0 * t190 * t191 * t581;
        let t586 = t585 * t199;
        let t588 = -0.0011605083333333334 * t558 * t559 * t199 - 0.015267333333333334 * t566 * t450 - t189 * t586;
        let t589 = t202 * t588;
        let t590 = f64::ln(zeta_threshold);
        let t592 = piecewise3(t183, t589 * t590, 0.0);
        let t593 = t592 * t208;
        let t595 = t207 * t207;
        let t596 = 1.0 / t595;
        let t597 = t205 * t596;
        let t598 = t206 * t588;
        let t599 = f64::ln(2.0);
        let t600 = t598 * t599;
        let t602 = t597 * t600 - 2.0 * t593;
        let t603 = t182 * t602;
        let t604 = t148 * t603;
        let t605 = t147 * t604;
        let t607 = t224 * t224;
        let t608 = 1.0 - t607;
        let t610 = t608 * t237 * t241;
        let t614 = t14 * t382;
        let t615 = t614 * t220;
        let t616 = t226 * t615;
        let t619 = t46 * t391;
        let t621 = t232 * t619 * t214;
        let t623 = -0.15030271604938272 * t616 + 0.00042099652561765496 * t388 - 0.11216460905349794 * t621;
        let t628 = t240 * t240;
        let t629 = 1.0 / t628;
        let t630 = t237 * t629;
        let t633 = -0.41039555555555557 * t616 - 0.3364938271604938 * t621;
        let t634 = t630 * t633;
        let t639 = 1.0 / t248 / t247;
        let t640 = t639 * t5;
        let t642 = t10 * param_T * t46;
        let t643 = t640 * t642;
        let t645 = t251 * t251;
        let t646 = 1.0 - t645;
        let t647 = t417 * t214 * t646;
        let t648 = t267 * t280;
        let t649 = t648 * t102;
        let t654 = t29 * t382 * t220;
        let t657 = t391 * t214;
        let t658 = t47 * t657;
        let t661 = -4.0 / 81.0 * t255 * t654 - 16.0 / 243.0 * t262 * t658;
        let t662 = t251 * t661;
        let t665 = t279 * t279;
        let t666 = 1.0 / t665;
        let t667 = t666 * t102;
        let t672 = -4.0 / 81.0 * t271 * t654 - 16.0 / 243.0 * t276 * t658;
        let t673 = t667 * t672;
        let t676 = t280 * t446;
        let t677 = t268 * t676;
        let t680 = t285 * t286;
        let t681 = t680 * t9;
        let t682 = t681 * t456;
        let t683 = t65 * t220;
        let t684 = t683 * t293;
        let t685 = t320 * t3;
        let t686 = t357 * t685;
        let t690 = t295 * t9;
        let t691 = t690 * t456;
        let t692 = t683 * t608;
        let t693 = t321 * t3;
        let t701 = -4.0 / 81.0 * t300 * t654 - 16.0 / 243.0 * t305 * t658;
        let t702 = t701 * t320;
        let t703 = t296 * t702;
        let t706 = t319 * t319;
        let t707 = 1.0 / t706;
        let t708 = t308 * t707;
        let t709 = t296 * t708;
        let t714 = -4.0 / 81.0 * t311 * t654 - 16.0 / 243.0 * t316 * t658;
        let t716 = t98 * t100 * t714;
        let t721 = t372 * t373 * t610 / 2.0 + t217 * t225 * t623 * t241 / 4.0 - t217 * t225 * t634 / 4.0 + t643 * t647 * t649 / 2.0 + t662 * t281 / 2.0 - t268 * t673 / 2.0 - t677 * t450 / 12.0 - t682 * t684 * t686 / 36.0 + t691 * t692 * t693 / 36.0 + t703 * t101 / 4.0 - t709 * t716 / 4.0 - t322 * t450 / 12.0;
        let t722 = t721 * t46;
        let t723 = t722 * t327;
        let t724 = t723 * t364;
        let t726 = t65 * t362;
        let t727 = t726 * t209;
        let t728 = t328 * t727;
        let t730 = t361 * t361;
        let t731 = 1.0 / t730;
        let t732 = t16 * t731;
        let t733 = t340 * t353;
        let t734 = t733 * t102;
        let t742 = -4.0 / 81.0 * t332 * t654 - 16.0 / 243.0 * t337 * t658;
        let t743 = t251 * t742;
        let t746 = t352 * t352;
        let t747 = 1.0 / t746;
        let t748 = t747 * t102;
        let t753 = -4.0 / 81.0 * t344 * t654 - 16.0 / 243.0 * t349 * t658;
        let t754 = t748 * t753;
        let t757 = t353 * t446;
        let t758 = t341 * t757;
        let t761 = t536 * t65;
        let t762 = t535 * t761;
        let t763 = t220 * t608;
        let t764 = t763 * t693;
        let t767 = t224 * t701;
        let t768 = t767 * t320;
        let t771 = t707 * t14;
        let t772 = t357 * t771;
        let t774 = t546 * t99 * t714;
        let t779 = t643 * t647 * t734 / 2.0 + t743 * t354 / 2.0 - t341 * t754 / 2.0 - t758 * t450 / 12.0 + t762 * t764 / 36.0 + t768 * t101 / 4.0 - t772 * t774 / 4.0 - t358 * t450 / 12.0;
        let t780 = t209 * t779;
        let t781 = t732 * t780;
        let t782 = t328 * t781;
        let t784 = t363 * t593;
        let t785 = t328 * t784;
        let t787 = t327 * t16;
        let t788 = t326 * t787;
        let t789 = t362 * t205;
        let t790 = t789 * t596;
        let t791 = t790 * t600;
        let t792 = t788 * t791;
        let tvrho0 = -t213 / 3.0 - t365 / 3.0 + rho[ip] * (-t497 / 3.0 - t501 / 9.0 + t556 / 3.0 - t605 / 3.0 - t724 / 3.0 - t728 / 9.0 + t782 / 3.0 - 2.0 / 3.0 * t785 + t792 / 3.0);
        vrho[ip] += tvrho0;
    }
}
