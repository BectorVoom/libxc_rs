//! MGGA_C_M08 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_m08.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_m08_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_m08_a_0: f64,
    param_m08_a_1: f64,
    param_m08_a_2: f64,
    param_m08_a_3: f64,
    param_m08_a_4: f64,
    param_m08_a_5: f64,
    param_m08_a_6: f64,
    param_m08_a_7: f64,
    param_m08_a_8: f64,
    param_m08_a_9: f64,
    param_m08_a_10: f64,
    param_m08_a_11: f64,
    param_m08_b_0: f64,
    param_m08_b_1: f64,
    param_m08_b_2: f64,
    param_m08_b_3: f64,
    param_m08_b_4: f64,
    param_m08_b_5: f64,
    param_m08_b_6: f64,
    param_m08_b_7: f64,
    param_m08_b_8: f64,
    param_m08_b_9: f64,
    param_m08_b_10: f64,
    param_m08_b_11: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = param_m08_a_1;
        let t4 = M_CBRT6;
        let t5 = t4 * t4;
        let t6 = M_PI * M_PI;
        let t7 = pow_1_3(t6);
        let t8 = t7 * t7;
        let t10 = 3.0 / 10.0 * t5 * t8;
        let t11 = M_CBRT2;
        let t12 = t11 * t11;
        let t13 = tau[ip] * t12;
        let t14 = pow_1_3(rho[ip]);
        let t15 = t14 * t14;
        let t17 = 1.0 / t15 / rho[ip];
        let t18 = t13 * t17;
        let t19 = t10 - t18;
        let t20 = t3 * t19;
        let t21 = t10 + t18;
        let t22 = 1.0 / t21;
        let t24 = param_m08_a_2;
        let t25 = t19 * t19;
        let t26 = t24 * t25;
        let t27 = t21 * t21;
        let t28 = 1.0 / t27;
        let t30 = param_m08_a_3;
        let t31 = t25 * t19;
        let t32 = t30 * t31;
        let t33 = t27 * t21;
        let t34 = 1.0 / t33;
        let t36 = param_m08_a_4;
        let t37 = t25 * t25;
        let t38 = t36 * t37;
        let t39 = t27 * t27;
        let t40 = 1.0 / t39;
        let t42 = param_m08_a_5;
        let t43 = t37 * t19;
        let t44 = t42 * t43;
        let t45 = t39 * t21;
        let t46 = 1.0 / t45;
        let t48 = param_m08_a_6;
        let t49 = t37 * t25;
        let t50 = t48 * t49;
        let t51 = t39 * t27;
        let t52 = 1.0 / t51;
        let t54 = param_m08_a_7;
        let t55 = t37 * t31;
        let t56 = t54 * t55;
        let t57 = t39 * t33;
        let t58 = 1.0 / t57;
        let t60 = param_m08_a_8;
        let t61 = t37 * t37;
        let t62 = t60 * t61;
        let t63 = t39 * t39;
        let t64 = 1.0 / t63;
        let t66 = param_m08_a_9;
        let t67 = t61 * t19;
        let t68 = t66 * t67;
        let t70 = 1.0 / t63 / t21;
        let t72 = param_m08_a_10;
        let t73 = t61 * t25;
        let t74 = t72 * t73;
        let t76 = 1.0 / t63 / t27;
        let t78 = param_m08_a_11;
        let t79 = t61 * t31;
        let t80 = t78 * t79;
        let t82 = 1.0 / t63 / t33;
        let t84 = t20 * t22 + t26 * t28 + t32 * t34 + t38 * t40 + t44 * t46 + t50 * t52 + t56 * t58 + t62 * t64 + t68 * t70 + t74 * t76 + t80 * t82 + param_m08_a_0;
        let t85 = M_CBRT3;
        let t86 = 1.0 / M_PI;
        let t87 = pow_1_3(t86);
        let t88 = t85 * t87;
        let t89 = M_CBRT4;
        let t90 = t89 * t89;
        let t93 = t88 * t90 / t14;
        let t95 = 1.0 + 0.53425e-1 * t93;
        let t96 = f64::sqrt(t93);
        let t99 = pow_3_2(t93);
        let t101 = t85 * t85;
        let t102 = t87 * t87;
        let t103 = t101 * t102;
        let t106 = t103 * t89 / t15;
        let t108 = 0.379785e1 * t96 + 0.8969e0 * t93 + 0.204775e0 * t99 + 0.123235e0 * t106;
        let t111 = 1.0 + 0.16081979498692535067e2 / t108;
        let t112 = f64::ln(t111);
        let t115 = 1.0 <= zeta_threshold;
        let t116 = pow_1_3(zeta_threshold);
        let t118 = piecewise3(t115, t116 * zeta_threshold, 1.0);
        let t124 = (2.0 * t118 - 2.0) / (2.0 * t11 - 2.0);
        let t126 = 1.0 + 0.278125e-1 * t93;
        let t131 = 0.51785e1 * t96 + 0.905775e0 * t93 + 0.1100325e0 * t99 + 0.1241775e0 * t106;
        let t134 = 1.0 + 0.29608749977793437516e2 / t131;
        let t135 = f64::ln(t134);
        let t139 = -0.621814e-1 * t95 * t112 + 0.19751673498613801407e-1 * t124 * t126 * t135;
        let t140 = t84 * t139;
        let t142 = param_m08_b_1;
        let t143 = t142 * t19;
        let t145 = param_m08_b_2;
        let t146 = t145 * t25;
        let t148 = param_m08_b_3;
        let t149 = t148 * t31;
        let t151 = param_m08_b_4;
        let t152 = t151 * t37;
        let t154 = param_m08_b_5;
        let t155 = t154 * t43;
        let t157 = param_m08_b_6;
        let t158 = t157 * t49;
        let t160 = param_m08_b_7;
        let t161 = t160 * t55;
        let t163 = param_m08_b_8;
        let t164 = t163 * t61;
        let t166 = param_m08_b_9;
        let t167 = t166 * t67;
        let t169 = param_m08_b_10;
        let t170 = t169 * t73;
        let t172 = param_m08_b_11;
        let t173 = t172 * t79;
        let t175 = t143 * t22 + t146 * t28 + t149 * t34 + t152 * t40 + t155 * t46 + t158 * t52 + t161 * t58 + t164 * t64 + t167 * t70 + t170 * t76 + t173 * t82 + param_m08_b_0;
        let t176 = f64::ln(2.0);
        let t177 = 1.0 - t176;
        let t178 = t175 * t177;
        let t179 = 1.0 / t6;
        let t180 = t116 * t116;
        let t181 = piecewise3(t115, t180, 1.0);
        let t182 = t181 * t181;
        let t183 = t182 * t181;
        let t184 = t179 * t183;
        let t185 = rho[ip] * rho[ip];
        let t187 = 1.0 / t14 / t185;
        let t190 = 1.0 / t182;
        let t192 = 1.0 / t87;
        let t194 = t190 * t101 * t192 * t89;
        let t197 = 1.0 / t177;
        let t199 = 1.0 / t183;
        let t200 = t6 * t199;
        let t202 = f64::exp(-t139 * t197 * t200);
        let t203 = t202 - 1.0;
        let t204 = 1.0 / t203;
        let t205 = t197 * t204;
        let t206 = sigma[ip] * sigma[ip];
        let t207 = t185 * t185;
        let t209 = 1.0 / t15 / t207;
        let t212 = t182 * t182;
        let t213 = 1.0 / t212;
        let t214 = t12 * t213;
        let t215 = 1.0 / t102;
        let t216 = t85 * t215;
        let t217 = t216 * t90;
        let t218 = t214 * t217;
        let t221 = sigma[ip] * t187 * t11 * t194 / 96.0 + 0.21437009059034868486e-3 * t205 * t206 * t209 * t218;
        let t222 = t221 * t197;
        let t225 = 1.0 + 0.65854491829355115987e0 * t205 * t221;
        let t226 = 1.0 / t225;
        let t229 = 1.0 + 0.65854491829355115987e0 * t222 * t226;
        let t230 = f64::ln(t229);
        let t231 = t184 * t230;
        let t232 = t178 * t231;
        let tzk0 = t140 + t232;
        zk[ip] += tzk0;
        let t233 = t3 * tau[ip];
        let t235 = 1.0 / t15 / t185;
        let t237 = t12 * t235 * t22;
        let t240 = t20 * t28;
        let t241 = t13 * t235;
        let t244 = t24 * t19;
        let t245 = t244 * t28;
        let t248 = t26 * t34;
        let t251 = t30 * t25;
        let t252 = t251 * t34;
        let t255 = t32 * t40;
        let t258 = t36 * t31;
        let t259 = t258 * t40;
        let t262 = t38 * t46;
        let t265 = t42 * t37;
        let t266 = t265 * t46;
        let t269 = t44 * t52;
        let t272 = t48 * t43;
        let t273 = t272 * t52;
        let t276 = 5.0 / 3.0 * t233 * t237 + 5.0 / 3.0 * t240 * t241 + 10.0 / 3.0 * t245 * t241 + 10.0 / 3.0 * t248 * t241 + 5.0 * t252 * t241 + 5.0 * t255 * t241 + 20.0 / 3.0 * t259 * t241 + 20.0 / 3.0 * t262 * t241 + 25.0 / 3.0 * t266 * t241 + 25.0 / 3.0 * t269 * t241 + 10.0 * t273 * t241;
        let t277 = t50 * t58;
        let t280 = t54 * t49;
        let t281 = t280 * t58;
        let t284 = t56 * t64;
        let t287 = t60 * t55;
        let t288 = t287 * t64;
        let t291 = t62 * t70;
        let t294 = t66 * t61;
        let t295 = t294 * t70;
        let t298 = t68 * t76;
        let t301 = t72 * t67;
        let t302 = t301 * t76;
        let t305 = t74 * t82;
        let t308 = t78 * t73;
        let t309 = t308 * t82;
        let t313 = 1.0 / t63 / t39;
        let t314 = t80 * t313;
        let t317 = 10.0 * t277 * t241 + 35.0 / 3.0 * t281 * t241 + 35.0 / 3.0 * t284 * t241 + 40.0 / 3.0 * t288 * t241 + 40.0 / 3.0 * t291 * t241 + 15.0 * t295 * t241 + 15.0 * t298 * t241 + 50.0 / 3.0 * t302 * t241 + 50.0 / 3.0 * t305 * t241 + 55.0 / 3.0 * t309 * t241 + 55.0 / 3.0 * t314 * t241;
        let t318 = t276 + t317;
        let t319 = t318 * t139;
        let t321 = 1.0 / t14 / rho[ip];
        let t322 = t90 * t321;
        let t326 = t108 * t108;
        let t327 = 1.0 / t326;
        let t328 = t95 * t327;
        let t330 = 1.0 / t96 * t85;
        let t331 = t87 * t90;
        let t332 = t331 * t321;
        let t333 = t330 * t332;
        let t335 = t88 * t322;
        let t337 = f64::sqrt(t93);
        let t338 = t337 * t85;
        let t339 = t338 * t332;
        let t342 = t103 * t89 * t17;
        let t344 = -0.632975e0 * t333 - 0.29896666666666666667e0 * t335 - 0.1023875e0 * t339 - 0.82156666666666666667e-1 * t342;
        let t345 = 1.0 / t111;
        let t346 = t344 * t345;
        let t349 = t124 * t85;
        let t354 = t124 * t126;
        let t355 = t131 * t131;
        let t356 = 1.0 / t355;
        let t361 = -0.86308333333333333334e0 * t333 - 0.301925e0 * t335 - 0.5501625e-1 * t339 - 0.82785e-1 * t342;
        let t363 = 1.0 / t134;
        let t364 = t356 * t361 * t363;
        let t367 = 0.11073470983333333333e-2 * t88 * t322 * t112 + 1.0 * t328 * t346 - 0.18311447306006545054e-3 * t349 * t331 * t321 * t135 - 0.5848223622634646207e0 * t354 * t364;
        let t368 = t84 * t367;
        let t369 = t142 * tau[ip];
        let t372 = t143 * t28;
        let t375 = t145 * t19;
        let t376 = t375 * t28;
        let t379 = t146 * t34;
        let t382 = t148 * t25;
        let t383 = t382 * t34;
        let t386 = t149 * t40;
        let t389 = t151 * t31;
        let t390 = t389 * t40;
        let t393 = t152 * t46;
        let t396 = t154 * t37;
        let t397 = t396 * t46;
        let t400 = t155 * t52;
        let t403 = t157 * t43;
        let t404 = t403 * t52;
        let t407 = 5.0 / 3.0 * t369 * t237 + 5.0 / 3.0 * t372 * t241 + 10.0 / 3.0 * t376 * t241 + 10.0 / 3.0 * t379 * t241 + 5.0 * t383 * t241 + 5.0 * t386 * t241 + 20.0 / 3.0 * t390 * t241 + 20.0 / 3.0 * t393 * t241 + 25.0 / 3.0 * t397 * t241 + 25.0 / 3.0 * t400 * t241 + 10.0 * t404 * t241;
        let t408 = t158 * t58;
        let t411 = t160 * t49;
        let t412 = t411 * t58;
        let t415 = t161 * t64;
        let t418 = t163 * t55;
        let t419 = t418 * t64;
        let t422 = t164 * t70;
        let t425 = t166 * t61;
        let t426 = t425 * t70;
        let t429 = t167 * t76;
        let t432 = t169 * t67;
        let t433 = t432 * t76;
        let t436 = t170 * t82;
        let t439 = t172 * t73;
        let t440 = t439 * t82;
        let t443 = t173 * t313;
        let t446 = 10.0 * t408 * t241 + 35.0 / 3.0 * t412 * t241 + 35.0 / 3.0 * t415 * t241 + 40.0 / 3.0 * t419 * t241 + 40.0 / 3.0 * t422 * t241 + 15.0 * t426 * t241 + 15.0 * t429 * t241 + 50.0 / 3.0 * t433 * t241 + 50.0 / 3.0 * t436 * t241 + 55.0 / 3.0 * t440 * t241 + 55.0 / 3.0 * t443 * t241;
        let t447 = t407 + t446;
        let t448 = t447 * t177;
        let t449 = t448 * t231;
        let t450 = t178 * t179;
        let t451 = t185 * rho[ip];
        let t453 = 1.0 / t14 / t451;
        let t458 = t177 * t177;
        let t459 = 1.0 / t458;
        let t460 = t203 * t203;
        let t461 = 1.0 / t460;
        let t462 = t459 * t461;
        let t463 = t462 * t206;
        let t464 = t209 * t12;
        let t466 = 1.0 / t212 / t183;
        let t467 = t464 * t466;
        let t468 = t463 * t467;
        let t469 = t367 * t6;
        let t470 = t469 * t202;
        let t471 = t217 * t470;
        let t474 = t207 * rho[ip];
        let t476 = 1.0 / t15 / t474;
        let t481 = -7.0 / 288.0 * sigma[ip] * t453 * t11 * t194 + 0.21437009059034868486e-3 * t468 * t471 - 0.10003937560882938627e-2 * t205 * t206 * t476 * t218;
        let t482 = t481 * t197;
        let t485 = t225 * t225;
        let t486 = 1.0 / t485;
        let t487 = t462 * t221;
        let t488 = t199 * t202;
        let t489 = t469 * t488;
        let t494 = 0.65854491829355115987e0 * t487 * t489 + 0.65854491829355115987e0 * t205 * t481;
        let t495 = t486 * t494;
        let t498 = 0.65854491829355115987e0 * t482 * t226 - 0.65854491829355115987e0 * t222 * t495;
        let t500 = 1.0 / t229;
        let t501 = t183 * t498 * t500;
        let t502 = t450 * t501;
        let tvrho0 = t140 + t232 + rho[ip] * (t319 + t368 + t449 + t502);
        vrho[ip] += tvrho0;
        let t505 = rho[ip] * t175;
        let t506 = t505 * t177;
        let t510 = t101 * t192 * t89;
        let t517 = t187 * t11 * t190 * t510 / 96.0 + 0.42874018118069736972e-3 * t205 * sigma[ip] * t209 * t218;
        let t518 = t517 * t197;
        let t521 = t221 * t459;
        let t522 = t486 * t204;
        let t523 = t522 * t517;
        let t526 = 0.65854491829355115987e0 * t518 * t226 - 0.4336814094102599731e0 * t521 * t523;
        let t528 = t184 * t526 * t500;
        let tvsigma0 = t506 * t528;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t529 = t3 * t12;
        let t530 = t17 * t22;
        let t532 = t28 * t12;
        let t533 = t532 * t17;
        let t537 = t34 * t12;
        let t538 = t537 * t17;
        let t543 = t40 * t12;
        let t544 = t543 * t17;
        let t549 = t46 * t12;
        let t550 = t549 * t17;
        let t555 = t52 * t12;
        let t556 = t555 * t17;
        let t561 = -t20 * t533 - 2.0 * t244 * t533 - 3.0 * t251 * t538 - 4.0 * t258 * t544 - 2.0 * t26 * t538 - 5.0 * t265 * t550 - 6.0 * t272 * t556 - 3.0 * t32 * t544 - 4.0 * t38 * t550 - 5.0 * t44 * t556 - t529 * t530;
        let t562 = t58 * t12;
        let t563 = t562 * t17;
        let t568 = t64 * t12;
        let t569 = t568 * t17;
        let t574 = t70 * t12;
        let t575 = t574 * t17;
        let t580 = t76 * t12;
        let t581 = t580 * t17;
        let t586 = t82 * t12;
        let t587 = t586 * t17;
        let t592 = t313 * t12;
        let t593 = t592 * t17;
        let t596 = -7.0 * t280 * t563 - 8.0 * t287 * t569 - 9.0 * t294 * t575 - 10.0 * t301 * t581 - 11.0 * t308 * t587 - 6.0 * t50 * t563 - 7.0 * t56 * t569 - 8.0 * t62 * t575 - 9.0 * t68 * t581 - 10.0 * t74 * t587 - 11.0 * t80 * t593;
        let t597 = t561 + t596;
        let t598 = t597 * t139;
        let t599 = t142 * t12;
        let t620 = -t143 * t533 - 2.0 * t146 * t538 - 3.0 * t149 * t544 - 4.0 * t152 * t550 - 5.0 * t155 * t556 - 2.0 * t375 * t533 - 3.0 * t382 * t538 - 4.0 * t389 * t544 - 5.0 * t396 * t550 - 6.0 * t403 * t556 - t599 * t530;
        let t643 = -6.0 * t158 * t563 - 7.0 * t161 * t569 - 8.0 * t164 * t575 - 9.0 * t167 * t581 - 10.0 * t170 * t587 - 11.0 * t173 * t593 - 7.0 * t411 * t563 - 8.0 * t418 * t569 - 9.0 * t425 * t575 - 10.0 * t432 * t581 - 11.0 * t439 * t587;
        let t644 = t620 + t643;
        let t645 = t644 * t177;
        let t646 = t645 * t231;
        let tvtau0 = rho[ip] * (t598 + t646);
        vtau[ip] += tvtau0;
    }
}
