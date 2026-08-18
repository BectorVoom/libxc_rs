//! GGA_X_SG4 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sg4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sg4_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = 1.0 - 0.0031233982573039467 * t34;
        let t37 = t20 * t20;
        let t38 = t21 * t21;
        let t39 = t38 * t21;
        let t41 = 1.0 / t22 / t39;
        let t42 = t37 * t41;
        let t43 = sigma[ip] * sigma[ip];
        let t44 = t43 * t43;
        let t45 = t44 * sigma[ip];
        let t47 = t29 * t29;
        let t48 = t47 * rho[ip];
        let t49 = t47 * t47;
        let t50 = t49 * t48;
        let t52 = 1.0 / t18 / t50;
        let t56 = 1.0 - 1.426849132767203e-11 * t42 * t45 * t26 * t52;
        let t57 = 1.0 / t56;
        let t61 = 1.0 + 0.03727064220183486 * t34;
        let t64 = 1.804 - 0.5602871794871794 * t36 * t57 - 0.2437128205128205 / t61;
        let t68 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t64);
        let tzk0 = 2.0 * t68;
        zk[ip] += tzk0;
        let t70 = t17 / t30;
        let t74 = t25 * sigma[ip];
        let t75 = t29 * rho[ip];
        let t77 = 1.0 / t30 / t75;
        let t79 = t27 * t77 * t57;
        let t82 = t56 * t56;
        let t83 = 1.0 / t82;
        let t85 = t36 * t83 * t37;
        let t86 = t41 * t45;
        let t87 = t47 * t29;
        let t88 = t49 * t87;
        let t91 = t26 / t18 / t88;
        let t95 = t61 * t61;
        let t97 = 1.0 / t95 * t20;
        let t98 = t97 * t24;
        let t102 = -0.004666666666666667 * t74 * t79 + 1.0659270348691523e-10 * t85 * t86 * t91 - 0.02422222222222222 * t98 * t28 * t77;
        let t107 = piecewise3(t2, 0.0, -t6 * t70 * t64 / 8.0 - 3.0 / 8.0 * t6 * t19 * t102);
        let tvrho0 = 2.0 * rho[ip] * t107 + 2.0 * t68;
        vrho[ip] += tvrho0;
        let t114 = t41 * t44;
        let t115 = t26 * t52;
        let t119 = t24 * t27;
        let t123 = 0.00175 * t25 * t27 * t32 * t57 - 3.997226380759321e-11 * t85 * t114 * t115 + 0.009083333333333334 * t97 * t119 * t32;
        let t127 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t123);
        let tvsigma0 = 2.0 * rho[ip] * t127;
        vsigma[ip] += tvsigma0;
        let t132 = t17 / t30 / rho[ip];
        let t140 = 1.0 / t30 / t47;
        let t142 = t27 * t140 * t57;
        let t145 = t44 * t43;
        let t146 = t49 * t49;
        let t148 = 1.0 / t146 / t29;
        let t153 = 1.0 / t82 / t56;
        let t155 = t36 * t153 * t20;
        let t156 = t38 * t38;
        let t159 = 1.0 / t23 / t156 / t38;
        let t160 = t44 * t44;
        let t161 = t160 * t43;
        let t162 = t159 * t161;
        let t163 = t49 * t47;
        let t167 = t27 / t30 / t146 / t163;
        let t171 = t47 * t75;
        let t172 = t49 * t171;
        let t175 = t26 / t18 / t172;
        let t181 = 1.0 / t95 / t61 * t37;
        let t183 = 1.0 / t22 / t21;
        let t184 = t181 * t183;
        let t185 = t43 * t26;
        let t187 = 1.0 / t18 / t171;
        let t194 = 0.01711111111111111 * t74 * t142 + 2.245617754729564e-15 * t145 * t148 * t83 - 2.4334673044738656e-19 * t155 * t162 * t167 - 1.5278287499791183e-09 * t85 * t86 * t175 - 0.00962962962962963 * t184 * t185 * t187 + 0.08881481481481482 * t98 * t28 * t140;
        let t199 = piecewise3(t2, 0.0, t6 * t132 * t64 / 12.0 - t6 * t70 * t102 / 4.0 - 3.0 / 8.0 * t6 * t19 * t194);
        let tv2rho20 = 2.0 * rho[ip] * t199 + 4.0 * t107;
        v2rho2[ip] += tv2rho20;
        let t207 = t146 * rho[ip];
        let t209 = 1.0 / t207 * t83;
        let t212 = t160 * sigma[ip];
        let t213 = t159 * t212;
        let t214 = t49 * t75;
        let t218 = t27 / t30 / t146 / t214;
        let t226 = 1.0 / t18 / t87;
        let t234 = -0.004666666666666667 * t25 * t79 - 8.421066580235865e-16 * t209 * t45 + 9.125502391776996e-20 * t155 * t213 * t218 + 5.329635174345761e-10 * t85 * t114 * t91 + 0.003611111111111111 * t184 * t26 * t226 * sigma[ip] - 0.02422222222222222 * t97 * t119 * t77;
        let t239 = piecewise3(t2, 0.0, -t6 * t70 * t123 / 8.0 - 3.0 / 8.0 * t6 * t19 * t234);
        let tv2rhosigma0 = 2.0 * rho[ip] * t239 + 2.0 * t127;
        v2rhosigma[ip] += tv2rhosigma0;
        let t243 = 1.0 / t146 * t83;
        let t246 = t159 * t160;
        let t247 = t49 * t29;
        let t251 = t27 / t30 / t146 / t247;
        let t255 = t43 * sigma[ip];
        let t256 = t41 * t255;
        let t260 = t183 * t26;
        let t266 = 3.157899967588449e-16 * t243 * t44 - 3.4220633969163733e-20 * t155 * t246 * t251 - 1.5988905523037283e-10 * t85 * t256 * t115 - 0.0013541666666666667 * t181 * t260 / t18 / t48;
        let t270 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t266);
        let tv2sigma20 = 2.0 * rho[ip] * t270;
        v2sigma2[ip] += tv2sigma20;
        let t273 = t17 * t32;
        let t284 = 1.0 / t30 / t48;
        let t286 = t27 * t284 * t57;
        let t290 = 1.0 / t146 / t75;
        let t294 = t160 * t255;
        let t295 = t146 * t146;
        let t297 = 1.0 / t18 / t295;
        let t300 = t42 * t26;
        let t306 = t37 / t22 / t156 / t39;
        let t307 = t306 * t294;
        let t309 = t26 * t297 * t153;
        let t312 = t82 * t82;
        let t313 = 1.0 / t312;
        let t314 = t36 * t313;
        let t315 = t44 * t255;
        let t316 = t160 * t315;
        let t317 = t295 * t214;
        let t318 = 1.0 / t317;
        let t322 = t146 * t50;
        let t325 = t27 / t30 / t322;
        let t331 = t26 / t18 / t146;
        let t335 = t95 * t95;
        let t336 = 1.0 / t335;
        let t337 = t336 * t255;
        let t338 = 1.0 / t214;
        let t342 = 1.0 / t18 / t49;
        let t349 = -0.07985185185185185 * t74 * t286 - 6.063167937769823e-14 * t145 * t290 * t83 - 8.544420655633364e-25 * t294 * t297 * t153 * t300 - 4.053700020266563e-21 * t307 * t309 + 1.9004102083915272e-37 * t314 * t316 * t318 + 1.0463909409237622e-17 * t155 * t162 * t325 + 2.3426707499679815e-08 * t85 * t86 * t331 - 0.00035371070357627984 * t337 * t338 + 0.10592592592592592 * t184 * t185 * t342 - 0.41446913580246914 * t98 * t28 * t284;
        let t354 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t273 * t64 + t6 * t132 * t102 / 4.0 - 3.0 / 8.0 * t6 * t70 * t194 - 3.0 / 8.0 * t6 * t19 * t349);
        let tv3rho30 = 2.0 * rho[ip] * t354 + 6.0 * t199;
        v3rho3[ip] += tv3rho30;
        let t366 = t148 * t83;
        let t371 = 1.0 / t18 / t146 / t172;
        let t372 = t371 * t153;
        let t376 = t306 * t161;
        let t378 = t26 * t371 * t153;
        let t381 = t160 * t145;
        let t382 = t295 * t247;
        let t383 = 1.0 / t382;
        let t394 = t336 / t247;
        let t404 = 0.01711111111111111 * t25 * t142 + 2.105266645058966e-14 * t366 * t45 + 3.204157745862512e-25 * t372 * t161 * t300 + 1.5201375075999608e-21 * t376 * t378 - 7.126538281468228e-38 * t314 * t381 * t383 - 3.741455980628569e-18 * t155 * t213 * t167 - 7.639143749895592e-09 * t85 * t114 * t175 + 0.00013264151384110494 * t394 * t43 - 0.0325 * t184 * t26 * t187 * sigma[ip] + 0.08881481481481482 * t97 * t119 * t140;
        let t409 = piecewise3(t2, 0.0, t6 * t132 * t123 / 12.0 - t6 * t70 * t234 / 4.0 - 3.0 / 8.0 * t6 * t19 * t404);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t409 + 4.0 * t239;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t417 = t146 * t88;
        let t419 = 1.0 / t18 / t417;
        let t420 = t419 * t153;
        let t424 = t306 * t212;
        let t429 = t160 * t45;
        let t430 = t49 * rho[ip];
        let t432 = 1.0 / t295 / t430;
        let t443 = t336 / t430;
        let t449 = -6.736853264188692e-15 * t209 * t44 - 1.201559154698442e-25 * t420 * t212 * t300 - 5.700515653499853e-22 * t424 * t26 * t419 * t153 + 2.672451855550585e-38 * t314 * t429 * t432 + 1.2775703348487794e-18 * t155 * t246 * t218 + 2.1318540697383044e-09 * t85 * t256 * t91 - 4.974056769041435e-05 * t443 * sigma[ip] + 0.007222222222222222 * t181 * t260 * t226;
        let t454 = piecewise3(t2, 0.0, -t6 * t70 * t266 / 8.0 - 3.0 / 8.0 * t6 * t19 * t449);
        let tv3rhosigma20 = 2.0 * rho[ip] * t454 + 2.0 * t270;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t459 = 1.0 / t18 / t322 * t153;
        let t460 = t459 * t160;
        let t465 = t306 * t26;
        let t468 = t160 * t44;
        let t470 = 1.0 / t295 / t49;
        let t474 = t159 * t315;
        let t478 = t41 * t43;
        let t485 = 4.505846830119157e-26 * t460 * t300 + 1.8947399805530697e-15 * t243 * t255 + 2.137693370062445e-22 * t465 * t460 - 1.0021694458314695e-38 * t314 * t468 * t470 - 4.106476076299648e-19 * t155 * t474 * t251 - 4.796671656911185e-10 * t85 * t478 * t115 + 1.8652712883905384e-05 * t336 / t49;
        let t489 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t485);
        let tv3sigma30 = 2.0 * rho[ip] * t489;
        v3sigma3[ip] += tv3sigma30;
        let t506 = 1.0 / t30 / t87;
        let t518 = 1.0 / t18 / t295 / rho[ip];
        let t523 = t160 * t160;
        let t526 = 1.0 / t30 / t295 / t88;
        let t530 = t20 * t159 * t27;
        let t537 = t156 * t156;
        let t541 = t20 / t23 / t537 / t38;
        let t544 = t27 * t526 * t313;
        let t552 = t36 / t312 / t56;
        let t560 = t41 * t26;
        let t564 = t295 * t163;
        let t582 = 1.0 / t335 / t61;
        let t587 = t25 * t27;
        let t601 = 0.4524938271604938 * t74 * t27 * t506 * t57 + 1.4179828555697902e-12 * t145 / t146 / t47 * t83 + 5.06968958900913e-23 * t294 * t518 * t153 * t300 + 2.925983808597274e-33 * t523 * t526 * t313 * t530 + 3.05378734860081e-19 * t307 * t26 * t518 * t153 + 1.388164406019777e-29 * t541 * t523 * t544 + 1.5828634488140606e-39 * t25 * t523 * t544 - 1.4461859507975415e-46 * t552 * t523 * t44 / t18 / t295 / t146 / t430 * t37 * t560 - 1.6343527792167136e-35 * t314 * t316 / t564 - 3.6391151612126397e-16 * t155 * t162 * t27 / t30 / t417 - 3.826362224947703e-07 * t85 * t86 * t26 / t18 / t207 - 0.00014061893414347518 * t582 * t44 / t30 / t88 * t587 + 0.007781635478678156 * t337 / t163 - 1.0474897119341564 * t184 * t185 / t18 / t430 + 2.348658436213992 * t98 * t28 * t506;
        let t606 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t77 * t64 - 5.0 / 9.0 * t6 * t273 * t102 + t6 * t132 * t194 / 2.0 - t6 * t70 * t349 / 2.0 - 3.0 / 8.0 * t6 * t19 * t601);
        let tv4rho40 = 2.0 * rho[ip] * t606 + 8.0 * t354;
        v4rho4[ip] += tv4rho40;
        let t630 = 1.0 / t30 / t295 / t50;
        let t639 = t27 * t630 * t313;
        let t680 = -0.07985185185185185 * t25 * t286 - 4.635329315387608e-13 * t290 * t83 * t45 - 1.8050088635025484e-23 * t297 * t153 * t161 * t300 - 1.0972439282239778e-33 * t630 * t313 * t316 * t530 - 1.0995661304973051e-19 * t376 * t309 - 5.205616522574164e-30 * t541 * t316 * t639 - 5.9357379330527265e-40 * t25 * t316 * t639 + 5.42319731549078e-47 * t552 * t523 * t255 / t18 / t295 / t146 / t49 * t37 * t560 + 5.915026773618629e-36 * t314 * t381 * t318 + 1.2469492046008166e-16 * t155 * t213 * t325 + 1.1713353749839907e-07 * t85 * t114 * t331 + 5.273210030380319e-05 * t582 / t30 / t50 * t255 * t587 - 0.002520188762980994 * t336 * t338 * t43 + 0.273641975308642 * t184 * t26 * t342 * sigma[ip] - 0.41446913580246914 * t97 * t119 * t284;
        let t685 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t273 * t123 + t6 * t132 * t234 / 4.0 - 3.0 / 8.0 * t6 * t70 * t404 - 3.0 / 8.0 * t6 * t19 * t680);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t685 + 6.0 * t409;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t701 = 1.0 / t30 / t564;
        let t710 = t27 * t701 * t313;
        let t746 = 1.369826830385034e-13 * t366 * t44 + 6.2080556326086165e-24 * t372 * t212 * t300 + 4.114664730839917e-34 * t701 * t313 * t381 * t530 + 3.857348925534901e-20 * t424 * t378 + 1.9521061959653114e-30 * t541 * t381 * t710 + 2.2259017248947725e-40 * t25 * t381 * t710 - 2.0336989933090428e-47 * t552 * t523 * t43 / t18 / t295 / t146 / t171 * t37 * t560 - 2.093420620181292e-36 * t314 * t429 * t383 - 4.021304720643063e-17 * t155 * t246 * t167 - 3.055657499958237e-08 * t85 * t256 * t175 - 1.9774537613926197e-05 * t582 / t30 / t163 * t43 * t587 + 0.0007129481368959391 * t394 * sigma[ip] - 0.04574074074074074 * t181 * t260 * t187;
        let t751 = piecewise3(t2, 0.0, t6 * t132 * t266 / 12.0 - t6 * t70 * t449 / 4.0 - 3.0 / 8.0 * t6 * t19 * t746);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t751 + 4.0 * t454;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t757 = t420 * t160;
        let t761 = 1.0 / t30 / t317;
        let t763 = t761 * t313 * t429;
        let t770 = t541 * t27;
        let t806 = -2.0426505629873512e-24 * t757 * t300 - 1.5429992740649686e-34 * t763 * t530 - 3.536847963699063e-14 * t209 * t255 - 1.3111186003049663e-20 * t465 * t757 - 7.320398234869918e-31 * t770 * t763 - 8.347131468355397e-41 * t25 * t429 * t27 * t761 * t313 + 7.62637122490891e-48 * t552 * t523 * sigma[ip] / t18 / t295 / t146 / t87 * t37 * t560 + 7.215620009986581e-37 * t314 * t468 * t432 + 1.2045663157145635e-17 * t155 * t474 * t218 + 6.395562209214914e-09 * t85 * t478 * t91 + 7.415451605222323e-06 * t582 / t30 / t214 * t20 * t24 * sigma[ip] * t27 - 0.00014922170307124307 * t443;
        let t811 = piecewise3(t2, 0.0, -t6 * t70 * t485 / 8.0 - 3.0 / 8.0 * t6 * t19 * t806);
        let tv4rhosigma30 = 2.0 * rho[ip] * t811 + 2.0 * t489;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t817 = 1.0 / t30 / t382 * t313 * t468;
        let t820 = t459 * t315;
        let t856 = 5.786247277743632e-35 * t817 * t530 + 6.30818556216682e-25 * t820 * t300 + 7.578959922212279e-15 * t243 * t43 + 2.745149338076219e-31 * t770 * t817 + 4.2753867401248904e-21 * t465 * t820 + 3.130174300633274e-41 * t587 * t817 - 2.859889209340841e-48 * t552 * t523 / t18 / t295 / t146 / t48 * t37 * t560 - 2.4052066699955267e-37 * t314 * t294 * t470 - 3.2851808610397184e-18 * t155 * t159 * t145 * t251 - 9.59334331382237e-10 * t85 * t41 * sigma[ip] * t115 - 2.7807943519583713e-06 * t582 / t30 / t247 * t587;
        let t860 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t856);
        let tv4sigma40 = 2.0 * rho[ip] * t860;
        v4sigma4[ip] += tv4sigma40;
    }
}
