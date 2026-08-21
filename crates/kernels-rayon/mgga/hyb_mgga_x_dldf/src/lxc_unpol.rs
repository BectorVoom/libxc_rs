//! HYB_MGGA_X_DLDF lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_dldf_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho3lapl: &mut [f64],
    v4rho3tau: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rho2sigmalapl: &mut [f64],
    v4rho2sigmatau: &mut [f64],
    v4rho2lapl2: &mut [f64],
    v4rho2lapltau: &mut [f64],
    v4rho2tau2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4rhosigma2lapl: &mut [f64],
    v4rhosigma2tau: &mut [f64],
    v4rhosigmalapl2: &mut [f64],
    v4rhosigmalapltau: &mut [f64],
    v4rhosigmatau2: &mut [f64],
    v4rholapl3: &mut [f64],
    v4rholapl2tau: &mut [f64],
    v4rholapltau2: &mut [f64],
    v4rhotau3: &mut [f64],
    v4sigma4: &mut [f64],
    v4sigma3lapl: &mut [f64],
    v4sigma3tau: &mut [f64],
    v4sigma2lapl2: &mut [f64],
    v4sigma2lapltau: &mut [f64],
    v4sigma2tau2: &mut [f64],
    v4sigmalapl3: &mut [f64],
    v4sigmalapl2tau: &mut [f64],
    v4sigmalapltau2: &mut [f64],
    v4sigmatau3: &mut [f64],
    v4lapl4: &mut [f64],
    v4lapl3tau: &mut [f64],
    v4lapl2tau2: &mut [f64],
    v4lapltau3: &mut [f64],
    v4tau4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = pow_1_3(zeta_threshold);
        let t13 = pow_1_3(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = t4 * t15;
        let t17 = pow_1_3(rho[ip]);
        let t18 = M_CBRT6;
        let t19 = M_PI * M_PI;
        let t20 = pow_1_3(t19);
        let t21 = t20 * t20;
        let t23 = t18 / t21;
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = sigma[ip] * t25;
        let t27 = rho[ip] * rho[ip];
        let t28 = t17 * t17;
        let t30 = 1.0 / t28 / t27;
        let t34 = 4.8827323 + 0.0146297 * t23 * t26 * t30;
        let t37 = 5.8827323 - 23.84107471346329 / t34;
        let t38 = t17 * t37;
        let t39 = t18 * t18;
        let t41 = 3.0 / 10.0 * t39 * t21;
        let t42 = tau[ip] * t25;
        let t44 = 1.0 / t28 / rho[ip];
        let t45 = t42 * t44;
        let t46 = t41 - t45;
        let t47 = t41 + t45;
        let t48 = 1.0 / t47;
        let t51 = t46 * t46;
        let t52 = t47 * t47;
        let t53 = 1.0 / t52;
        let t56 = t51 * t46;
        let t57 = t52 * t47;
        let t58 = 1.0 / t57;
        let t61 = t51 * t51;
        let t62 = t52 * t52;
        let t63 = 1.0 / t62;
        let t66 = 1.0 - 0.1637571 * t46 * t48 - 0.1880028 * t51 * t53 - 0.4490609 * t56 * t58 - 0.0082359 * t61 * t63;
        let t70 = piecewise3(t3, 0.0, -0.09872727257880975 * t16 * t38 * t66);
        let tzk0 = 2.0 * t70;
        zk[ip] += tzk0;
        let t72 = 1.0 / t28 * t37;
        let t76 = t27 * rho[ip];
        let t78 = 1.0 / t17 / t76;
        let t79 = t34 * t34;
        let t80 = 1.0 / t79;
        let t82 = t16 * t78 * t80;
        let t84 = t23 * t26 * t66;
        let t90 = t46 * t53;
        let t91 = t42 * t30;
        let t94 = t51 * t58;
        let t97 = t56 * t63;
        let t101 = 1.0 / t62 / t47;
        let t102 = t61 * t101;
        let t105 = -0.2729285 * t42 * t30 * t48 - 0.8996045 * t90 * t91 - 2.8719805 * t94 * t91 - 2.3002105 * t97 * t91 - 0.054906 * t102 * t91;
        let t110 = piecewise3(t3, 0.0, -0.03290909085960325 * t16 * t72 * t66 + 0.09182630750283849 * t82 * t84 - 0.09872727257880975 * t16 * t38 * t105);
        let tvrho0 = 2.0 * rho[ip] * t110 + 2.0 * t70;
        vrho[ip] += tvrho0;
        let t116 = t16 / t17 / t27 * t80;
        let t118 = t23 * t25 * t66;
        let t121 = piecewise3(t3, 0.0, -0.03443486531356443 * t116 * t118);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t123 = t25 * t44;
        let t134 = 0.1637571 * t123 * t48 + 0.5397627 * t90 * t123 + 1.7231883 * t94 * t123 + 1.3801263 * t97 * t123 + 0.0329436 * t102 * t123;
        let t138 = piecewise3(t3, 0.0, -0.09872727257880975 * t16 * t38 * t134);
        let tvtau0 = 2.0 * rho[ip] * t138;
        vtau[ip] += tvtau0;
        let t141 = t44 * t37;
        let t145 = t27 * t27;
        let t147 = 1.0 / t17 / t145;
        let t149 = t16 * t147 * t80;
        let t155 = t145 * t76;
        let t156 = 1.0 / t155;
        let t158 = 1.0 / t79 / t34;
        let t160 = t16 * t156 * t158;
        let t163 = t39 / t20 / t19;
        let t164 = sigma[ip] * sigma[ip];
        let t165 = t164 * t24;
        let t167 = t163 * t165 * t66;
        let t171 = t23 * t26 * t105;
        let t175 = 1.0 / t28 / t76;
        let t179 = tau[ip] * tau[ip];
        let t180 = t179 * t24;
        let t181 = t145 * rho[ip];
        let t183 = 1.0 / t17 / t181;
        let t187 = t46 * t58;
        let t188 = t180 * t183;
        let t191 = t42 * t175;
        let t194 = t51 * t63;
        let t199 = t56 * t101;
        let t205 = 1.0 / t62 / t52;
        let t206 = t61 * t205;
        let t211 = 0.7278093333333333 * t42 * t175 * t48 - 3.9084433333333335 * t180 * t183 * t53 - 25.1439 * t187 * t188 + 2.3989453333333333 * t90 * t191 - 51.72191 * t194 * t188 + 7.658614666666667 * t94 * t191 - 31.401553333333332 * t199 * t188 + 6.1338946666666665 * t97 * t191 - 0.9151 * t206 * t188 + 0.146416 * t102 * t191;
        let t216 = piecewise3(t3, 0.0, 0.02193939390640217 * t16 * t141 * t66 - 0.27547892250851547 * t149 * t84 - 0.0658181817192065 * t16 * t72 * t105 + 0.014329507529325615 * t160 * t167 + 0.18365261500567698 * t82 * t171 - 0.09872727257880975 * t16 * t38 * t211);
        let tv2rho20 = 2.0 * rho[ip] * t216 + 4.0 * t110;
        v2rho2[ip] += tv2rho20;
        let t221 = t145 * t27;
        let t222 = 1.0 / t221;
        let t224 = t16 * t222 * t158;
        let t225 = t24 * t66;
        let t227 = t163 * t225 * sigma[ip];
        let t231 = t23 * t25 * t105;
        let t235 = piecewise3(t3, 0.0, 0.08034801906498368 * t82 * t118 - 0.005373565323497105 * t224 * t227 - 0.03443486531356443 * t116 * t231);
        let tv2rhosigma0 = 2.0 * rho[ip] * t235 + 2.0 * t121;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t242 = t23 * t26 * t134;
        let t245 = t25 * t30;
        let t248 = t24 * t147;
        let t249 = t53 * tau[ip];
        let t252 = t248 * tau[ip];
        let t269 = -0.2729285 * t245 * t48 + 2.345066 * t248 * t249 + 15.08634 * t187 * t252 - 0.8996045 * t90 * t245 + 31.033146 * t194 * t252 - 2.8719805 * t94 * t245 + 18.840932 * t199 * t252 - 2.3002105 * t97 * t245 + 0.54906 * t206 * t252 - 0.054906 * t102 * t245;
        let t274 = piecewise3(t3, 0.0, -0.03290909085960325 * t16 * t72 * t134 + 0.09182630750283849 * t82 * t242 - 0.09872727257880975 * t16 * t38 * t269);
        let tv2rhotau0 = 2.0 * rho[ip] * t274 + 2.0 * t138;
        v2rhotau[ip] += tv2rhotau0;
        let t277 = 1.0 / t181;
        let t279 = t16 * t277 * t158;
        let t280 = t163 * t225;
        let t283 = piecewise3(t3, 0.0, 0.0020150869963114146 * t279 * t280);
        let tv2sigma20 = 2.0 * rho[ip] * t283;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t286 = t23 * t25 * t134;
        let t289 = piecewise3(t3, 0.0, -0.03443486531356443 * t116 * t286);
        let tv2sigmatau0 = 2.0 * rho[ip] * t289;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t291 = t24 * t78;
        let t302 = -1.4070396 * t291 * t53 - 9.051804 * t187 * t291 - 18.6198876 * t194 * t291 - 11.3045592 * t199 * t291 - 0.329436 * t206 * t291;
        let t306 = piecewise3(t3, 0.0, -0.09872727257880975 * t16 * t38 * t302);
        let tv2tau20 = 2.0 * rho[ip] * t306;
        v2tau2[ip] += tv2tau20;
        let t309 = t30 * t37;
        let t314 = t16 * t183 * t80;
        let t320 = t145 * t145;
        let t321 = 1.0 / t320;
        let t323 = t16 * t321 * t158;
        let t331 = t320 * t27;
        let t333 = 1.0 / t28 / t331;
        let t334 = t16 * t333;
        let t335 = t79 * t79;
        let t336 = 1.0 / t335;
        let t337 = t164 * sigma[ip];
        let t338 = t336 * t337;
        let t339 = t338 * t66;
        let t343 = t163 * t165 * t105;
        let t347 = t23 * t26 * t211;
        let t351 = 1.0 / t28 / t145;
        let t352 = t42 * t351;
        let t356 = 1.0 / t17 / t221;
        let t357 = t180 * t356;
        let t375 = t179 * tau[ip];
        let t376 = t375 * t321;
        let t379 = t356 * t53;
        let t382 = t46 * t63;
        let t385 = t51 * t101;
        let t388 = t56 * t205;
        let t392 = 1.0 / t62 / t57;
        let t393 = t61 * t392;
        let t396 = -0.5368586666666667 * t102 * t352 + 201.1512 * t187 * t357 - 8.79613288888889 * t90 * t352 + 413.77528 * t194 * t357 - 28.081587111111112 * t94 * t352 + 251.21242666666666 * t199 * t357 - 22.49094711111111 * t97 * t352 + 7.3208 * t206 * t357 - 2.6686342222222224 * t42 * t351 * t48 - 109.86928888888889 * t376 * t58 + 31.267546666666668 * t180 * t379 - 596.2517333333333 * t382 * t376 - 1003.641 * t385 * t376 - 535.5605555555555 * t388 * t376 - 18.302 * t393 * t376;
        let t401 = piecewise3(t3, 0.0, -0.03656565651067028 * t16 * t309 * t66 + 1.1733361514251586 * t314 * t84 + 0.0658181817192065 * t16 * t141 * t105 - 0.14329507529325614 * t323 * t167 - 0.8264367675255464 * t149 * t171 - 0.09872727257880975 * t16 * t72 * t211 + 0.00020660385833951944 * t334 * t339 + 0.04298852258797684 * t160 * t343 + 0.27547892250851547 * t82 * t347 - 0.09872727257880975 * t16 * t38 * t396);
        let tv3rho30 = 2.0 * rho[ip] * t401 + 6.0 * t216;
        v3rho3[ip] += tv3rho30;
        let t411 = t320 * rho[ip];
        let t413 = 1.0 / t28 / t411;
        let t414 = t16 * t413;
        let t415 = t336 * t66;
        let t416 = t415 * t164;
        let t419 = t24 * t105;
        let t421 = t163 * t419 * sigma[ip];
        let t425 = t23 * t25 * t211;
        let t429 = piecewise3(t3, 0.0, -0.26782673021661224 * t149 * t118 + 0.04477971102914254 * t160 * t227 + 0.16069603812996736 * t82 * t231 - 7.747644687731979e-05 * t414 * t416 - 0.01074713064699421 * t224 * t421 - 0.03443486531356443 * t116 * t425);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t429 + 4.0 * t235;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t442 = t163 * t165 * t134;
        let t446 = t23 * t26 * t269;
        let t449 = t24 * t183;
        let t450 = t449 * tau[ip];
        let t459 = t25 * t175;
        let t468 = t156 * t58;
        let t475 = t156 * t179;
        let t484 = -90.51804 * t187 * t450 - 186.198876 * t194 * t450 - 113.045592 * t199 * t450 - 3.29436 * t206 * t450 + 2.3989453333333333 * t90 * t459 + 7.658614666666667 * t94 * t459 + 6.1338946666666665 * t97 * t459 + 0.146416 * t102 * t459 + 65.92157333333333 * t468 * t179 + 0.7278093333333333 * t459 * t48 - 14.070396 * t449 * t249 + 357.75104 * t382 * t475 + 602.1846 * t385 * t475 + 321.33633333333336 * t388 * t475 + 10.9812 * t393 * t475;
        let t489 = piecewise3(t3, 0.0, 0.02193939390640217 * t16 * t141 * t134 - 0.27547892250851547 * t149 * t242 - 0.0658181817192065 * t16 * t72 * t269 + 0.014329507529325615 * t160 * t442 + 0.18365261500567698 * t82 * t446 - 0.09872727257880975 * t16 * t38 * t484);
        let tv3rho2tau0 = 2.0 * rho[ip] * t489 + 4.0 * t274;
        v3rho2tau[ip] += tv3rho2tau0;
        let t495 = 1.0 / t28 / t320;
        let t496 = t16 * t495;
        let t497 = t415 * sigma[ip];
        let t500 = t163 * t419;
        let t504 = piecewise3(t3, 0.0, -0.010075434981557073 * t224 * t280 + 2.9053667578994923e-05 * t496 * t497 + 0.0020150869963114146 * t279 * t500);
        let tv3rhosigma20 = 2.0 * rho[ip] * t504 + 2.0 * t283;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t509 = t24 * t134;
        let t511 = t163 * t509 * sigma[ip];
        let t515 = t23 * t25 * t269;
        let t519 = piecewise3(t3, 0.0, 0.08034801906498368 * t82 * t286 - 0.005373565323497105 * t224 * t511 - 0.03443486531356443 * t116 * t515);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t519 + 2.0 * t289;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t526 = t23 * t26 * t302;
        let t531 = t222 * t58;
        let t534 = t222 * tau[ip];
        let t551 = 4.690132 * t248 * t53 - 39.552944 * t531 * tau[ip] - 214.650624 * t382 * t534 + 30.17268 * t187 * t248 - 361.31076 * t385 * t534 + 62.066292 * t194 * t248 - 192.8018 * t388 * t534 + 37.681864 * t199 * t248 - 6.58872 * t393 * t534 + 1.09812 * t206 * t248;
        let t556 = piecewise3(t3, 0.0, -0.03290909085960325 * t16 * t72 * t302 + 0.09182630750283849 * t82 * t526 - 0.09872727257880975 * t16 * t38 * t551);
        let tv3rhotau20 = 2.0 * rho[ip] * t556 + 2.0 * t306;
        v3rhotau2[ip] += tv3rhotau20;
        let t560 = 1.0 / t28 / t155;
        let t561 = t560 * t336;
        let t565 = piecewise3(t3, 0.0, -1.0895125342123096e-05 * t16 * t561 * t66);
        let tv3sigma30 = 2.0 * rho[ip] * t565;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let t567 = t163 * t509;
        let t570 = piecewise3(t3, 0.0, 0.0020150869963114146 * t279 * t567);
        let tv3sigma2tau0 = 2.0 * rho[ip] * t570;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let t573 = t23 * t25 * t302;
        let t576 = piecewise3(t3, 0.0, -0.03443486531356443 * t116 * t573);
        let tv3sigmatau20 = 2.0 * rho[ip] * t576;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let t588 = 23.7317664 * t277 * t58 + 128.7903744 * t382 * t277 + 216.786456 * t385 * t277 + 115.68108 * t388 * t277 + 3.953232 * t393 * t277;
        let t592 = piecewise3(t3, 0.0, -0.09872727257880975 * t16 * t38 * t588);
        let tv3tau30 = 2.0 * rho[ip] * t592;
        v3tau3[ip] += tv3tau30;
        let t598 = t320 * t76;
        let t608 = 1.0 / t411;
        let t609 = t375 * t608;
        let t613 = 1.0 / t28 / t181;
        let t614 = t42 * t613;
        let t620 = 1.0 / t17 / t155;
        let t621 = t180 * t620;
        let t630 = t46 * t101;
        let t631 = t179 * t179;
        let t632 = t631 * t333;
        let t633 = t632 * t25;
        let t636 = t51 * t205;
        let t639 = t56 * t392;
        let t642 = t62 * t62;
        let t644 = t61 / t642;
        let t668 = 1757.9086222222222 * t609 * t58 + 104.95775318518518 * t97 * t614 + 2.5053404444444443 * t102 * t614 - 1519.8090666666667 * t187 * t621 + 41.048620148148146 * t90 * t614 - 3126.3021155555557 * t194 * t621 + 131.04740651851853 * t94 * t614 - 7320.481555555555 * t630 * t633 - 11041.477777777778 * t636 * t633 - 5477.618888888889 * t639 * t633 - 213.52333333333334 * t644 * t633 - 1898.049445925926 * t199 * t621 - 55.31271111111111 * t206 * t621 + 12.45362637037037 * t42 * t613 * t48 - 236.24368592592592 * t180 * t620 * t53 + 16058.256 * t385 * t609 + 8568.968888888889 * t388 * t609 + 292.832 * t393 * t609 - 1543.0993333333333 * t632 * t63 * t25 + 9540.027733333332 * t382 * t609;
        let t685 = 1.0 / t335 / t34;
        let t688 = t164 * t164;
        let t690 = t23 * t25;
        let t716 = 0.0008264154333580778 * t334 * t338 * t105 - 0.0042698130723500686 * t16 / t28 / t598 * t339 + 0.09750841736178742 * t16 * t175 * t37 * t66 - 0.09872727257880975 * t16 * t38 * t668 - 0.131636363438413 * t16 * t72 * t396 + 0.131636363438413 * t16 * t141 * t211 - 0.14626262604268112 * t16 * t309 * t105 + 3.224055964106312e-05 * t16 / t17 / t320 / t221 * t685 * t688 * t66 * t690 + 0.08597704517595368 * t160 * t163 * t165 * t211 + 0.36730523001135396 * t82 * t23 * t26 * t396 - 0.5731803011730245 * t323 * t343 - 1.6528735350510928 * t149 * t347 - 6.2237830640812755 * t16 * t356 * t80 * t84 + 4.693344605700634 * t314 * t171 + 1.3294598652207652 * t16 * t608 * t158 * t167;
        let t717 = piecewise3(t3, 0.0, t716);
        let tv4rho40 = 2.0 * rho[ip] * t717 + 8.0 * t401;
        v4rho4[ip] += tv4rho40;
        let t742 = t336 * t105;
        let t746 = t24 * t211;
        let t756 = piecewise3(t3, 0.0, 1.1605824976053198 * t314 * t118 - 0.3552523741645308 * t323 * t227 - 0.8034801906498368 * t149 * t231 + 0.0013945760437917563 * t334 * t416 + 0.13433913308742762 * t160 * t421 + 0.24104405719495103 * t82 * t425 - 1.2090209865398671e-05 * t16 / t17 / t320 / t181 * t685 * t66 * t337 * t690 - 0.00023242934063195939 * t414 * t742 * t164 - 0.016120695970491317 * t224 * t163 * t746 * sigma[ip] - 0.03443486531356443 * t116 * t23 * t25 * t396);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t756 + 6.0 * t429;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = 0.0;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t786 = t321 * t179;
        let t802 = t25 * t351;
        let t806 = t413 * t375 * t25;
        let t824 = t24 * t356 * tau[ip];
        let t835 = -4650.76352 * t382 * t786 - 7828.3998 * t385 * t786 - 4177.372333333334 * t388 * t786 - 142.7556 * t393 * t786 + 85.46462755555555 * tau[ip] * t24 * t379 + 925.8596 * t413 * t63 * t375 * t25 - 2.6686342222222224 * t802 * t48 + 4392.288933333333 * t630 * t806 + 6624.886666666666 * t636 * t806 + 3286.5713333333333 * t639 * t806 + 128.114 * t644 * t806 - 8.79613288888889 * t90 * t802 - 28.081587111111112 * t94 * t802 - 22.49094711111111 * t97 * t802 - 0.5368586666666667 * t102 * t802 + 1130.9857653333333 * t194 * t824 + 686.6472995555556 * t199 * t824 + 20.010186666666666 * t206 * t824 + 549.81328 * t187 * t824 - 856.9804533333333 * t786 * t58;
        let t840 = piecewise3(t3, 0.0, -0.03656565651067028 * t16 * t309 * t134 + 1.1733361514251586 * t314 * t242 + 0.0658181817192065 * t16 * t141 * t269 - 0.14329507529325614 * t323 * t442 - 0.8264367675255464 * t149 * t446 - 0.09872727257880975 * t16 * t72 * t484 + 0.00020660385833951944 * t334 * t338 * t134 + 0.04298852258797684 * t160 * t163 * t165 * t269 + 0.27547892250851547 * t82 * t23 * t26 * t484 - 0.09872727257880975 * t16 * t38 * t835);
        let tv4rho3tau0 = 2.0 * rho[ip] * t840 + 6.0 * t489;
        v4rho3tau[ip] += tv4rho3tau0;
        let t866 = piecewise3(t3, 0.0, 0.06045260988934243 * t160 * t280 - 0.00039706679024626394 * t414 * t497 - 0.020150869963114146 * t224 * t500 + 4.533828699524501e-06 * t16 / t17 / t320 / t145 * t685 * t66 * t164 * t690 + 5.8107335157989847e-05 * t496 * t742 * sigma[ip] + 0.0020150869963114146 * t279 * t163 * t746);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t866 + 4.0 * t504;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let t876 = t336 * t134;
        let t880 = t24 * t269;
        let t890 = piecewise3(t3, 0.0, -0.26782673021661224 * t149 * t286 + 0.04477971102914254 * t160 * t511 + 0.16069603812996736 * t82 * t515 - 7.747644687731979e-05 * t414 * t876 * t164 - 0.01074713064699421 * t224 * t163 * t880 * sigma[ip] - 0.03443486531356443 * t116 * t23 * t25 * t484);
        let tv4rho2sigmatau0 = 2.0 * rho[ip] * t890 + 4.0 * t519;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let tv4rho2lapl20 = 0.0;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let t913 = t495 * t179 * t25;
        let t918 = t156 * tau[ip];
        let t945 = 369.1608106666667 * t468 * tau[ip] - 1971.9428 * t639 * t913 - 163.28807733333332 * t199 * t449 + 61.49472 * t393 * t918 - 76.8684 * t644 * t913 - 4.75852 * t206 * t449 + 2003.405824 * t382 * t918 - 2635.37336 * t630 * t913 - 130.74828 * t187 * t449 + 3372.23376 * t385 * t918 - 3974.932 * t636 * t913 - 268.953932 * t194 * t449 + 1799.4834666666666 * t388 * t918 - 20.323905333333332 * t449 * t53 - 555.51576 * t495 * t63 * t179 * t25;
        let t950 = piecewise3(t3, 0.0, 0.02193939390640217 * t16 * t141 * t302 - 0.27547892250851547 * t149 * t526 - 0.0658181817192065 * t16 * t72 * t551 + 0.014329507529325615 * t160 * t163 * t165 * t302 + 0.18365261500567698 * t82 * t23 * t26 * t551 - 0.09872727257880975 * t16 * t38 * t945);
        let tv4rho2tau20 = 2.0 * rho[ip] * t950 + 4.0 * t556;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t967 = piecewise3(t3, 0.0, 8.35292942896104e-05 * t16 * t495 * t336 * t66 - 1.7001857623216881e-06 * t16 / t17 / t598 * t685 * t84 - 1.0895125342123096e-05 * t16 * t561 * t105);
        let tv4rhosigma30 = 2.0 * rho[ip] * t967 + 2.0 * t565;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4rhosigma2lapl0 = 0.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let t979 = piecewise3(t3, 0.0, -0.010075434981557073 * t224 * t567 + 2.9053667578994923e-05 * t496 * t876 * sigma[ip] + 0.0020150869963114146 * t279 * t163 * t880);
        let tv4rhosigma2tau0 = 2.0 * rho[ip] * t979 + 2.0 * t570;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let tv4rhosigmalapl20 = 0.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let t984 = t24 * t302;
        let t994 = piecewise3(t3, 0.0, 0.08034801906498368 * t82 * t573 - 0.005373565323497105 * t224 * t163 * t984 * sigma[ip] - 0.03443486531356443 * t116 * t23 * t25 * t551);
        let tv4rhosigmatau20 = 2.0 * rho[ip] * t994 + 2.0 * t576;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let t1009 = t560 * tau[ip] * t25;
        let t1031 = piecewise3(t3, 0.0, -0.03290909085960325 * t16 * t72 * t588 + 0.09182630750283849 * t82 * t23 * t26 * t588 - 0.09872727257880975 * t16 * t38 * (-118.658832 * t531 + 333.309456 * t560 * t63 * t42 + 1581.224016 * t630 * t1009 - 643.951872 * t382 * t222 + 2384.9592 * t636 * t1009 - 1083.93228 * t385 * t222 + 1183.16568 * t639 * t1009 - 578.4054 * t388 * t222 + 46.12104 * t644 * t1009 - 19.76616 * t393 * t222));
        let tv4rhotau30 = 2.0 * rho[ip] * t1031 + 2.0 * t592;
        v4rhotau3[ip] += tv4rhotau30;
        let t1040 = piecewise3(t3, 0.0, 6.37569660870633e-07 * t16 / t17 / t331 * t685 * t118);
        let tv4sigma40 = 2.0 * rho[ip] * t1040;
        v4sigma4[ip] += tv4sigma40;
        let tv4sigma3lapl0 = 0.0;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let t1045 = piecewise3(t3, 0.0, -1.0895125342123096e-05 * t16 * t561 * t134);
        let tv4sigma3tau0 = 2.0 * rho[ip] * t1045;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let tv4sigma2lapl20 = 0.0;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let t1050 = piecewise3(t3, 0.0, 0.0020150869963114146 * t279 * t163 * t984);
        let tv4sigma2tau20 = 2.0 * rho[ip] * t1050;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let tv4sigmalapl30 = 0.0;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let t1056 = piecewise3(t3, 0.0, -0.03443486531356443 * t116 * t23 * t25 * t588);
        let tv4sigmatau30 = 2.0 * rho[ip] * t1056;
        v4sigmatau3[ip] += tv4sigmatau30;
        let tv4lapl40 = 0.0;
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let t1059 = 1.0 / t28 / t221;
        let t1063 = t1059 * t25;
        let t1076 = piecewise3(t3, 0.0, -0.09872727257880975 * t16 * t38 * (-199.9856736 * t1059 * t63 * t25 - 948.7344096 * t630 * t1063 - 1430.97552 * t636 * t1063 - 709.899408 * t639 * t1063 - 27.672624 * t644 * t1063));
        let tv4tau40 = 2.0 * rho[ip] * t1076;
        v4tau4[ip] += tv4tau40;
    }
}
