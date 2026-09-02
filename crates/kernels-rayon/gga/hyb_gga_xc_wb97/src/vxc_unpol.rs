//! HYB_GGA_XC_WB97 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/hyb_gga_xc_wb97.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_gga_xc_wb97_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_c_x_3: f64,
    param_c_x_4: f64,
    param_c_x_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_ss_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ab_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = M_CBRT3;
        let t6 = 1.0 / M_PI;
        let t7 = pow_1_3(t6);
        let t8 = t5 * t7;
        let t9 = M_CBRT4;
        let t10 = t9 * t9;
        let t11 = M_CBRT2;
        let t13 = t8 * t10 * t11;
        let t14 = 2.0 <= zeta_threshold;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * zeta_threshold;
        let t18 = piecewise3(t14, t16, 2.0 * t11);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = pow_1_3(9.0);
        let t22 = t21 * t21;
        let t23 = t7 * t7;
        let t25 = t22 * t23 * param_hyb_omega_0;
        let t26 = 1.0 / t19;
        let t28 = piecewise3(t14, t15, t11);
        let t30 = t11 / t28;
        let t33 = t25 * t5 * t26 * t30 / 18.0;
        let t34 = 1.35 <= t33;
        let t35 = 1.35 < t33;
        let t36 = piecewise3(t35, t33, 1.35);
        let t37 = t36 * t36;
        let t40 = t37 * t37;
        let t41 = 1.0 / t40;
        let t43 = t40 * t37;
        let t44 = 1.0 / t43;
        let t46 = t40 * t40;
        let t47 = 1.0 / t46;
        let t50 = 1.0 / t46 / t37;
        let t53 = 1.0 / t46 / t40;
        let t56 = 1.0 / t46 / t43;
        let t58 = t46 * t46;
        let t59 = 1.0 / t58;
        let t62 = piecewise3(t35, 1.35, t33);
        let t63 = rmath::sqrt(M_PI);
        let t64 = 1.0 / t62;
        let t66 = rmath::erf(t64 / 2.0);
        let t68 = t62 * t62;
        let t69 = 1.0 / t68;
        let t71 = rmath::exp(-t69 / 4.0);
        let t72 = t71 - 1.0;
        let t75 = t71 - 3.0 / 2.0 - 2.0 * t68 * t72;
        let t78 = 2.0 * t62 * t75 + t63 * t66;
        let t82 = piecewise3(t34, 1.0 / t37 / 36.0 - t41 / 960.0 + t44 / 26880.0 - t47 / 829440.0 + t50 / 28385280.0 - t53 / 1073479680.0 + t56 / 44590694400.0 - t59 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t62 * t78);
        let t84 = param_c_x_1;
        let t85 = t84 * sigma[ip];
        let t86 = t11 * t11;
        let t87 = rho[ip] * rho[ip];
        let t88 = t19 * t19;
        let t90 = 1.0 / t88 / t87;
        let t91 = t86 * t90;
        let t93 = sigma[ip] * t86 * t90;
        let t95 = 1.0 + 0.004 * t93;
        let t96 = 1.0 / t95;
        let t100 = param_c_x_2;
        let t101 = sigma[ip] * sigma[ip];
        let t102 = t100 * t101;
        let t103 = t87 * t87;
        let t104 = t103 * rho[ip];
        let t106 = 1.0 / t19 / t104;
        let t107 = t11 * t106;
        let t108 = t95 * t95;
        let t109 = 1.0 / t108;
        let t110 = t107 * t109;
        let t113 = param_c_x_3;
        let t114 = t101 * sigma[ip];
        let t115 = t113 * t114;
        let t116 = t103 * t103;
        let t117 = 1.0 / t116;
        let t118 = t108 * t95;
        let t119 = 1.0 / t118;
        let t120 = t117 * t119;
        let t123 = param_c_x_4;
        let t124 = t101 * t101;
        let t125 = t123 * t124;
        let t126 = t116 * t87;
        let t128 = 1.0 / t88 / t126;
        let t129 = t86 * t128;
        let t130 = t108 * t108;
        let t131 = 1.0 / t130;
        let t132 = t129 * t131;
        let t135 = param_c_x_0 + 0.004 * t85 * t91 * t96 + 3.2e-05 * t102 * t110 + 2.56e-07 * t115 * t120 + 1.024e-09 * t125 * t132;
        let t136 = t82 * t135;
        let t140 = piecewise3(t4, 0.0, -3.0 / 64.0 * t13 * t20 * t136);
        let t141 = 2.0 * t140;
        let t142 = piecewise3(t3, zeta_threshold, 1.0);
        let t143 = t8 * t10;
        let t146 = piecewise3(t3, 1.0 / t15, 1.0);
        let t148 = t143 * t26 * t11 * t146;
        let t150 = 1.0 + 0.053425 * t148;
        let t151 = rmath::sqrt(t148);
        let t154 = pow_3_2(t148);
        let t156 = t5 * t5;
        let t157 = t156 * t23;
        let t158 = t157 * t9;
        let t159 = 1.0 / t88;
        let t161 = t146 * t146;
        let t163 = t158 * t159 * t86 * t161;
        let t165 = 3.79785 * t151 + 0.8969 * t148 + 0.204775 * t154 + 0.123235 * t163;
        let t168 = 1.0 + 16.081824322151103 / t165;
        let t169 = rmath::ln(t168);
        let t171 = 0.062182 * t150 * t169;
        let t173 = piecewise3(0.0 <= zeta_threshold, t16, 0.0);
        let t177 = 1.0 / (2.0 * t11 - 2.0);
        let t178 = (t18 + t173 - 2.0) * t177;
        let t180 = 1.0 + 0.05137 * t148;
        let t185 = 7.05945 * t151 + 1.549425 * t148 + 0.420775 * t154 + 0.1562925 * t163;
        let t188 = 1.0 + 32.1646831778707 / t185;
        let t189 = rmath::ln(t188);
        let t193 = 1.0 + 0.0278125 * t148;
        let t198 = 5.1785 * t151 + 0.905775 * t148 + 0.1100325 * t154 + 0.1241775 * t163;
        let t201 = 1.0 + 29.608574643216677 / t198;
        let t202 = rmath::ln(t201);
        let t203 = t193 * t202;
        let t212 = piecewise3(t4, 0.0, t142 * (-t171 + t178 * (-0.03109 * t180 * t189 + t171 - 0.019751789702565206 * t203) + 0.019751789702565206 * t178 * t203) / 2.0);
        let t214 = param_c_ss_1;
        let t215 = t214 * sigma[ip];
        let t217 = 1.0 + 0.2 * t93;
        let t218 = 1.0 / t217;
        let t222 = param_c_ss_2;
        let t223 = t222 * t101;
        let t224 = t217 * t217;
        let t225 = 1.0 / t224;
        let t226 = t107 * t225;
        let t229 = param_c_ss_3;
        let t230 = t229 * t114;
        let t231 = t224 * t217;
        let t232 = 1.0 / t231;
        let t233 = t117 * t232;
        let t236 = param_c_ss_4;
        let t237 = t236 * t124;
        let t238 = t224 * t224;
        let t239 = 1.0 / t238;
        let t240 = t129 * t239;
        let t243 = param_c_ss_0 + 0.2 * t215 * t91 * t218 + 0.08 * t223 * t226 + 0.032 * t230 * t233 + 0.0064 * t237 * t240;
        let t245 = 2.0 * t212 * t243;
        let t247 = t8 * t10 * t26;
        let t249 = 1.0 + 0.053425 * t247;
        let t250 = rmath::sqrt(t247);
        let t253 = pow_3_2(t247);
        let t256 = t157 * t9 * t159;
        let t258 = 3.79785 * t250 + 0.8969 * t247 + 0.204775 * t253 + 0.123235 * t256;
        let t261 = 1.0 + 16.081824322151103 / t258;
        let t262 = rmath::ln(t261);
        let t265 = piecewise3(t3, t16, 1.0);
        let t268 = (2.0 * t265 - 2.0) * t177;
        let t270 = 1.0 + 0.0278125 * t247;
        let t275 = 5.1785 * t250 + 0.905775 * t247 + 0.1100325 * t253 + 0.1241775 * t256;
        let t278 = 1.0 + 29.608574643216677 / t275;
        let t279 = rmath::ln(t278);
        let t284 = -0.062182 * t249 * t262 + 0.019751789702565206 * t268 * t270 * t279 - 2.0 * t212;
        let t286 = param_c_ab_1;
        let t287 = t286 * sigma[ip];
        let t289 = 1.0 + 0.006 * t93;
        let t290 = 1.0 / t289;
        let t294 = param_c_ab_2;
        let t295 = t294 * t101;
        let t296 = t289 * t289;
        let t297 = 1.0 / t296;
        let t298 = t107 * t297;
        let t301 = param_c_ab_3;
        let t302 = t301 * t114;
        let t303 = t296 * t289;
        let t304 = 1.0 / t303;
        let t305 = t117 * t304;
        let t308 = param_c_ab_4;
        let t309 = t308 * t124;
        let t310 = t296 * t296;
        let t311 = 1.0 / t310;
        let t312 = t129 * t311;
        let t315 = param_c_ab_0 + 0.006 * t287 * t91 * t290 + 7.2e-05 * t295 * t298 + 8.64e-07 * t302 * t305 + 5.184e-09 * t309 * t312;
        let t316 = t284 * t315;
        let tzk0 = t141 + t245 + t316;
        zk[ip] += tzk0;
        let t317 = t18 * t159;
        let t321 = t37 * t36;
        let t322 = 1.0 / t321;
        let t324 = 1.0 / t19 / rho[ip];
        let t328 = t25 * t5 * t324 * t30 / 54.0;
        let t329 = piecewise3(t35, -t328, 0.0);
        let t332 = t40 * t36;
        let t333 = 1.0 / t332;
        let t336 = t40 * t321;
        let t337 = 1.0 / t336;
        let t341 = 1.0 / t46 / t36;
        let t345 = 1.0 / t46 / t321;
        let t349 = 1.0 / t46 / t332;
        let t353 = 1.0 / t46 / t336;
        let t357 = 1.0 / t58 / t36;
        let t361 = piecewise3(t35, 0.0, -t328);
        let t363 = t71 * t69;
        let t367 = t68 * t62;
        let t368 = 1.0 / t367;
        let t372 = t62 * t72;
        let t377 = t368 * t361 * t71 / 2.0 - 4.0 * t372 * t361 - t64 * t361 * t71;
        let t380 = -t363 * t361 + 2.0 * t361 * t75 + 2.0 * t62 * t377;
        let t384 = piecewise3(t34, -t322 * t329 / 18.0 + t333 * t329 / 240.0 - t337 * t329 / 4480.0 + t341 * t329 / 103680.0 - t345 * t329 / 2838528.0 + t349 * t329 / 89456640.0 - t353 * t329 / 3185049600.0 + t357 * t329 / 126340300800.0, -8.0 / 3.0 * t361 * t78 - 8.0 / 3.0 * t62 * t380);
        let t385 = t384 * t135;
        let t389 = t87 * rho[ip];
        let t391 = 1.0 / t88 / t389;
        let t392 = t86 * t391;
        let t396 = t84 * t101;
        let t397 = t103 * t87;
        let t399 = 1.0 / t19 / t397;
        let t400 = t11 * t399;
        let t401 = t400 * t109;
        let t406 = t100 * t114;
        let t407 = t116 * rho[ip];
        let t408 = 1.0 / t407;
        let t409 = t408 * t119;
        let t414 = t113 * t124;
        let t415 = t116 * t389;
        let t417 = 1.0 / t88 / t415;
        let t419 = t417 * t131 * t86;
        let t424 = t124 * sigma[ip];
        let t425 = t123 * t424;
        let t426 = t116 * t397;
        let t429 = t11 / t19 / t426;
        let t431 = 1.0 / t130 / t95;
        let t432 = t429 * t431;
        let t435 = -0.010666666666666666 * t85 * t392 * t96 + 8.533333333333334e-05 * t396 * t401 - 0.00017066666666666668 * t102 * t401 + 1.3653333333333333e-06 * t406 * t409 - 2.048e-06 * t115 * t409 + 8.192e-09 * t414 * t419 - 1.0922666666666667e-08 * t125 * t419 + 8.738133333333333e-11 * t425 * t432;
        let t436 = t82 * t435;
        let t441 = piecewise3(t4, 0.0, -t13 * t317 * t136 / 64.0 - 3.0 / 64.0 * t13 * t20 * t385 - 3.0 / 64.0 * t13 * t20 * t436);
        let t443 = t324 * t11;
        let t444 = t146 * t169;
        let t447 = 0.0011073577833333333 * t143 * t443 * t444;
        let t448 = t165 * t165;
        let t449 = 1.0 / t448;
        let t450 = t150 * t449;
        let t453 = 1.0 / t151 * t5 * t7;
        let t454 = t10 * t324;
        let t455 = t11 * t146;
        let t456 = t454 * t455;
        let t457 = t453 * t456;
        let t459 = t443 * t146;
        let t460 = t143 * t459;
        let t462 = rmath::sqrt(t148);
        let t464 = t462 * t5 * t7;
        let t465 = t464 * t456;
        let t468 = 1.0 / t88 / rho[ip];
        let t471 = t158 * t468 * t86 * t161;
        let t473 = -0.632975 * t457 - 0.29896666666666666 * t460 - 0.1023875 * t465 - 0.08215666666666667 * t471;
        let t474 = 1.0 / t168;
        let t475 = t473 * t474;
        let t477 = 1.0 * t450 * t475;
        let t478 = t146 * t189;
        let t482 = t185 * t185;
        let t483 = 1.0 / t482;
        let t484 = t180 * t483;
        let t489 = -1.176575 * t457 - 0.516475 * t460 - 0.2103875 * t465 - 0.104195 * t471;
        let t490 = 1.0 / t188;
        let t491 = t489 * t490;
        let t494 = t146 * t202;
        let t498 = t198 * t198;
        let t499 = 1.0 / t498;
        let t500 = t193 * t499;
        let t505 = -0.8630833333333333 * t457 - 0.301925 * t460 - 0.05501625 * t465 - 0.082785 * t471;
        let t506 = 1.0 / t201;
        let t507 = t505 * t506;
        let t512 = t178 * t8;
        let t513 = t455 * t202;
        let t517 = t178 * t193;
        let t519 = t499 * t505 * t506;
        let t525 = piecewise3(t4, 0.0, t142 * (t447 + t477 + t178 * (0.0005323644333333333 * t143 * t443 * t478 + 1.0 * t484 * t491 - t447 - t477 + 0.0001831155503675316 * t143 * t443 * t494 + 0.5848223397455204 * t500 * t507) - 0.0001831155503675316 * t512 * t454 * t513 - 0.5848223397455204 * t517 * t519) / 2.0);
        let t526 = t525 * t243;
        let t531 = t214 * t101;
        let t532 = t400 * t225;
        let t537 = t222 * t114;
        let t538 = t408 * t232;
        let t543 = t229 * t124;
        let t545 = t417 * t239 * t86;
        let t550 = t236 * t424;
        let t552 = 1.0 / t238 / t217;
        let t553 = t429 * t552;
        let t556 = -0.5333333333333333 * t215 * t392 * t218 + 0.21333333333333335 * t531 * t532 - 0.4266666666666667 * t223 * t532 + 0.17066666666666666 * t537 * t538 - 0.256 * t230 * t538 + 0.0512 * t543 * t545 - 0.06826666666666667 * t237 * t545 + 0.027306666666666667 * t550 * t553;
        let t557 = t212 * t556;
        let t562 = t258 * t258;
        let t563 = 1.0 / t562;
        let t564 = t249 * t563;
        let t566 = 1.0 / t250 * t5;
        let t567 = t7 * t10;
        let t568 = t567 * t324;
        let t569 = t566 * t568;
        let t571 = t8 * t454;
        let t573 = rmath::sqrt(t247);
        let t574 = t573 * t5;
        let t575 = t574 * t568;
        let t578 = t157 * t9 * t468;
        let t580 = -0.632975 * t569 - 0.29896666666666666 * t571 - 0.1023875 * t575 - 0.08215666666666667 * t578;
        let t581 = 1.0 / t261;
        let t582 = t580 * t581;
        let t585 = t268 * t5;
        let t590 = t268 * t270;
        let t591 = t275 * t275;
        let t592 = 1.0 / t591;
        let t597 = -0.8630833333333333 * t569 - 0.301925 * t571 - 0.05501625 * t575 - 0.082785 * t578;
        let t599 = 1.0 / t278;
        let t600 = t592 * t597 * t599;
        let t604 = 0.0011073577833333333 * t8 * t454 * t262 + 1.0 * t564 * t582 - 0.0001831155503675316 * t585 * t567 * t324 * t279 - 0.5848223397455204 * t590 * t600 - 2.0 * t525;
        let t605 = t604 * t315;
        let t609 = t286 * t101;
        let t610 = t400 * t297;
        let t615 = t294 * t114;
        let t616 = t408 * t304;
        let t621 = t301 * t124;
        let t623 = t417 * t311 * t86;
        let t628 = t308 * t424;
        let t630 = 1.0 / t310 / t289;
        let t631 = t429 * t630;
        let t634 = -0.016 * t287 * t392 * t290 + 0.000192 * t609 * t610 - 0.000384 * t295 * t610 + 4.608e-06 * t615 * t616 - 6.912e-06 * t302 * t616 + 4.1472e-08 * t621 * t623 - 5.5296e-08 * t309 * t623 + 6.63552e-10 * t628 * t631;
        let t635 = t284 * t634;
        let tvrho0 = t141 + t245 + t316 + rho[ip] * (2.0 * t441 + 2.0 * t526 + 2.0 * t557 + t605 + t635);
        vrho[ip] += tvrho0;
        let t638 = t84 * t86;
        let t644 = t100 * sigma[ip];
        let t649 = t113 * t101;
        let t654 = t123 * t114;
        let t657 = t116 * t104;
        let t660 = t11 / t19 / t657;
        let t661 = t660 * t431;
        let t664 = 0.004 * t638 * t90 * t96 - 3.2e-05 * t85 * t110 + 6.4e-05 * t644 * t110 - 5.12e-07 * t102 * t120 + 7.68e-07 * t649 * t120 - 3.072e-09 * t115 * t132 + 4.096e-09 * t654 * t132 - 3.2768e-11 * t125 * t661;
        let t665 = t82 * t664;
        let t669 = piecewise3(t4, 0.0, -3.0 / 64.0 * t13 * t20 * t665);
        let t670 = 2.0 * t669;
        let t671 = t214 * t86;
        let t677 = t222 * sigma[ip];
        let t682 = t229 * t101;
        let t687 = t236 * t114;
        let t690 = t660 * t552;
        let t693 = 0.2 * t671 * t90 * t218 - 0.08 * t215 * t226 + 0.16 * t677 * t226 - 0.064 * t223 * t233 + 0.096 * t682 * t233 - 0.0192 * t230 * t240 + 0.0256 * t687 * t240 - 0.01024 * t237 * t690;
        let t695 = 2.0 * t212 * t693;
        let t696 = t286 * t86;
        let t702 = t294 * sigma[ip];
        let t707 = t301 * t101;
        let t712 = t308 * t114;
        let t715 = t660 * t630;
        let t718 = 0.006 * t696 * t90 * t290 - 7.2e-05 * t287 * t298 + 0.000144 * t702 * t298 - 1.728e-06 * t295 * t305 + 2.592e-06 * t707 * t305 - 1.5552e-08 * t302 * t312 + 2.0736e-08 * t712 * t312 - 2.48832e-10 * t309 * t715;
        let t719 = t284 * t718;
        let tvsigma0 = rho[ip] * (t670 + t695 + t719);
        vsigma[ip] += tvsigma0;
    }
}
