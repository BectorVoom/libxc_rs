//! HYB_MGGA_X_DLDF kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_dldf_kxc_unpol(
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
        let t34 = 0.48827323e1 + 0.146297e-1 * t23 * t26 * t30;
        let t37 = 0.58827323e1 - 0.2384107471346329e2 / t34;
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
        let t66 = 1.0 - 0.1637571e0 * t46 * t48 - 0.1880028e0 * t51 * t53 - 0.4490609e0 * t56 * t58 - 0.82359e-2 * t61 * t63;
        let t70 = piecewise3(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * t66);
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
        let t105 = -0.2729285e0 * t42 * t30 * t48 - 0.8996045e0 * t90 * t91 - 0.28719805e1 * t94 * t91 - 0.23002105e1 * t97 * t91 - 0.54906e-1 * t102 * t91;
        let t110 = piecewise3(t3, 0.0, -0.32909090859603252682e-1 * t16 * t72 * t66 + 0.91826307502838492063e-1 * t82 * t84 - 0.98727272578809758046e-1 * t16 * t38 * t105);
        let tvrho0 = 2.0 * rho[ip] * t110 + 2.0 * t70;
        vrho[ip] += tvrho0;
        let t116 = t16 / t17 / t27 * t80;
        let t118 = t23 * t25 * t66;
        let t121 = piecewise3(t3, 0.0, -0.34434865313564434524e-1 * t116 * t118);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t123 = t25 * t44;
        let t134 = 0.1637571e0 * t123 * t48 + 0.5397627e0 * t90 * t123 + 0.17231883e1 * t94 * t123 + 0.13801263e1 * t97 * t123 + 0.329436e-1 * t102 * t123;
        let t138 = piecewise3(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * t134);
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
        let t211 = 0.72780933333333333333e0 * t42 * t175 * t48 - 0.39084433333333333334e1 * t180 * t183 * t53 - 0.251439e2 * t187 * t188 + 0.23989453333333333333e1 * t90 * t191 - 0.5172191e2 * t194 * t188 + 0.76586146666666666667e1 * t94 * t191 - 0.31401553333333333333e2 * t199 * t188 + 0.61338946666666666667e1 * t97 * t191 - 0.9151e0 * t206 * t188 + 0.146416e0 * t102 * t191;
        let t216 = piecewise3(t3, 0.0, 0.21939393906402168455e-1 * t16 * t141 * t66 - 0.27547892250851547619e0 * t149 * t84 - 0.65818181719206505364e-1 * t16 * t72 * t105 + 0.14329507529325613731e-1 * t160 * t167 + 0.18365261500567698413e0 * t82 * t171 - 0.98727272578809758046e-1 * t16 * t38 * t211);
        let tv2rho20 = 2.0 * rho[ip] * t216 + 4.0 * t110;
        v2rho2[ip] += tv2rho20;
        let t221 = t145 * t27;
        let t222 = 1.0 / t221;
        let t224 = t16 * t222 * t158;
        let t225 = t24 * t66;
        let t227 = t163 * t225 * sigma[ip];
        let t231 = t23 * t25 * t105;
        let t235 = piecewise3(t3, 0.0, 0.80348019064983680556e-1 * t82 * t118 - 0.53735653234971051493e-2 * t224 * t227 - 0.34434865313564434524e-1 * t116 * t231);
        let tv2rhosigma0 = 2.0 * rho[ip] * t235 + 2.0 * t121;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t242 = t23 * t26 * t134;
        let t245 = t25 * t30;
        let t248 = t24 * t147;
        let t249 = t53 * tau[ip];
        let t252 = t248 * tau[ip];
        let t269 = -0.2729285e0 * t245 * t48 + 0.2345066e1 * t248 * t249 + 0.1508634e2 * t187 * t252 - 0.8996045e0 * t90 * t245 + 0.31033146e2 * t194 * t252 - 0.28719805e1 * t94 * t245 + 0.18840932e2 * t199 * t252 - 0.23002105e1 * t97 * t245 + 0.54906e0 * t206 * t252 - 0.54906e-1 * t102 * t245;
        let t274 = piecewise3(t3, 0.0, -0.32909090859603252682e-1 * t16 * t72 * t134 + 0.91826307502838492063e-1 * t82 * t242 - 0.98727272578809758046e-1 * t16 * t38 * t269);
        let tv2rhotau0 = 2.0 * rho[ip] * t274 + 2.0 * t138;
        v2rhotau[ip] += tv2rhotau0;
        let t277 = 1.0 / t181;
        let t279 = t16 * t277 * t158;
        let t280 = t163 * t225;
        let t283 = piecewise3(t3, 0.0, 0.2015086996311414431e-2 * t279 * t280);
        let tv2sigma20 = 2.0 * rho[ip] * t283;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t286 = t23 * t25 * t134;
        let t289 = piecewise3(t3, 0.0, -0.34434865313564434524e-1 * t116 * t286);
        let tv2sigmatau0 = 2.0 * rho[ip] * t289;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t291 = t24 * t78;
        let t302 = -0.14070396e1 * t291 * t53 - 0.9051804e1 * t187 * t291 - 0.186198876e2 * t194 * t291 - 0.113045592e2 * t199 * t291 - 0.329436e0 * t206 * t291;
        let t306 = piecewise3(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * t302);
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
        let t396 = -0.53685866666666666667e0 * t102 * t352 + 0.2011512e3 * t187 * t357 - 0.87961328888888888888e1 * t90 * t352 + 0.41377528000000000001e3 * t194 * t357 - 0.28081587111111111111e2 * t94 * t352 + 0.25121242666666666666e3 * t199 * t357 - 0.22490947111111111111e2 * t97 * t352 + 0.73208e1 * t206 * t357 - 0.26686342222222222222e1 * t42 * t351 * t48 - 0.10986928888888888889e3 * t376 * t58 + 0.31267546666666666666e2 * t180 * t379 - 0.59625173333333333333e3 * t382 * t376 - 0.1003641e4 * t385 * t376 - 0.53556055555555555555e3 * t388 * t376 - 0.18302e2 * t393 * t376;
        let t401 = piecewise3(t3, 0.0, -0.36565656510670280758e-1 * t16 * t309 * t66 + 0.11733361514251585097e1 * t314 * t84 + 0.65818181719206505364e-1 * t16 * t141 * t105 - 0.14329507529325613731e0 * t323 * t167 - 0.82643676752554642858e0 * t149 * t171 - 0.98727272578809758046e-1 * t16 * t72 * t211 + 0.20660385833951944658e-3 * t334 * t339 + 0.42988522587976841194e-1 * t160 * t343 + 0.27547892250851547619e0 * t82 * t347 - 0.98727272578809758046e-1 * t16 * t38 * t396);
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
        let t429 = piecewise3(t3, 0.0, -0.26782673021661226852e0 * t149 * t118 + 0.44779711029142542911e-1 * t160 * t227 + 0.16069603812996736111e0 * t82 * t231 - 0.77476446877319792471e-4 * t414 * t416 - 0.10747130646994210299e-1 * t224 * t421 - 0.34434865313564434524e-1 * t116 * t425);
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
        let t484 = -0.9051804e2 * t187 * t450 - 0.186198876e3 * t194 * t450 - 0.113045592e3 * t199 * t450 - 0.329436e1 * t206 * t450 + 0.23989453333333333333e1 * t90 * t459 + 0.76586146666666666667e1 * t94 * t459 + 0.61338946666666666667e1 * t97 * t459 + 0.146416e0 * t102 * t459 + 0.65921573333333333333e2 * t468 * t179 + 0.72780933333333333333e0 * t459 * t48 - 0.14070396000000000001e2 * t449 * t249 + 0.35775104e3 * t382 * t475 + 0.6021846e3 * t385 * t475 + 0.32133633333333333333e3 * t388 * t475 + 0.109812e2 * t393 * t475;
        let t489 = piecewise3(t3, 0.0, 0.21939393906402168455e-1 * t16 * t141 * t134 - 0.27547892250851547619e0 * t149 * t242 - 0.65818181719206505364e-1 * t16 * t72 * t269 + 0.14329507529325613731e-1 * t160 * t442 + 0.18365261500567698413e0 * t82 * t446 - 0.98727272578809758046e-1 * t16 * t38 * t484);
        let tv3rho2tau0 = 2.0 * rho[ip] * t489 + 4.0 * t274;
        v3rho2tau[ip] += tv3rho2tau0;
        let t495 = 1.0 / t28 / t320;
        let t496 = t16 * t495;
        let t497 = t415 * sigma[ip];
        let t500 = t163 * t419;
        let t504 = piecewise3(t3, 0.0, -0.10075434981557072155e-1 * t224 * t280 + 0.29053667578994922177e-4 * t496 * t497 + 0.2015086996311414431e-2 * t279 * t500);
        let tv3rhosigma20 = 2.0 * rho[ip] * t504 + 2.0 * t283;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t509 = t24 * t134;
        let t511 = t163 * t509 * sigma[ip];
        let t515 = t23 * t25 * t269;
        let t519 = piecewise3(t3, 0.0, 0.80348019064983680556e-1 * t82 * t286 - 0.53735653234971051493e-2 * t224 * t511 - 0.34434865313564434524e-1 * t116 * t515);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t519 + 2.0 * t289;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t526 = t23 * t26 * t302;
        let t531 = t222 * t58;
        let t534 = t222 * tau[ip];
        let t551 = 0.4690132e1 * t248 * t53 - 0.39552944e2 * t531 * tau[ip] - 0.214650624e3 * t382 * t534 + 0.3017268e2 * t187 * t248 - 0.36131076e3 * t385 * t534 + 0.62066292e2 * t194 * t248 - 0.1928018e3 * t388 * t534 + 0.37681864e2 * t199 * t248 - 0.658872e1 * t393 * t534 + 0.109812e1 * t206 * t248;
        let t556 = piecewise3(t3, 0.0, -0.32909090859603252682e-1 * t16 * t72 * t302 + 0.91826307502838492063e-1 * t82 * t526 - 0.98727272578809758046e-1 * t16 * t38 * t551);
        let tv3rhotau20 = 2.0 * rho[ip] * t556 + 2.0 * t306;
        v3rhotau2[ip] += tv3rhotau20;
        let t560 = 1.0 / t28 / t155;
        let t561 = t560 * t336;
        let t565 = piecewise3(t3, 0.0, -0.10895125342123095816e-4 * t16 * t561 * t66);
        let tv3sigma30 = 2.0 * rho[ip] * t565;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let t567 = t163 * t509;
        let t570 = piecewise3(t3, 0.0, 0.2015086996311414431e-2 * t279 * t567);
        let tv3sigma2tau0 = 2.0 * rho[ip] * t570;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let t573 = t23 * t25 * t302;
        let t576 = piecewise3(t3, 0.0, -0.34434865313564434524e-1 * t116 * t573);
        let tv3sigmatau20 = 2.0 * rho[ip] * t576;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let t588 = 0.237317664e2 * t277 * t58 + 0.1287903744e3 * t382 * t277 + 0.216786456e3 * t385 * t277 + 0.11568108e3 * t388 * t277 + 0.3953232e1 * t393 * t277;
        let t592 = piecewise3(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * t588);
        let tv3tau30 = 2.0 * rho[ip] * t592;
        v3tau3[ip] += tv3tau30;
    }
}
