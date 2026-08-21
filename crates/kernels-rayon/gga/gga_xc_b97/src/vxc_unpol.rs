//! GGA_XC_B97 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_b97.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_b97_vxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = piecewise3(t3, zeta_threshold, 1.0);
        let t6 = pow_1_3(zeta_threshold);
        let t8 = piecewise3(t3, 1.0 / t6, 1.0);
        let t9 = t8 * t8;
        let t10 = t9 * t8;
        let t14 = rho[ip] / t10 / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = M_CBRTPI;
        let t19 = M_CBRT2;
        let t20 = t19 * t19;
        let t21 = t15 / t16 * t20;
        let t23 = t6 * zeta_threshold;
        let t25 = piecewise3(2.0 <= zeta_threshold, t23, 2.0 * t19);
        let t26 = pow_1_3(rho[ip]);
        let t28 = 1.0 / t8;
        let t32 = piecewise3(t14, 0.0, -3.0 / 16.0 * t21 * t25 * t26 * t28);
        let t33 = 0.0 <= dens_threshold;
        let t35 = piecewise3(0.0 <= zeta_threshold, t23, 0.0);
        let t40 = piecewise3(t33, 0.0, -3.0 / 16.0 * t21 * t35 * t26 * t28);
        let t44 = piecewise3(t4, 0.0, t5 * (t32 + t40) / 2.0);
        let t46 = param_c_x_1;
        let t47 = t46 * sigma[ip];
        let t48 = rho[ip] * rho[ip];
        let t49 = t26 * t26;
        let t51 = 1.0 / t49 / t48;
        let t52 = t20 * t51;
        let t54 = sigma[ip] * t20 * t51;
        let t56 = 1.0 + 0.004 * t54;
        let t57 = 1.0 / t56;
        let t61 = param_c_x_2;
        let t62 = sigma[ip] * sigma[ip];
        let t63 = t61 * t62;
        let t64 = t48 * t48;
        let t65 = t64 * rho[ip];
        let t67 = 1.0 / t26 / t65;
        let t68 = t19 * t67;
        let t69 = t56 * t56;
        let t70 = 1.0 / t69;
        let t71 = t68 * t70;
        let t74 = param_c_x_3;
        let t75 = t62 * sigma[ip];
        let t76 = t74 * t75;
        let t77 = t64 * t64;
        let t78 = 1.0 / t77;
        let t79 = t69 * t56;
        let t80 = 1.0 / t79;
        let t81 = t78 * t80;
        let t84 = param_c_x_4;
        let t85 = t62 * t62;
        let t86 = t84 * t85;
        let t87 = t77 * t48;
        let t89 = 1.0 / t49 / t87;
        let t90 = t20 * t89;
        let t91 = t69 * t69;
        let t92 = 1.0 / t91;
        let t93 = t90 * t92;
        let t96 = param_c_x_0 + 0.004 * t47 * t52 * t57 + 3.2e-05 * t63 * t71 + 2.56e-07 * t76 * t81 + 1.024e-09 * t86 * t93;
        let t98 = 2.0 * t44 * t96;
        let t99 = 1.0 / M_PI;
        let t100 = pow_1_3(t99);
        let t101 = t15 * t100;
        let t102 = M_CBRT4;
        let t103 = t102 * t102;
        let t104 = t101 * t103;
        let t105 = 1.0 / t26;
        let t108 = t104 * t105 * t19 * t8;
        let t110 = 1.0 + 0.053425 * t108;
        let t111 = rmath::sqrt(t108);
        let t114 = pow_3_2(t108);
        let t116 = t15 * t15;
        let t117 = t100 * t100;
        let t118 = t116 * t117;
        let t119 = t118 * t102;
        let t120 = 1.0 / t49;
        let t123 = t119 * t120 * t20 * t9;
        let t125 = 3.79785 * t111 + 0.8969 * t108 + 0.204775 * t114 + 0.123235 * t123;
        let t128 = 1.0 + 16.081824322151103 / t125;
        let t129 = rmath::ln(t128);
        let t131 = 0.062182 * t110 * t129;
        let t135 = 1.0 / (2.0 * t19 - 2.0);
        let t136 = (t25 + t35 - 2.0) * t135;
        let t138 = 1.0 + 0.05137 * t108;
        let t143 = 7.05945 * t111 + 1.549425 * t108 + 0.420775 * t114 + 0.1562925 * t123;
        let t146 = 1.0 + 32.1646831778707 / t143;
        let t147 = rmath::ln(t146);
        let t151 = 1.0 + 0.0278125 * t108;
        let t156 = 5.1785 * t111 + 0.905775 * t108 + 0.1100325 * t114 + 0.1241775 * t123;
        let t159 = 1.0 + 29.608574643216677 / t156;
        let t160 = rmath::ln(t159);
        let t161 = t151 * t160;
        let t170 = piecewise3(t4, 0.0, t5 * (-t131 + t136 * (-0.03109 * t138 * t147 + t131 - 0.019751789702565206 * t161) + 0.019751789702565206 * t136 * t161) / 2.0);
        let t172 = param_c_ss_1;
        let t173 = t172 * sigma[ip];
        let t175 = 1.0 + 0.2 * t54;
        let t176 = 1.0 / t175;
        let t180 = param_c_ss_2;
        let t181 = t180 * t62;
        let t182 = t175 * t175;
        let t183 = 1.0 / t182;
        let t184 = t68 * t183;
        let t187 = param_c_ss_3;
        let t188 = t187 * t75;
        let t189 = t182 * t175;
        let t190 = 1.0 / t189;
        let t191 = t78 * t190;
        let t194 = param_c_ss_4;
        let t195 = t194 * t85;
        let t196 = t182 * t182;
        let t197 = 1.0 / t196;
        let t198 = t90 * t197;
        let t201 = param_c_ss_0 + 0.2 * t173 * t52 * t176 + 0.08 * t181 * t184 + 0.032 * t188 * t191 + 0.0064 * t195 * t198;
        let t203 = 2.0 * t170 * t201;
        let t205 = t101 * t103 * t105;
        let t207 = 1.0 + 0.053425 * t205;
        let t208 = rmath::sqrt(t205);
        let t211 = pow_3_2(t205);
        let t214 = t118 * t102 * t120;
        let t216 = 3.79785 * t208 + 0.8969 * t205 + 0.204775 * t211 + 0.123235 * t214;
        let t219 = 1.0 + 16.081824322151103 / t216;
        let t220 = rmath::ln(t219);
        let t223 = piecewise3(t3, t23, 1.0);
        let t226 = (2.0 * t223 - 2.0) * t135;
        let t228 = 1.0 + 0.0278125 * t205;
        let t233 = 5.1785 * t208 + 0.905775 * t205 + 0.1100325 * t211 + 0.1241775 * t214;
        let t236 = 1.0 + 29.608574643216677 / t233;
        let t237 = rmath::ln(t236);
        let t242 = -0.062182 * t207 * t220 + 0.019751789702565206 * t226 * t228 * t237 - 2.0 * t170;
        let t244 = param_c_ab_1;
        let t245 = t244 * sigma[ip];
        let t247 = 1.0 + 0.006 * t54;
        let t248 = 1.0 / t247;
        let t252 = param_c_ab_2;
        let t253 = t252 * t62;
        let t254 = t247 * t247;
        let t255 = 1.0 / t254;
        let t256 = t68 * t255;
        let t259 = param_c_ab_3;
        let t260 = t259 * t75;
        let t261 = t254 * t247;
        let t262 = 1.0 / t261;
        let t263 = t78 * t262;
        let t266 = param_c_ab_4;
        let t267 = t266 * t85;
        let t268 = t254 * t254;
        let t269 = 1.0 / t268;
        let t270 = t90 * t269;
        let t273 = param_c_ab_0 + 0.006 * t245 * t52 * t248 + 7.2e-05 * t253 * t256 + 8.64e-07 * t260 * t263 + 5.184e-09 * t267 * t270;
        let t274 = t242 * t273;
        let tzk0 = t98 + t203 + t274;
        zk[ip] += tzk0;
        let t279 = piecewise3(t14, 0.0, -t21 * t25 * t120 * t28 / 16.0);
        let t284 = piecewise3(t33, 0.0, -t21 * t35 * t120 * t28 / 16.0);
        let t288 = piecewise3(t4, 0.0, t5 * (t279 + t284) / 2.0);
        let t289 = t288 * t96;
        let t291 = t48 * rho[ip];
        let t293 = 1.0 / t49 / t291;
        let t294 = t20 * t293;
        let t298 = t46 * t62;
        let t299 = t64 * t48;
        let t301 = 1.0 / t26 / t299;
        let t302 = t19 * t301;
        let t303 = t302 * t70;
        let t308 = t61 * t75;
        let t309 = t77 * rho[ip];
        let t310 = 1.0 / t309;
        let t311 = t310 * t80;
        let t316 = t74 * t85;
        let t317 = t77 * t291;
        let t319 = 1.0 / t49 / t317;
        let t321 = t319 * t92 * t20;
        let t326 = t85 * sigma[ip];
        let t327 = t84 * t326;
        let t328 = t77 * t299;
        let t331 = t19 / t26 / t328;
        let t333 = 1.0 / t91 / t56;
        let t334 = t331 * t333;
        let t337 = -0.010666666666666666 * t47 * t294 * t57 + 8.533333333333334e-05 * t298 * t303 - 0.00017066666666666668 * t63 * t303 + 1.3653333333333333e-06 * t308 * t311 - 2.048e-06 * t76 * t311 + 8.192e-09 * t316 * t321 - 1.0922666666666667e-08 * t86 * t321 + 8.738133333333333e-11 * t327 * t334;
        let t338 = t44 * t337;
        let t341 = 1.0 / t26 / rho[ip];
        let t342 = t341 * t19;
        let t343 = t8 * t129;
        let t346 = 0.0011073577833333333 * t104 * t342 * t343;
        let t347 = t125 * t125;
        let t348 = 1.0 / t347;
        let t349 = t110 * t348;
        let t352 = 1.0 / t111 * t15 * t100;
        let t353 = t103 * t341;
        let t354 = t19 * t8;
        let t355 = t353 * t354;
        let t356 = t352 * t355;
        let t358 = t342 * t8;
        let t359 = t104 * t358;
        let t361 = rmath::sqrt(t108);
        let t363 = t361 * t15 * t100;
        let t364 = t363 * t355;
        let t367 = 1.0 / t49 / rho[ip];
        let t370 = t119 * t367 * t20 * t9;
        let t372 = -0.632975 * t356 - 0.29896666666666666 * t359 - 0.1023875 * t364 - 0.08215666666666667 * t370;
        let t373 = 1.0 / t128;
        let t374 = t372 * t373;
        let t376 = 1.0 * t349 * t374;
        let t377 = t8 * t147;
        let t381 = t143 * t143;
        let t382 = 1.0 / t381;
        let t383 = t138 * t382;
        let t388 = -1.176575 * t356 - 0.516475 * t359 - 0.2103875 * t364 - 0.104195 * t370;
        let t389 = 1.0 / t146;
        let t390 = t388 * t389;
        let t393 = t8 * t160;
        let t397 = t156 * t156;
        let t398 = 1.0 / t397;
        let t399 = t151 * t398;
        let t404 = -0.8630833333333333 * t356 - 0.301925 * t359 - 0.05501625 * t364 - 0.082785 * t370;
        let t405 = 1.0 / t159;
        let t406 = t404 * t405;
        let t411 = t136 * t101;
        let t412 = t354 * t160;
        let t416 = t136 * t151;
        let t418 = t398 * t404 * t405;
        let t424 = piecewise3(t4, 0.0, t5 * (t346 + t376 + t136 * (0.0005323644333333333 * t104 * t342 * t377 + 1.0 * t383 * t390 - t346 - t376 + 0.0001831155503675316 * t104 * t342 * t393 + 0.5848223397455204 * t399 * t406) - 0.0001831155503675316 * t411 * t353 * t412 - 0.5848223397455204 * t416 * t418) / 2.0);
        let t425 = t424 * t201;
        let t430 = t172 * t62;
        let t431 = t302 * t183;
        let t436 = t180 * t75;
        let t437 = t310 * t190;
        let t442 = t187 * t85;
        let t444 = t319 * t197 * t20;
        let t449 = t194 * t326;
        let t451 = 1.0 / t196 / t175;
        let t452 = t331 * t451;
        let t455 = -0.5333333333333333 * t173 * t294 * t176 + 0.21333333333333335 * t430 * t431 - 0.4266666666666667 * t181 * t431 + 0.17066666666666666 * t436 * t437 - 0.256 * t188 * t437 + 0.0512 * t442 * t444 - 0.06826666666666667 * t195 * t444 + 0.027306666666666667 * t449 * t452;
        let t456 = t170 * t455;
        let t461 = t216 * t216;
        let t462 = 1.0 / t461;
        let t463 = t207 * t462;
        let t465 = 1.0 / t208 * t15;
        let t466 = t100 * t103;
        let t467 = t466 * t341;
        let t468 = t465 * t467;
        let t470 = t101 * t353;
        let t472 = rmath::sqrt(t205);
        let t473 = t472 * t15;
        let t474 = t473 * t467;
        let t477 = t118 * t102 * t367;
        let t479 = -0.632975 * t468 - 0.29896666666666666 * t470 - 0.1023875 * t474 - 0.08215666666666667 * t477;
        let t480 = 1.0 / t219;
        let t481 = t479 * t480;
        let t484 = t226 * t15;
        let t489 = t226 * t228;
        let t490 = t233 * t233;
        let t491 = 1.0 / t490;
        let t496 = -0.8630833333333333 * t468 - 0.301925 * t470 - 0.05501625 * t474 - 0.082785 * t477;
        let t498 = 1.0 / t236;
        let t499 = t491 * t496 * t498;
        let t503 = 0.0011073577833333333 * t101 * t353 * t220 + 1.0 * t463 * t481 - 0.0001831155503675316 * t484 * t466 * t341 * t237 - 0.5848223397455204 * t489 * t499 - 2.0 * t424;
        let t504 = t503 * t273;
        let t508 = t244 * t62;
        let t509 = t302 * t255;
        let t514 = t252 * t75;
        let t515 = t310 * t262;
        let t520 = t259 * t85;
        let t522 = t319 * t269 * t20;
        let t527 = t266 * t326;
        let t529 = 1.0 / t268 / t247;
        let t530 = t331 * t529;
        let t533 = -0.016 * t245 * t294 * t248 + 0.000192 * t508 * t509 - 0.000384 * t253 * t509 + 4.608e-06 * t514 * t515 - 6.912e-06 * t260 * t515 + 4.1472e-08 * t520 * t522 - 5.5296e-08 * t267 * t522 + 6.63552e-10 * t527 * t530;
        let t534 = t242 * t533;
        let tvrho0 = t98 + t203 + t274 + rho[ip] * (2.0 * t289 + 2.0 * t338 + 2.0 * t425 + 2.0 * t456 + t504 + t534);
        vrho[ip] += tvrho0;
        let t537 = t46 * t20;
        let t543 = t61 * sigma[ip];
        let t548 = t74 * t62;
        let t553 = t84 * t75;
        let t556 = t77 * t65;
        let t559 = t19 / t26 / t556;
        let t560 = t559 * t333;
        let t563 = 0.004 * t537 * t51 * t57 - 3.2e-05 * t47 * t71 + 6.4e-05 * t543 * t71 - 5.12e-07 * t63 * t81 + 7.68e-07 * t548 * t81 - 3.072e-09 * t76 * t93 + 4.096e-09 * t553 * t93 - 3.2768e-11 * t86 * t560;
        let t565 = 2.0 * t44 * t563;
        let t566 = t172 * t20;
        let t572 = t180 * sigma[ip];
        let t577 = t187 * t62;
        let t582 = t194 * t75;
        let t585 = t559 * t451;
        let t588 = 0.2 * t566 * t51 * t176 - 0.08 * t173 * t184 + 0.16 * t572 * t184 - 0.064 * t181 * t191 + 0.096 * t577 * t191 - 0.0192 * t188 * t198 + 0.0256 * t582 * t198 - 0.01024 * t195 * t585;
        let t590 = 2.0 * t170 * t588;
        let t591 = t244 * t20;
        let t597 = t252 * sigma[ip];
        let t602 = t259 * t62;
        let t607 = t266 * t75;
        let t610 = t559 * t529;
        let t613 = 0.006 * t591 * t51 * t248 - 7.2e-05 * t245 * t256 + 0.000144 * t597 * t256 - 1.728e-06 * t253 * t263 + 2.592e-06 * t602 * t263 - 1.5552e-08 * t260 * t270 + 2.0736e-08 * t607 * t270 - 2.48832e-10 * t267 * t610;
        let t614 = t242 * t613;
        let tvsigma0 = rho[ip] * (t565 + t590 + t614);
        vsigma[ip] += tvsigma0;
    }
}
