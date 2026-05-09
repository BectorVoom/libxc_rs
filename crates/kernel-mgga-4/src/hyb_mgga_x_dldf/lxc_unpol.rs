//! HYB_MGGA_X_DLDF lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 45 shared lines across all orders.
//! Delta: 86 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn hyb_mgga_x_dldf_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho3lapl: &mut Array<f64>,
    v4rho3tau: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rho2sigmalapl: &mut Array<f64>,
    v4rho2sigmatau: &mut Array<f64>,
    v4rho2lapl2: &mut Array<f64>,
    v4rho2lapltau: &mut Array<f64>,
    v4rho2tau2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4rhosigma2lapl: &mut Array<f64>,
    v4rhosigma2tau: &mut Array<f64>,
    v4rhosigmalapl2: &mut Array<f64>,
    v4rhosigmalapltau: &mut Array<f64>,
    v4rhosigmatau2: &mut Array<f64>,
    v4rholapl3: &mut Array<f64>,
    v4rholapl2tau: &mut Array<f64>,
    v4rholapltau2: &mut Array<f64>,
    v4rhotau3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    v4sigma3lapl: &mut Array<f64>,
    v4sigma3tau: &mut Array<f64>,
    v4sigma2lapl2: &mut Array<f64>,
    v4sigma2lapltau: &mut Array<f64>,
    v4sigma2tau2: &mut Array<f64>,
    v4sigmalapl3: &mut Array<f64>,
    v4sigmalapl2tau: &mut Array<f64>,
    v4sigmalapltau2: &mut Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (45 lines) ---
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
        // --- vxc delta (25 lines) ---
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
        // --- fxc delta (60 lines) ---
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
        // --- kxc delta (90 lines) ---
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
        // --- lxc delta (this level) (86 lines) ---
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
        let t668 = 0.17579086222222222222e4 * t609 * t58 + 0.10495775318518518518e3 * t97 * t614 + 0.25053404444444444445e1 * t102 * t614 - 0.15198090666666666666e4 * t187 * t621 + 0.41048620148148148148e2 * t90 * t614 - 0.31263021155555555556e4 * t194 * t621 + 0.13104740651851851852e3 * t94 * t614 - 0.73204815555555555555e4 * t630 * t633 - 0.11041477777777777778e5 * t636 * t633 - 0.54776188888888888888e4 * t639 * t633 - 0.21352333333333333333e3 * t644 * t633 - 0.18980494459259259259e4 * t199 * t621 - 0.55312711111111111112e2 * t206 * t621 + 0.1245362637037037037e2 * t42 * t613 * t48 - 0.23624368592592592592e3 * t180 * t620 * t53 + 0.16058256e5 * t385 * t609 + 0.85689688888888888888e4 * t388 * t609 + 0.292832e3 * t393 * t609 - 0.15430993333333333333e4 * t632 * t63 * t25 + 0.95400277333333333333e4 * t382 * t609;
        let t685 = 1.0 / t335 / t34;
        let t688 = t164 * t164;
        let t690 = t23 * t25;
        let t716 = 0.82641543335807778634e-3 * t334 * t338 * t105 - 0.42698130723500685627e-2 * t16 / t28 / t598 * t339 + 0.97508417361787415355e-1 * t16 * t175 * t37 * t66 - 0.98727272578809758046e-1 * t16 * t38 * t668 - 0.13163636343841301073e0 * t16 * t72 * t396 + 0.13163636343841301073e0 * t16 * t141 * t211 - 0.14626262604268112303e0 * t16 * t309 * t105 + 0.32240559641063121574e-4 * t16 / t17 / t320 / t221 * t685 * t688 * t66 * t690 + 0.85977045175953682388e-1 * t160 * t163 * t165 * t211 + 0.36730523001135396825e0 * t82 * t23 * t26 * t396 - 0.57318030117302454925e0 * t323 * t343 - 0.16528735350510928572e1 * t149 * t347 - 0.62237830640812755732e1 * t16 * t356 * t80 * t84 + 0.46933446057006340388e1 * t314 * t171 + 0.1329459865220765274e1 * t16 * t608 * t158 * t167;
        let t717 = piecewise3(t3, 0.0, t716);
        let tv4rho40 = 2.0 * rho[ip] * t717 + 8.0 * t401;
        v4rho4[ip] += tv4rho40;
        let t742 = t336 * t105;
        let t746 = t24 * t211;
        let t756 = piecewise3(t3, 0.0, 0.11605824976053198303e1 * t314 * t118 - 0.35525237416453084043e0 * t323 * t227 - 0.80348019064983680555e0 * t149 * t231 + 0.13945760437917562645e-2 * t334 * t416 + 0.13433913308742762873e0 * t160 * t421 + 0.24104405719495104167e0 * t82 * t425 - 0.12090209865398670591e-4 * t16 / t17 / t320 / t181 * t685 * t66 * t337 * t690 - 0.23242934063195937742e-3 * t414 * t742 * t164 - 0.16120695970491315448e-1 * t224 * t163 * t746 * sigma[ip] - 0.34434865313564434524e-1 * t116 * t23 * t25 * t396);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t756 + 6.0 * t429;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = 0.0;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t786 = t321 * t179;
        let t802 = t25 * t351;
        let t806 = t413 * t375 * t25;
        let t824 = t24 * t356 * tau[ip];
        let t835 = -0.465076352e4 * t382 * t786 - 0.78283998e4 * t385 * t786 - 0.41773723333333333333e4 * t388 * t786 - 0.1427556e3 * t393 * t786 + 0.8546462755555555556e2 * tau[ip] * t24 * t379 + 0.92585959999999999999e3 * t413 * t63 * t375 * t25 - 0.26686342222222222222e1 * t802 * t48 + 0.43922889333333333333e4 * t630 * t806 + 0.66248866666666666666e4 * t636 * t806 + 0.32865713333333333333e4 * t639 * t806 + 0.128114e3 * t644 * t806 - 0.87961328888888888888e1 * t90 * t802 - 0.28081587111111111111e2 * t94 * t802 - 0.22490947111111111111e2 * t97 * t802 - 0.53685866666666666667e0 * t102 * t802 + 0.11309857653333333334e4 * t194 * t824 + 0.68664729955555555555e3 * t199 * t824 + 0.20010186666666666667e2 * t206 * t824 + 0.54981328e3 * t187 * t824 - 0.85698045333333333334e3 * t786 * t58;
        let t840 = piecewise3(t3, 0.0, -0.36565656510670280758e-1 * t16 * t309 * t134 + 0.11733361514251585097e1 * t314 * t242 + 0.65818181719206505364e-1 * t16 * t141 * t269 - 0.14329507529325613731e0 * t323 * t442 - 0.82643676752554642858e0 * t149 * t446 - 0.98727272578809758046e-1 * t16 * t72 * t484 + 0.20660385833951944658e-3 * t334 * t338 * t134 + 0.42988522587976841194e-1 * t160 * t163 * t165 * t269 + 0.27547892250851547619e0 * t82 * t23 * t26 * t484 - 0.98727272578809758046e-1 * t16 * t38 * t835);
        let tv4rho3tau0 = 2.0 * rho[ip] * t840 + 6.0 * t489;
        v4rho3tau[ip] += tv4rho3tau0;
        let t866 = piecewise3(t3, 0.0, 0.6045260988934243293e-1 * t160 * t280 - 0.39706679024626393641e-3 * t414 * t497 - 0.2015086996311414431e-1 * t224 * t500 + 0.45338286995245014716e-5 * t16 / t17 / t320 / t145 * t685 * t66 * t164 * t690 + 0.58107335157989844354e-4 * t496 * t742 * sigma[ip] + 0.2015086996311414431e-2 * t279 * t163 * t746);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t866 + 4.0 * t504;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let t876 = t336 * t134;
        let t880 = t24 * t269;
        let t890 = piecewise3(t3, 0.0, -0.26782673021661226852e0 * t149 * t286 + 0.44779711029142542911e-1 * t160 * t511 + 0.16069603812996736111e0 * t82 * t515 - 0.77476446877319792471e-4 * t414 * t876 * t164 - 0.10747130646994210299e-1 * t224 * t163 * t880 * sigma[ip] - 0.34434865313564434524e-1 * t116 * t23 * t25 * t484);
        let tv4rho2sigmatau0 = 2.0 * rho[ip] * t890 + 4.0 * t519;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let tv4rho2lapl20 = 0.0;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let t913 = t495 * t179 * t25;
        let t918 = t156 * tau[ip];
        let t945 = 0.36916081066666666667e3 * t468 * tau[ip] - 0.19719428e4 * t639 * t913 - 0.16328807733333333333e3 * t199 * t449 + 0.6149472e2 * t393 * t918 - 0.768684e2 * t644 * t913 - 0.475852e1 * t206 * t449 + 0.2003405824e4 * t382 * t918 - 0.263537336e4 * t630 * t913 - 0.13074828e3 * t187 * t449 + 0.337223376e4 * t385 * t918 - 0.3974932e4 * t636 * t913 - 0.268953932e3 * t194 * t449 + 0.17994834666666666667e4 * t388 * t918 - 0.20323905333333333333e2 * t449 * t53 - 0.55551576e3 * t495 * t63 * t179 * t25;
        let t950 = piecewise3(t3, 0.0, 0.21939393906402168455e-1 * t16 * t141 * t302 - 0.27547892250851547619e0 * t149 * t526 - 0.65818181719206505364e-1 * t16 * t72 * t551 + 0.14329507529325613731e-1 * t160 * t163 * t165 * t302 + 0.18365261500567698413e0 * t82 * t23 * t26 * t551 - 0.98727272578809758046e-1 * t16 * t38 * t945);
        let tv4rho2tau20 = 2.0 * rho[ip] * t950 + 4.0 * t556;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t967 = piecewise3(t3, 0.0, 0.83529294289610401256e-4 * t16 * t495 * t336 * t66 - 0.17001857623216880518e-5 * t16 / t17 / t598 * t685 * t84 - 0.10895125342123095816e-4 * t16 * t561 * t105);
        let tv4rhosigma30 = 2.0 * rho[ip] * t967 + 2.0 * t565;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4rhosigma2lapl0 = 0.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let t979 = piecewise3(t3, 0.0, -0.10075434981557072155e-1 * t224 * t567 + 0.29053667578994922177e-4 * t496 * t876 * sigma[ip] + 0.2015086996311414431e-2 * t279 * t163 * t880);
        let tv4rhosigma2tau0 = 2.0 * rho[ip] * t979 + 2.0 * t570;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let tv4rhosigmalapl20 = 0.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let t984 = t24 * t302;
        let t994 = piecewise3(t3, 0.0, 0.80348019064983680556e-1 * t82 * t573 - 0.53735653234971051493e-2 * t224 * t163 * t984 * sigma[ip] - 0.34434865313564434524e-1 * t116 * t23 * t25 * t551);
        let tv4rhosigmatau20 = 2.0 * rho[ip] * t994 + 2.0 * t576;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let t1009 = t560 * tau[ip] * t25;
        let t1031 = piecewise3(t3, 0.0, -0.32909090859603252682e-1 * t16 * t72 * t588 + 0.91826307502838492063e-1 * t82 * t23 * t26 * t588 - 0.98727272578809758046e-1 * t16 * t38 * (-0.118658832e3 * t531 + 0.333309456e3 * t560 * t63 * t42 + 0.1581224016e4 * t630 * t1009 - 0.643951872e3 * t382 * t222 + 0.23849592e4 * t636 * t1009 - 0.108393228e4 * t385 * t222 + 0.118316568e4 * t639 * t1009 - 0.5784054e3 * t388 * t222 + 0.4612104e2 * t644 * t1009 - 0.1976616e2 * t393 * t222));
        let tv4rhotau30 = 2.0 * rho[ip] * t1031 + 2.0 * t592;
        v4rhotau3[ip] += tv4rhotau30;
        let t1040 = piecewise3(t3, 0.0, 0.63756966087063301944e-6 * t16 / t17 / t331 * t685 * t118);
        let tv4sigma40 = 2.0 * rho[ip] * t1040;
        v4sigma4[ip] += tv4sigma40;
        let tv4sigma3lapl0 = 0.0;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let t1045 = piecewise3(t3, 0.0, -0.10895125342123095816e-4 * t16 * t561 * t134);
        let tv4sigma3tau0 = 2.0 * rho[ip] * t1045;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let tv4sigma2lapl20 = 0.0;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let t1050 = piecewise3(t3, 0.0, 0.2015086996311414431e-2 * t279 * t163 * t984);
        let tv4sigma2tau20 = 2.0 * rho[ip] * t1050;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let tv4sigmalapl30 = 0.0;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let t1056 = piecewise3(t3, 0.0, -0.34434865313564434524e-1 * t116 * t23 * t25 * t588);
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
        let t1076 = piecewise3(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * (-0.1999856736e3 * t1059 * t63 * t25 - 0.9487344096e3 * t630 * t1063 - 0.143097552e4 * t636 * t1063 - 0.709899408e3 * t639 * t1063 - 0.27672624e2 * t644 * t1063));
        let tv4tau40 = 2.0 * rho[ip] * t1076;
        v4tau4[ip] += tv4tau40;
    }
}
