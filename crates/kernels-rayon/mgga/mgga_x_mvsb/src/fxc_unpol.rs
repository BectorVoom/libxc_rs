//! MGGA_X_MVSB fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvsb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mvsb_fxc_unpol(
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
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t7 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = tau[ip] * t22;
        let t24 = t20 * t20;
        let t26 = 1.0 / t24 / rho[ip];
        let t27 = t23 * t26;
        let t28 = sigma[ip] * t22;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t24 / t29;
        let t34 = t27 - t28 * t31 / 8.0;
        let t35 = M_CBRT6;
        let t36 = t35 * t35;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3(t37);
        let t39 = t38 * t38;
        let t42 = t27 - 3.0 / 10.0 * t36 * t39;
        let t43 = 1.0 / t42;
        let t46 = param_k0 * (-t34 * t43 + 1.0);
        let t47 = t34 * t34;
        let t48 = param_e1 * t47;
        let t49 = t42 * t42;
        let t50 = 1.0 / t49;
        let t52 = t48 * t50 + 1.0;
        let t53 = t52 * t52;
        let t54 = t47 * t47;
        let t55 = param_c1 * t54;
        let t56 = t49 * t49;
        let t57 = 1.0 / t56;
        let t59 = t55 * t57 + t53;
        let t60 = pow_1_4(t59);
        let t61 = 1.0 / t60;
        let t63 = t46 * t61 + 1.0;
        let t67 = 1.0 / t38 / t37;
        let t69 = sigma[ip] * sigma[ip];
        let t71 = t29 * t29;
        let t72 = t71 * rho[ip];
        let t74 = 1.0 / t20 / t72;
        let t78 = 1.0 + param_b * t36 * t67 * t69 * t21 * t74 / 288.0;
        let t79 = f64::powf(t78, 1.0 / 8.0);
        let t80 = 1.0 / t79;
        let t84 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t63 * t80);
        let tzk0 = 2.0 * t84;
        zk[ip] += tzk0;
        let t85 = 1.0 / t24;
        let t90 = t23 * t31;
        let t92 = t29 * rho[ip];
        let t94 = 1.0 / t24 / t92;
        let t97 = -5.0 / 3.0 * t90 + t28 * t94 / 3.0;
        let t99 = t34 * t50;
        let t103 = param_k0 * (-t97 * t43 - 5.0 / 3.0 * t99 * t90);
        let t106 = 1.0 / t60 / t59;
        let t107 = param_e1 * t34;
        let t108 = t50 * t97;
        let t111 = t49 * t42;
        let t112 = 1.0 / t111;
        let t113 = t48 * t112;
        let t116 = 2.0 * t107 * t108 + 10.0 / 3.0 * t113 * t90;
        let t120 = param_c1 * t47 * t34;
        let t121 = t57 * t97;
        let t125 = 1.0 / t56 / t42;
        let t126 = t55 * t125;
        let t129 = 2.0 * t52 * t116 + 4.0 * t120 * t121 + 20.0 / 3.0 * t126 * t90;
        let t130 = t106 * t129;
        let t133 = t103 * t61 - t46 * t130 / 4.0;
        let t138 = t71 * t29;
        let t139 = 1.0 / t138;
        let t140 = t18 * t139;
        let t142 = t7 * t140 * t63;
        let t145 = 1.0 / t79 / t78 * param_b;
        let t146 = t145 * t36;
        let t149 = t146 * t67 * t69 * t21;
        let t153 = piecewise3(t3, 0.0, -t19 * t85 * t63 * t80 / 8.0 - 3.0 / 8.0 * t19 * t20 * t133 * t80 - t142 * t149 / 1152.0);
        let tvrho0 = 2.0 * rho[ip] * t153 + 2.0 * t84;
        vrho[ip] += tvrho0;
        let t156 = param_k0 * t22;
        let t157 = t31 * t43;
        let t161 = t52 * param_e1;
        let t162 = t161 * t34;
        let t163 = t50 * t22;
        let t164 = t163 * t31;
        let t166 = t57 * t22;
        let t167 = t166 * t31;
        let t168 = t120 * t167;
        let t170 = -t162 * t164 / 2.0 - t168 / 2.0;
        let t171 = t106 * t170;
        let t174 = t156 * t157 * t61 / 8.0 - t46 * t171 / 4.0;
        let t179 = 1.0 / t72;
        let t180 = t18 * t179;
        let t182 = t7 * t180 * t63;
        let t185 = t146 * t67 * sigma[ip] * t21;
        let t189 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t174 * t80 + t182 * t185 / 3072.0);
        let tvsigma0 = 2.0 * rho[ip] * t189;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t191 = t22 * t26;
        let t195 = param_k0 * (-t191 * t43 + t99 * t191);
        let t197 = t163 * t26;
        let t199 = t112 * t22;
        let t200 = t199 * t26;
        let t203 = 2.0 * t107 * t197 - 2.0 * t48 * t200;
        let t206 = t166 * t26;
        let t209 = t125 * t22;
        let t213 = -4.0 * t55 * t209 * t26 + 4.0 * t120 * t206 + 2.0 * t52 * t203;
        let t214 = t106 * t213;
        let t217 = t195 * t61 - t46 * t214 / 4.0;
        let t222 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t217 * t80);
        let tvtau0 = 2.0 * rho[ip] * t222;
        vtau[ip] += tvtau0;
        let t233 = t71 * t92;
        let t234 = 1.0 / t233;
        let t235 = t18 * t234;
        let t237 = t7 * t235 * t63;
        let t240 = t23 * t94;
        let t243 = 1.0 / t24 / t71;
        let t246 = 40.0 / 9.0 * t240 - 11.0 / 9.0 * t28 * t243;
        let t250 = t34 * t112;
        let t251 = tau[ip] * tau[ip];
        let t252 = t251 * t21;
        let t253 = t252 * t74;
        let t259 = param_k0 * (-t246 * t43 - 10.0 / 3.0 * t108 * t90 - 100.0 / 9.0 * t250 * t253 + 40.0 / 9.0 * t99 * t240);
        let t263 = t59 * t59;
        let t265 = 1.0 / t60 / t263;
        let t266 = t129 * t129;
        let t267 = t265 * t266;
        let t270 = t116 * t116;
        let t272 = t97 * t97;
        let t273 = param_e1 * t272;
        let t276 = t107 * t112;
        let t277 = t97 * tau[ip];
        let t278 = t22 * t31;
        let t279 = t277 * t278;
        let t282 = t50 * t246;
        let t285 = t48 * t57;
        let t290 = 2.0 * t273 * t50 + 40.0 / 3.0 * t276 * t279 + 2.0 * t107 * t282 + 100.0 / 3.0 * t285 * t253 - 80.0 / 9.0 * t113 * t240;
        let t293 = param_c1 * t47;
        let t294 = t57 * t272;
        let t297 = t120 * t125;
        let t304 = 1.0 / t56 / t49;
        let t305 = t55 * t304;
        let t310 = 2.0 * t270 + 2.0 * t52 * t290 + 12.0 * t293 * t294 + 160.0 / 3.0 * t297 * t279 + 4.0 * t120 * t57 * t246 + 1000.0 / 9.0 * t305 * t253 - 160.0 / 9.0 * t126 * t240;
        let t311 = t106 * t310;
        let t314 = t259 * t61 - t103 * t130 / 2.0 + 5.0 / 16.0 * t46 * t267 - t46 * t311 / 4.0;
        let t320 = t7 * t140 * t133;
        let t323 = t71 * t71;
        let t327 = t18 / t20 / t323 / t71;
        let t329 = t7 * t327 * t63;
        let t330 = t78 * t78;
        let t333 = param_b * param_b;
        let t334 = 1.0 / t79 / t330 * t333;
        let t335 = t334 * t35;
        let t336 = t37 * t37;
        let t338 = 1.0 / t39 / t336;
        let t339 = t69 * t69;
        let t342 = t335 * t338 * t339 * t22;
        let t346 = piecewise3(t3, 0.0, t19 * t26 * t63 * t80 / 12.0 - t19 * t85 * t133 * t80 / 4.0 + 17.0 / 3456.0 * t237 * t149 - 3.0 / 8.0 * t19 * t20 * t314 * t80 - t320 * t149 / 576.0 - t329 * t342 / 9216.0);
        let tv2rho20 = 2.0 * rho[ip] * t346 + 4.0 * t153;
        v2rho2[ip] += tv2rho20;
        let t357 = param_k0 * t21;
        let t358 = t357 * t74;
        let t360 = t50 * t61 * tau[ip];
        let t363 = t156 * t31;
        let t364 = t43 * t106;
        let t365 = t364 * t129;
        let t370 = t265 * t170;
        let t371 = t370 * t129;
        let t374 = t116 * param_e1;
        let t375 = t374 * t34;
        let t378 = t161 * t97;
        let t381 = t112 * t21;
        let t383 = t381 * t74 * tau[ip];
        let t386 = t163 * t94;
        let t389 = t293 * t57;
        let t390 = t278 * t97;
        let t391 = t389 * t390;
        let t393 = t21 * t74;
        let t394 = t393 * tau[ip];
        let t395 = t297 * t394;
        let t398 = t120 * t166 * t94;
        let t400 = -t375 * t164 / 2.0 - t378 * t164 / 2.0 - 10.0 / 3.0 * t162 * t383 + 4.0 / 3.0 * t162 * t386 - 3.0 / 2.0 * t391 - 20.0 / 3.0 * t395 + 4.0 / 3.0 * t398;
        let t401 = t106 * t400;
        let t404 = -t156 * t94 * t43 * t61 / 3.0 + 5.0 / 12.0 * t358 * t360 - t363 * t365 / 32.0 - t103 * t171 / 4.0 + 5.0 / 16.0 * t46 * t371 - t46 * t401 / 4.0;
        let t410 = t7 * t140 * t174;
        let t416 = t7 * t180 * t133;
        let t422 = t18 / t20 / t323 / t92;
        let t424 = t7 * t422 * t63;
        let t425 = t69 * sigma[ip];
        let t428 = t335 * t338 * t425 * t22;
        let t432 = piecewise3(t3, 0.0, -t19 * t85 * t174 * t80 / 8.0 - 3.0 / 8.0 * t19 * t20 * t404 * t80 - t410 * t149 / 1152.0 - 5.0 / 3072.0 * t142 * t185 + t416 * t185 / 3072.0 + t424 * t428 / 24576.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t432 + 2.0 * t189;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t442 = 1.0 / t20 / t71;
        let t443 = t21 * t442;
        let t444 = t50 * tau[ip];
        let t448 = t443 * tau[ip];
        let t454 = param_k0 * (5.0 / 3.0 * t278 * t43 - 10.0 / 3.0 * t443 * t444 + t108 * t191 + 20.0 / 3.0 * t250 * t448 - 5.0 / 3.0 * t99 * t278);
        let t460 = t265 * t213;
        let t461 = t460 * t129;
        let t466 = param_e1 * t97;
        let t473 = t191 * t97;
        let t478 = t199 * t31;
        let t481 = 2.0 * t466 * t197 + 40.0 / 3.0 * t276 * t448 - 10.0 / 3.0 * t107 * t164 - 4.0 * t276 * t473 - 20.0 * t285 * t448 + 10.0 / 3.0 * t48 * t478;
        let t496 = 2.0 * t116 * t203 + 2.0 * t52 * t481 + 12.0 * t389 * t473 + 160.0 / 3.0 * t297 * t448 - 20.0 / 3.0 * t168 - 16.0 * t297 * t473 - 200.0 / 3.0 * t305 * t448 + 20.0 / 3.0 * t55 * t209 * t31;
        let t497 = t106 * t496;
        let t500 = t454 * t61 - t195 * t130 / 4.0 - t103 * t214 / 4.0 + 5.0 / 16.0 * t46 * t461 - t46 * t497 / 4.0;
        let t506 = t7 * t140 * t217;
        let t510 = piecewise3(t3, 0.0, -t19 * t85 * t217 * t80 / 8.0 - 3.0 / 8.0 * t19 * t20 * t500 * t80 - t506 * t149 / 1152.0);
        let tv2rhotau0 = 2.0 * rho[ip] * t510 + 2.0 * t222;
        v2rhotau[ip] += tv2rhotau0;
        let t513 = t364 * t170;
        let t516 = t170 * t170;
        let t517 = t265 * t516;
        let t520 = param_e1 * param_e1;
        let t521 = t520 * t47;
        let t522 = t57 * t21;
        let t523 = t522 * t74;
        let t526 = t393 * t50;
        let t529 = t293 * t523;
        let t531 = t521 * t523 / 4.0 + t161 * t526 / 8.0 + 3.0 / 8.0 * t529;
        let t532 = t106 * t531;
        let t535 = -t363 * t513 / 16.0 + 5.0 / 16.0 * t46 * t517 - t46 * t532 / 4.0;
        let t541 = t7 * t180 * t174;
        let t544 = t323 * t29;
        let t547 = t18 / t20 / t544;
        let t549 = t7 * t547 * t63;
        let t552 = t335 * t338 * t69 * t22;
        let t556 = t36 * t67 * t21;
        let t557 = t145 * t556;
        let t561 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t535 * t80 + t541 * t185 / 1536.0 - t549 * t552 / 65536.0 + t182 * t557 / 3072.0);
        let tv2sigma20 = 2.0 * rho[ip] * t561;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t563 = t442 * t50;
        let t569 = t364 * t213;
        let t572 = t460 * t170;
        let t575 = t107 * t50;
        let t576 = t278 * t203;
        let t579 = param_e1 * t21;
        let t580 = t579 * t563;
        let t582 = t381 * t442;
        let t583 = t107 * t582;
        let t584 = -t580 / 2.0 + t583;
        let t587 = t522 * t442;
        let t588 = t293 * t587;
        let t590 = t125 * t21;
        let t592 = t120 * t590 * t442;
        let t594 = -t575 * t576 / 2.0 + 2.0 * t52 * t584 - 3.0 * t588 + 4.0 * t592;
        let t595 = t106 * t594;
        let t598 = -t357 * t563 * t61 / 4.0 - t195 * t171 / 4.0 - t363 * t569 / 32.0 + 5.0 / 16.0 * t46 * t572 - t46 * t595 / 4.0;
        let t604 = t7 * t180 * t217;
        let t608 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t598 * t80 + t604 * t185 / 3072.0);
        let tv2sigmatau0 = 2.0 * rho[ip] * t608;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t611 = 1.0 / t20 / t92;
        let t612 = t21 * t611;
        let t617 = param_k0 * (-4.0 * t250 * t612 + 4.0 * t612 * t50);
        let t621 = t213 * t213;
        let t622 = t265 * t621;
        let t625 = t203 * t203;
        let t630 = t381 * t611;
        let t633 = t522 * t611;
        let t636 = 4.0 * t579 * t611 * t50 - 16.0 * t107 * t630 + 12.0 * t48 * t633;
        let t644 = t304 * t21;
        let t648 = -64.0 * t120 * t590 * t611 + 40.0 * t55 * t644 * t611 + 24.0 * t293 * t633 + 2.0 * t52 * t636 + 2.0 * t625;
        let t649 = t106 * t648;
        let t652 = t617 * t61 - t195 * t214 / 2.0 + 5.0 / 16.0 * t46 * t622 - t46 * t649 / 4.0;
        let t657 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t652 * t80);
        let tv2tau20 = 2.0 * rho[ip] * t657;
        v2tau2[ip] += tv2tau20;
    }
}
