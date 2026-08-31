//! GGA_C_CHACHIYO kxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_chachiyo.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(
    unused_imports,
    unused_variables,
    non_snake_case,
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::needless_return
)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::piecewise3;
use libxc_rkernel_math::powers::pow_1_3;
use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_chachiyo_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    param_h: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = rho0 + rho1;
        let t10 = pow_1_3(t9);
        let t11 = t8 * t10;
        let t14 = param_cp * t1;
        let t15 = t5 * t5;
        let t17 = t7 * t7;
        let t18 = 1.0 / t15 * t17;
        let t19 = t10 * t10;
        let t20 = t18 * t19;
        let t23 = 1.0 + t3 * t11 / 3.0 + t14 * t20 / 3.0;
        let t24 = rmath::ln(t23);
        let t25 = param_ap * t24;
        let t26 = param_bf * t2;
        let t29 = param_cf * t1;
        let t32 = 1.0 + t26 * t11 / 3.0 + t29 * t20 / 3.0;
        let t33 = rmath::ln(t32);
        let t35 = param_af * t33 - t25;
        let t36 = rho0 - rho1;
        let t37 = 1.0 / t9;
        let t38 = t36 * t37;
        let t39 = 1.0 + t38;
        let t40 = t39 <= zeta_threshold;
        let t41 = pow_1_3(zeta_threshold);
        let t42 = t41 * t41;
        let t43 = pow_1_3(t39);
        let t44 = t43 * t43;
        let t45 = piecewise3(t40, t42, t44);
        let t46 = 1.0 - t38;
        let t47 = t46 <= zeta_threshold;
        let t48 = pow_1_3(t46);
        let t49 = t48 * t48;
        let t50 = piecewise3(t47, t42, t49);
        let t52 = t45 / 2.0 + t50 / 2.0;
        let t53 = t52 * t52;
        let t56 = -2.0 * t53 * t52 + 2.0;
        let t58 = t35 * t56 + t25;
        let t59 = M_CBRTPI;
        let t60 = t2 * t59;
        let t61 = t9 * t9;
        let t63 = 1.0 / t10 / t61;
        let t65 = sigma0 + 2.0 * sigma1 + sigma2;
        let t69 = 1.0 + t60 * t63 * t65 / 48.0;
        let t70 = 1.0 / t58;
        let t71 = param_h * t70;
        let t72 = rmath::pow(t69, t71);
        let tzk0 = t58 * t72;
        zk[ip] += tzk0;
        let t74 = t8 / t19;
        let t78 = t18 / t10;
        let t81 = t3 * t74 / 9.0 + 2.0 / 9.0 * t14 * t78;
        let t83 = 1.0 / t23;
        let t84 = param_ap * t81 * t83;
        let t89 = t26 * t74 / 9.0 + 2.0 / 9.0 * t29 * t78;
        let t91 = 1.0 / t32;
        let t93 = param_af * t89 * t91 - t84;
        let t94 = t93 * t56;
        let t95 = t35 * t53;
        let t96 = 1.0 / t43;
        let t97 = 1.0 / t61;
        let t98 = t36 * t97;
        let t99 = t37 - t98;
        let t102 = piecewise3(t40, 0.0, 2.0 / 3.0 * t96 * t99);
        let t103 = 1.0 / t48;
        let t104 = -t99;
        let t107 = piecewise3(t47, 0.0, 2.0 / 3.0 * t103 * t104);
        let t109 = t102 / 2.0 + t107 / 2.0;
        let t112 = -6.0 * t95 * t109 + t84 + t94;
        let t113 = t9 * t112;
        let t115 = t9 * t58;
        let t116 = t58 * t58;
        let t117 = 1.0 / t116;
        let t118 = param_h * t117;
        let t119 = rmath::ln(t69);
        let t120 = t112 * t119;
        let t122 = t71 * t2;
        let t123 = t61 * t9;
        let t125 = 1.0 / t10 / t123;
        let t126 = t59 * t125;
        let t127 = 1.0 / t69;
        let t128 = t65 * t127;
        let t129 = t126 * t128;
        let t131 = 7.0 / 144.0 * t122 * t129;
        let t132 = -t118 * t120 - t131;
        let t133 = t72 * t132;
        let tvrho0 = t113 * t72 + t115 * t133 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t135 = -t37 - t98;
        let t138 = piecewise3(t40, 0.0, 2.0 / 3.0 * t96 * t135);
        let t139 = -t135;
        let t142 = piecewise3(t47, 0.0, 2.0 / 3.0 * t103 * t139);
        let t144 = t138 / 2.0 + t142 / 2.0;
        let t147 = -6.0 * t95 * t144 + t84 + t94;
        let t148 = t9 * t147;
        let t150 = t147 * t119;
        let t152 = -t118 * t150 - t131;
        let t153 = t72 * t152;
        let tvrho1 = t115 * t153 + t148 * t72 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t156 = 1.0 / t10 / t9;
        let t157 = t156 * t72;
        let t159 = t60 * t127;
        let t160 = t157 * param_h * t159;
        let tvsigma0 = t160 / 48.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = t160 / 24.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t161 = t112 * t72;
        let t163 = t58 * t72;
        let t164 = t163 * t132;
        let t168 = t8 / t19 / t9;
        let t170 = t18 * t156;
        let t173 = -2.0 / 27.0 * t14 * t170 - 2.0 / 27.0 * t3 * t168;
        let t174 = param_ap * t173;
        let t175 = t174 * t83;
        let t176 = t81 * t81;
        let t178 = t23 * t23;
        let t179 = 1.0 / t178;
        let t180 = param_ap * t176 * t179;
        let t184 = -2.0 / 27.0 * t26 * t168 - 2.0 / 27.0 * t29 * t170;
        let t185 = param_af * t184;
        let t187 = t89 * t89;
        let t189 = t32 * t32;
        let t190 = 1.0 / t189;
        let t192 = -param_af * t187 * t190 + t185 * t91 - t175 + t180;
        let t193 = t192 * t56;
        let t194 = t93 * t53;
        let t195 = t194 * t109;
        let t197 = t35 * t52;
        let t198 = t109 * t109;
        let t202 = 1.0 / t43 / t39;
        let t203 = t99 * t99;
        let t206 = 1.0 / t123;
        let t207 = t36 * t206;
        let t209 = -2.0 * t97 + 2.0 * t207;
        let t213 = piecewise3(t40, 0.0, -2.0 / 9.0 * t202 * t203 + 2.0 / 3.0 * t96 * t209);
        let t215 = 1.0 / t48 / t46;
        let t216 = t104 * t104;
        let t219 = -t209;
        let t223 = piecewise3(t47, 0.0, -2.0 / 9.0 * t215 * t216 + 2.0 / 3.0 * t103 * t219);
        let t225 = t213 / 2.0 + t223 / 2.0;
        let t228 = -12.0 * t197 * t198 - 6.0 * t95 * t225 + t175 - t180 + t193 - 12.0 * t195;
        let t229 = t9 * t228;
        let t233 = t132 * t132;
        let t234 = t72 * t233;
        let t237 = 1.0 / t116 / t58;
        let t238 = param_h * t237;
        let t239 = t112 * t112;
        let t240 = t239 * t119;
        let t245 = t112 * t2;
        let t246 = t118 * t245;
        let t247 = t246 * t129;
        let t249 = t61 * t61;
        let t251 = 1.0 / t10 / t249;
        let t252 = t59 * t251;
        let t253 = t252 * t128;
        let t255 = 35.0 / 216.0 * t122 * t253;
        let t256 = t71 * t1;
        let t257 = t59 * t59;
        let t258 = t249 * t61;
        let t260 = 1.0 / t19 / t258;
        let t261 = t257 * t260;
        let t262 = t65 * t65;
        let t263 = t69 * t69;
        let t264 = 1.0 / t263;
        let t265 = t262 * t264;
        let t266 = t261 * t265;
        let t268 = 49.0 / 6912.0 * t256 * t266;
        let t269 = 2.0 * t238 * t240 - t118 * t228 * t119 + 7.0 / 72.0 * t247 + t255 - t268;
        let t270 = t72 * t269;
        let tv2rho20 =
            2.0 * t113 * t133 + t115 * t234 + t115 * t270 + t229 * t72 + 2.0 * t161 + 2.0 * t164;
        v2rho2[ip * 3] += tv2rho20;
        let t272 = t147 * t72;
        let t274 = t194 * t144;
        let t276 = t144 * t109;
        let t279 = t202 * t135;
        let t282 = t96 * t36;
        let t286 = piecewise3(t40, 0.0, -2.0 / 9.0 * t279 * t99 + 4.0 / 3.0 * t282 * t206);
        let t287 = t215 * t139;
        let t290 = t103 * t36;
        let t294 = piecewise3(t47, 0.0, -2.0 / 9.0 * t287 * t104 - 4.0 / 3.0 * t290 * t206);
        let t296 = t286 / 2.0 + t294 / 2.0;
        let t299 =
            -12.0 * t197 * t276 - 6.0 * t95 * t296 + t175 - t180 + t193 - 6.0 * t195 - 6.0 * t274;
        let t300 = t9 * t299;
        let t303 = t163 * t152;
        let t305 = t133 * t152;
        let t310 = t299 * t119;
        let t312 = t147 * t2;
        let t313 = t118 * t312;
        let t314 = t313 * t129;
        let t317 =
            2.0 * t238 * t150 * t112 - t118 * t310 + 7.0 / 144.0 * t314 + 7.0 / 144.0 * t247 + t255
                - t268;
        let t318 = t72 * t317;
        let tv2rho21 = t113 * t153
            + t115 * t305
            + t115 * t318
            + t148 * t133
            + t300 * t72
            + t161
            + t164
            + t272
            + t303;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t323 = t144 * t144;
        let t326 = t135 * t135;
        let t330 = 2.0 * t97 + 2.0 * t207;
        let t334 = piecewise3(t40, 0.0, -2.0 / 9.0 * t202 * t326 + 2.0 / 3.0 * t96 * t330);
        let t335 = t139 * t139;
        let t338 = -t330;
        let t342 = piecewise3(t47, 0.0, -2.0 / 9.0 * t215 * t335 + 2.0 / 3.0 * t103 * t338);
        let t344 = t334 / 2.0 + t342 / 2.0;
        let t347 = -12.0 * t197 * t323 - 6.0 * t95 * t344 + t175 - t180 + t193 - 12.0 * t274;
        let t348 = t9 * t347;
        let t352 = t152 * t152;
        let t353 = t72 * t352;
        let t355 = t147 * t147;
        let t356 = t355 * t119;
        let t359 = t347 * t119;
        let t362 = 2.0 * t238 * t356 - t118 * t359 + 7.0 / 72.0 * t314 + t255 - t268;
        let t363 = t72 * t362;
        let tv2rho22 =
            t115 * t353 + t115 * t363 + 2.0 * t148 * t153 + t348 * t72 + 2.0 * t272 + 2.0 * t303;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t365 = t63 * t72;
        let t367 = t365 * param_h * t159;
        let t368 = t367 / 36.0;
        let t372 = param_h * t2 * t59 * t127;
        let t373 = t157 * t132 * t372;
        let t377 = 1.0 / t19 / t249 * t72;
        let t378 = t377 * param_h;
        let t379 = t1 * t257;
        let t381 = t379 * t264 * t65;
        let t382 = t378 * t381;
        let t383 = 7.0 / 2304.0 * t382;
        let tv2rhosigma0 = -t368 + t373 / 48.0 + t383;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let t384 = t367 / 18.0;
        let t386 = 7.0 / 1152.0 * t382;
        let tv2rhosigma1 = -t384 + t373 / 24.0 + t386;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = tv2rhosigma0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t388 = t157 * t152 * t372;
        let tv2rhosigma3 = -t368 + t388 / 48.0 + t383;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = -t384 + t388 / 24.0 + t386;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let tv2rhosigma5 = tv2rhosigma3;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t392 = 1.0 / t19 / t123;
        let t393 = t392 * t72;
        let t394 = param_h * param_h;
        let t397 = t257 * t264;
        let t398 = t70 * t1 * t397;
        let t401 = t379 * t264;
        let t403 = t393 * t394 * t398 - t393 * param_h * t401;
        let tv2sigma20 = t403 / 768.0;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = t403 / 384.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = tv2sigma20;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = t403 / 192.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = tv2sigma21;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = tv2sigma22;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
        let t404 = t228 * t72;
        let t406 = t161 * t132;
        let t408 = t163 * t233;
        let t410 = t163 * t269;
        let t414 = t8 / t19 / t61;
        let t417 = t18 * t63;
        let t421 = param_ap * (10.0 / 81.0 * t3 * t414 + 8.0 / 81.0 * t14 * t417);
        let t422 = t421 * t83;
        let t423 = t179 * t81;
        let t425 = 3.0 * t174 * t423;
        let t429 = 1.0 / t178 / t23;
        let t431 = 2.0 * param_ap * t176 * t81 * t429;
        let t437 = param_af * (10.0 / 81.0 * t26 * t414 + 8.0 / 81.0 * t29 * t417);
        let t439 = t190 * t89;
        let t445 = 1.0 / t189 / t32;
        let t448 = 2.0 * param_af * t187 * t89 * t445 - 3.0 * t185 * t439 + t437 * t91 - t422
            + t425
            - t431;
        let t449 = t448 * t56;
        let t450 = t192 * t53;
        let t451 = t450 * t109;
        let t453 = t93 * t52;
        let t454 = t453 * t198;
        let t456 = t194 * t225;
        let t458 = t198 * t109;
        let t461 = t109 * t225;
        let t464 = t39 * t39;
        let t466 = 1.0 / t43 / t464;
        let t467 = t203 * t99;
        let t470 = t202 * t99;
        let t473 = 1.0 / t249;
        let t474 = t36 * t473;
        let t476 = 6.0 * t206 - 6.0 * t474;
        let t480 = piecewise3(
            t40,
            0.0,
            8.0 / 27.0 * t466 * t467 - 2.0 / 3.0 * t470 * t209 + 2.0 / 3.0 * t96 * t476,
        );
        let t481 = t46 * t46;
        let t483 = 1.0 / t48 / t481;
        let t484 = t216 * t104;
        let t487 = t215 * t104;
        let t490 = -t476;
        let t494 = piecewise3(
            t47,
            0.0,
            8.0 / 27.0 * t483 * t484 - 2.0 / 3.0 * t487 * t219 + 2.0 / 3.0 * t103 * t490,
        );
        let t496 = t480 / 2.0 + t494 / 2.0;
        let t499 =
            -36.0 * t197 * t461 - 12.0 * t35 * t458 - 6.0 * t95 * t496 + t422 - t425 + t431 + t449
                - 18.0 * t451
                - 36.0 * t454
                - 18.0 * t456;
        let t500 = t9 * t499;
        let t508 = t233 * t132;
        let t509 = t72 * t508;
        let t511 = t133 * t269;
        let t514 = t116 * t116;
        let t516 = param_h / t514;
        let t517 = t239 * t112;
        let t521 = t120 * t228;
        let t525 = t238 * t239 * t2;
        let t526 = t525 * t129;
        let t531 = t118 * t228 * t2;
        let t532 = t531 * t129;
        let t534 = t246 * t253;
        let t537 = t118 * t112 * t1;
        let t538 = t537 * t266;
        let t540 = t249 * t9;
        let t544 = t59 / t10 / t540 * t128;
        let t546 = 455.0 / 648.0 * t122 * t544;
        let t547 = t249 * t123;
        let t551 = t257 / t19 / t547 * t265;
        let t553 = 245.0 / 3456.0 * t256 * t551;
        let t554 = t71 * M_PI;
        let t555 = t249 * t249;
        let t556 = t555 * t61;
        let t557 = 1.0 / t556;
        let t558 = t262 * t65;
        let t561 = 1.0 / t263 / t69;
        let t564 = 343.0 / 165888.0 * t554 * t557 * t558 * t561;
        let t565 =
            -6.0 * t516 * t517 * t119 + 6.0 * t238 * t521 - 7.0 / 24.0 * t526 - t118 * t499 * t119
                + 7.0 / 48.0 * t532
                - 35.0 / 72.0 * t534
                + 49.0 / 2304.0 * t538
                - t546
                + t553
                - t564;
        let t566 = t72 * t565;
        let tv3rho30 = 3.0 * t113 * t234
            + 3.0 * t113 * t270
            + t115 * t509
            + 3.0 * t115 * t511
            + t115 * t566
            + 3.0 * t229 * t133
            + t500 * t72
            + 3.0 * t404
            + 6.0 * t406
            + 3.0 * t408
            + 3.0 * t410;
        v3rho3[ip * 4] += tv3rho30;
        let t571 = t132 * t152;
        let t573 = 2.0 * tzk0 * t571;
        let t580 = t310 * t112;
        let t583 = t238 * t312;
        let t584 = t128 * t112;
        let t585 = t126 * t584;
        let t587 = 7.0 / 36.0 * t583 * t585;
        let t594 = t450 * t144;
        let t597 = 24.0 * t453 * t276;
        let t599 = 12.0 * t194 * t296;
        let t600 = t35 * t198;
        let t603 = t296 * t109;
        let t606 = t144 * t225;
        let t609 = t466 * t135;
        let t612 = t202 * t36;
        let t623 = piecewise3(
            t40,
            0.0,
            8.0 / 27.0 * t609 * t203 - 8.0 / 9.0 * t612 * t206 * t99 - 2.0 / 9.0 * t279 * t209
                + 4.0 / 3.0 * t96 * t206
                - 4.0 * t282 * t473,
        );
        let t624 = t483 * t139;
        let t627 = t215 * t36;
        let t638 = piecewise3(
            t47,
            0.0,
            8.0 / 27.0 * t624 * t216 + 8.0 / 9.0 * t627 * t206 * t104
                - 2.0 / 9.0 * t287 * t219
                - 4.0 / 3.0 * t103 * t206
                + 4.0 * t290 * t473,
        );
        let t640 = t623 / 2.0 + t638 / 2.0;
        let t643 = -12.0 * t600 * t144 - 24.0 * t197 * t603 - 12.0 * t197 * t606 - 6.0 * t95 * t640
            + t422
            - t425
            + t431
            + t449
            - 12.0 * t451
            - 12.0 * t454
            - 6.0 * t456
            - 6.0 * t594
            - t597
            - t599;
        let t644 = t643 * t119;
        let t646 = t299 * t2;
        let t647 = t118 * t646;
        let t649 = 7.0 / 72.0 * t647 * t129;
        let t650 = t313 * t253;
        let t652 = t147 * t1;
        let t653 = t118 * t652;
        let t654 = t653 * t266;
        let t660 = -6.0 * t516 * t150 * t239 + 4.0 * t238 * t580 - t587 + 2.0 * t238 * t150 * t228
            - t118 * t644
            + t649
            - 35.0 / 216.0 * t650
            + 49.0 / 6912.0 * t654
            - 7.0 / 72.0 * t526
            + 7.0 / 144.0 * t532
            - 35.0 / 108.0 * t534
            + 49.0 / 3456.0 * t538
            - t546
            + t553
            - t564;
        let t661 = t72 * t660;
        let t663 = t234 * t152;
        let t668 = t270 * t152;
        let t670 = t133 * t317;
        let t673 = t299 * t72;
        let t674 = 2.0 * t673;
        let t677 = 2.0 * t272 * t132;
        let t678 = t9 * t643;
        let t681 = 2.0 * t161 * t152;
        let t683 = 2.0 * t163 * t317;
        let tv3rho31 = 2.0 * t113 * t305
            + 2.0 * t113 * t318
            + t115 * t661
            + t115 * t663
            + t115 * t668
            + 2.0 * t115 * t670
            + 2.0 * t300 * t133
            + t148 * t234
            + t148 * t270
            + t229 * t153
            + t678 * t72
            + t404
            + 2.0 * t406
            + t408
            + t410
            + t573
            + t674
            + t677
            + t681
            + t683;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t684 = t347 * t72;
        let t687 = t453 * t323;
        let t689 = t35 * t109;
        let t692 = t144 * t296;
        let t695 = t194 * t344;
        let t697 = t344 * t109;
        let t700 = t466 * t326;
        let t705 = t202 * t330;
        let t710 = -2.0 * t206 - 6.0 * t474;
        let t714 = piecewise3(
            t40,
            0.0,
            8.0 / 27.0 * t700 * t99 - 8.0 / 9.0 * t279 * t207 - 2.0 / 9.0 * t705 * t99
                + 2.0 / 3.0 * t96 * t710,
        );
        let t715 = t483 * t335;
        let t720 = t215 * t338;
        let t723 = -t710;
        let t727 = piecewise3(
            t47,
            0.0,
            8.0 / 27.0 * t715 * t104 + 8.0 / 9.0 * t287 * t207 - 2.0 / 9.0 * t720 * t104
                + 2.0 / 3.0 * t103 * t723,
        );
        let t729 = t714 / 2.0 + t727 / 2.0;
        let t732 = -24.0 * t197 * t692 - 12.0 * t197 * t697 - 12.0 * t689 * t323 - 6.0 * t95 * t729
            + t422
            - t425
            + t431
            + t449
            - 6.0 * t451
            - 12.0 * t594
            - t597
            - t599
            - 12.0 * t687
            - 6.0 * t695;
        let t733 = t9 * t732;
        let t736 = t272 * t152;
        let t744 = t163 * t352;
        let t746 = t133 * t352;
        let t748 = t153 * t317;
        let t751 = t163 * t362;
        let t753 = t133 * t362;
        let t761 = t355 * t2;
        let t762 = t238 * t761;
        let t763 = t762 * t129;
        let t765 = t359 * t112;
        let t768 = t732 * t119;
        let t770 = t347 * t2;
        let t771 = t118 * t770;
        let t772 = t771 * t129;
        let t778 = -6.0 * t516 * t356 * t112 + 4.0 * t238 * t150 * t299 - 7.0 / 72.0 * t763
            + 2.0 * t238 * t765
            - t118 * t768
            + 7.0 / 144.0 * t772
            - t587
            + t649
            - 35.0 / 108.0 * t650
            + 49.0 / 3456.0 * t654
            - 35.0 / 216.0 * t534
            - t546
            + t553
            + 49.0 / 6912.0 * t538
            - t564;
        let t779 = t72 * t778;
        let tv3rho32 = t113 * t353
            + t113 * t363
            + t115 * t746
            + 2.0 * t115 * t748
            + t115 * t753
            + t115 * t779
            + t348 * t133
            + 2.0 * t148 * t305
            + 2.0 * t148 * t318
            + 2.0 * t300 * t153
            + t733 * t72
            + t573
            + t674
            + t677
            + t681
            + t683
            + t684
            + 2.0 * t736
            + t744
            + t751;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t788 = t323 * t144;
        let t791 = t144 * t344;
        let t794 = t326 * t135;
        let t800 = -6.0 * t206 - 6.0 * t474;
        let t804 = piecewise3(
            t40,
            0.0,
            8.0 / 27.0 * t466 * t794 - 2.0 / 3.0 * t279 * t330 + 2.0 / 3.0 * t96 * t800,
        );
        let t805 = t335 * t139;
        let t810 = -t800;
        let t814 = piecewise3(
            t47,
            0.0,
            8.0 / 27.0 * t483 * t805 - 2.0 / 3.0 * t287 * t338 + 2.0 / 3.0 * t103 * t810,
        );
        let t816 = t804 / 2.0 + t814 / 2.0;
        let t819 =
            -36.0 * t197 * t791 - 12.0 * t35 * t788 - 6.0 * t95 * t816 + t422 - t425 + t431 + t449
                - 18.0 * t594
                - 36.0 * t687
                - 18.0 * t695;
        let t820 = t9 * t819;
        let t828 = t352 * t152;
        let t829 = t72 * t828;
        let t831 = t153 * t362;
        let t834 = t355 * t147;
        let t835 = t834 * t119;
        let t842 = t819 * t119;
        let t847 = -6.0 * t516 * t835 + 6.0 * t238 * t150 * t347 - 7.0 / 24.0 * t763 - t118 * t842
            + 7.0 / 48.0 * t772
            - 35.0 / 72.0 * t650
            + 49.0 / 2304.0 * t654
            - t546
            + t553
            - t564;
        let t848 = t72 * t847;
        let tv3rho33 = t115 * t829
            + 3.0 * t115 * t831
            + t115 * t848
            + 3.0 * t148 * t353
            + 3.0 * t148 * t363
            + 3.0 * t348 * t153
            + t820 * t72
            + 3.0 * t684
            + 6.0 * t736
            + 3.0 * t744
            + 3.0 * t751;
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t850 = t125 * t72;
        let t852 = t850 * param_h * t159;
        let t853 = 7.0 / 108.0 * t852;
        let t855 = t365 * t132 * t372;
        let t856 = t855 / 18.0;
        let t859 = 1.0 / t19 / t540 * t72;
        let t860 = t859 * param_h;
        let t861 = t860 * t381;
        let t862 = 7.0 / 384.0 * t861;
        let t864 = t157 * t233 * t372;
        let t867 = t157 * t269 * t372;
        let t869 = t132 * param_h;
        let t871 = t377 * t869 * t381;
        let t872 = 7.0 / 1152.0 * t871;
        let t874 = 1.0 / t555 * t72;
        let t875 = t874 * param_h;
        let t876 = M_PI * t561;
        let t877 = t876 * t262;
        let t878 = t875 * t877;
        let t879 = 49.0 / 55296.0 * t878;
        let tv3rho2sigma0 = t853 - t856 - t862 + t864 / 48.0 + t867 / 48.0 + t872 + t879;
        v3rho2sigma[ip * 9] += tv3rho2sigma0;
        let t880 = 7.0 / 54.0 * t852;
        let t882 = 7.0 / 192.0 * t861;
        let t886 = 49.0 / 27648.0 * t878;
        let tv3rho2sigma1 =
            t880 - t855 / 9.0 - t882 + t864 / 24.0 + t867 / 24.0 + 7.0 / 576.0 * t871 + t886;
        v3rho2sigma[ip * 9 + 1] += tv3rho2sigma1;
        let tv3rho2sigma2 = tv3rho2sigma0;
        v3rho2sigma[ip * 9 + 2] += tv3rho2sigma2;
        let t889 = t365 * t152 * t372;
        let t892 = t157 * t571 * t372;
        let t895 = t157 * t317 * t372;
        let t897 = t152 * param_h;
        let t899 = t377 * t897 * t381;
        let tv3rho2sigma3 = t853 - t855 / 36.0 - t862 - t889 / 36.0
            + t892 / 48.0
            + t895 / 48.0
            + 7.0 / 2304.0 * t899
            + 7.0 / 2304.0 * t871
            + t879;
        v3rho2sigma[ip * 9 + 3] += tv3rho2sigma3;
        let t902 = t889 / 18.0;
        let t905 = 7.0 / 1152.0 * t899;
        let tv3rho2sigma4 =
            t880 - t856 - t882 - t902 + t892 / 24.0 + t895 / 24.0 + t905 + t872 + t886;
        v3rho2sigma[ip * 9 + 4] += tv3rho2sigma4;
        let tv3rho2sigma5 = tv3rho2sigma3;
        v3rho2sigma[ip * 9 + 5] += tv3rho2sigma5;
        let t907 = t157 * t352 * t372;
        let t910 = t157 * t362 * t372;
        let tv3rho2sigma6 = t853 - t902 - t862 + t907 / 48.0 + t910 / 48.0 + t905 + t879;
        v3rho2sigma[ip * 9 + 6] += tv3rho2sigma6;
        let tv3rho2sigma7 =
            t880 - t889 / 9.0 - t882 + t907 / 24.0 + t910 / 24.0 + 7.0 / 576.0 * t899 + t886;
        v3rho2sigma[ip * 9 + 7] += tv3rho2sigma7;
        let tv3rho2sigma8 = tv3rho2sigma6;
        v3rho2sigma[ip * 9 + 8] += tv3rho2sigma8;
        let t917 = t377 * t394 * t398;
        let t918 = 11.0 / 2304.0 * t917;
        let t919 = t132 * t394;
        let t920 = t393 * t919;
        let t921 = t920 * t398;
        let t923 = t394 * t117;
        let t924 = t393 * t923;
        let t926 = t379 * t264 * t112;
        let t927 = t924 * t926;
        let t930 = 1.0 / t547 * t72;
        let t931 = t930 * t394;
        let t932 = t70 * M_PI;
        let t933 = t561 * t65;
        let t934 = t932 * t933;
        let t935 = t931 * t934;
        let t936 = 7.0 / 18432.0 * t935;
        let t937 = t378 * t401;
        let t938 = 11.0 / 2304.0 * t937;
        let t940 = param_h * t1;
        let t941 = t940 * t397;
        let t942 = t393 * t132 * t941;
        let t945 = t876 * t65;
        let t946 = t930 * param_h * t945;
        let t947 = 7.0 / 18432.0 * t946;
        let tv3rhosigma20 = -t918 + t921 / 768.0 - t927 / 768.0 + t936 + t938 - t942 / 768.0 - t947;
        v3rhosigma2[ip * 12] += tv3rhosigma20;
        let t948 = 11.0 / 1152.0 * t917;
        let t951 = 7.0 / 9216.0 * t935;
        let t952 = 11.0 / 1152.0 * t937;
        let t954 = 7.0 / 9216.0 * t946;
        let tv3rhosigma21 = -t948 + t921 / 384.0 - t927 / 384.0 + t951 + t952 - t942 / 384.0 - t954;
        v3rhosigma2[ip * 12 + 1] += tv3rhosigma21;
        let tv3rhosigma22 = tv3rhosigma20;
        v3rhosigma2[ip * 12 + 2] += tv3rhosigma22;
        let t955 = 11.0 / 576.0 * t917;
        let t958 = 7.0 / 4608.0 * t935;
        let t959 = 11.0 / 576.0 * t937;
        let t961 = 7.0 / 4608.0 * t946;
        let tv3rhosigma23 = -t955 + t921 / 192.0 - t927 / 192.0 + t958 + t959 - t942 / 192.0 - t961;
        v3rhosigma2[ip * 12 + 3] += tv3rhosigma23;
        let tv3rhosigma24 = tv3rhosigma21;
        v3rhosigma2[ip * 12 + 4] += tv3rhosigma24;
        let tv3rhosigma25 = tv3rhosigma22;
        v3rhosigma2[ip * 12 + 5] += tv3rhosigma25;
        let t962 = t152 * t394;
        let t963 = t393 * t962;
        let t964 = t963 * t398;
        let t966 = t264 * t147;
        let t967 = t379 * t966;
        let t968 = t924 * t967;
        let t971 = t393 * t152 * t941;
        let tv3rhosigma26 = -t918 + t964 / 768.0 - t968 / 768.0 + t936 + t938 - t971 / 768.0 - t947;
        v3rhosigma2[ip * 12 + 6] += tv3rhosigma26;
        let tv3rhosigma27 = -t948 + t964 / 384.0 - t968 / 384.0 + t951 + t952 - t971 / 384.0 - t954;
        v3rhosigma2[ip * 12 + 7] += tv3rhosigma27;
        let tv3rhosigma28 = tv3rhosigma26;
        v3rhosigma2[ip * 12 + 8] += tv3rhosigma28;
        let tv3rhosigma29 = -t955 + t964 / 192.0 - t968 / 192.0 + t958 + t959 - t971 / 192.0 - t961;
        v3rhosigma2[ip * 12 + 9] += tv3rhosigma29;
        let tv3rhosigma210 = tv3rhosigma27;
        v3rhosigma2[ip * 12 + 10] += tv3rhosigma210;
        let tv3rhosigma211 = tv3rhosigma28;
        v3rhosigma2[ip * 12 + 11] += tv3rhosigma211;
        let t979 = 1.0 / t258;
        let t980 = t979 * t72;
        let t981 = t394 * param_h;
        let t982 = t980 * t981;
        let t983 = t117 * M_PI;
        let t984 = t983 * t561;
        let t985 = t982 * t984;
        let t987 = t980 * t394;
        let t988 = t932 * t561;
        let t989 = t987 * t988;
        let t991 = param_h * M_PI;
        let t992 = t991 * t561;
        let t993 = t980 * t992;
        let tv3sigma30 = t985 / 12288.0 - t989 / 4096.0 + t993 / 6144.0;
        v3sigma3[ip * 10] += tv3sigma30;
        let tv3sigma31 = t985 / 6144.0 - t989 / 2048.0 + t993 / 3072.0;
        v3sigma3[ip * 10 + 1] += tv3sigma31;
        let tv3sigma32 = tv3sigma30;
        v3sigma3[ip * 10 + 2] += tv3sigma32;
        let tv3sigma33 = t985 / 3072.0 - t989 / 1024.0 + t993 / 1536.0;
        v3sigma3[ip * 10 + 3] += tv3sigma33;
        let tv3sigma34 = tv3sigma31;
        v3sigma3[ip * 10 + 4] += tv3sigma34;
        let tv3sigma35 = tv3sigma32;
        v3sigma3[ip * 10 + 5] += tv3sigma35;
        let tv3sigma36 = t985 / 1536.0 - t989 / 512.0 + t993 / 768.0;
        v3sigma3[ip * 10 + 6] += tv3sigma36;
        let tv3sigma37 = tv3sigma33;
        v3sigma3[ip * 10 + 7] += tv3sigma37;
        let tv3sigma38 = tv3sigma34;
        v3sigma3[ip * 10 + 8] += tv3sigma38;
        let tv3sigma39 = tv3sigma35;
        v3sigma3[ip * 10 + 9] += tv3sigma39;
    }
}
