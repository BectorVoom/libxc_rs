//! MGGA_X_PKZB lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pkzb.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_pkzb_lxc_unpol(
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
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = t26 * t34;
        let t37 = tau[ip] * t28;
        let t39 = 1.0 / t31 / rho[ip];
        let t44 = t26 * t37 * t39 / 4.0 - 9.0 / 20.0 - t35 / 288.0;
        let t45 = t44 * t44;
        let t47 = t44 * t21;
        let t48 = t47 * t25;
        let t51 = t21 * t21;
        let t53 = 1.0 / t23 / t22;
        let t54 = t51 * t53;
        let t55 = sigma[ip] * sigma[ip];
        let t56 = t55 * t27;
        let t57 = t30 * t30;
        let t58 = t57 * rho[ip];
        let t60 = 1.0 / t19 / t58;
        let t64 = 0.804e0 + 5.0 / 972.0 * t35 + 146.0 / 2025.0 * t45 - 73.0 / 9720.0 * t48 * t34 + 0.45818468001825619316e-3 * t54 * t56 * t60;
        let t67 = 0.1804e1 - 0.646416e0 / t64;
        let t71 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t18 * t19 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        let t72 = 1.0 / t31;
        let t77 = t4 * t18;
        let t78 = t64 * t64;
        let t79 = 1.0 / t78;
        let t80 = t19 * t79;
        let t81 = t30 * rho[ip];
        let t83 = 1.0 / t31 / t81;
        let t84 = t29 * t83;
        let t85 = t26 * t84;
        let t91 = -5.0 / 12.0 * t26 * t37 * t33 + t85 / 108.0;
        let t94 = t91 * t21;
        let t95 = t94 * t25;
        let t100 = t57 * t30;
        let t102 = 1.0 / t19 / t100;
        let t106 = -10.0 / 729.0 * t85 + 292.0 / 2025.0 * t44 * t91 - 73.0 / 9720.0 * t95 * t34 + 73.0 / 3645.0 * t48 * t84 - 0.24436516267640330302e-2 * t54 * t56 * t102;
        let t111 = piecewise3(t3, 0.0, -t7 * t18 * t72 * t67 / 8.0 - 0.16551095363746320496e0 * t77 * t80 * t106);
        let tvrho0 = 2.0 * rho[ip] * t111 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t117 = t25 * t28;
        let t118 = t117 * t33;
        let t119 = t47 * t118;
        let t123 = t54 * t27 * t60 * sigma[ip];
        let t125 = 5.0 / 972.0 * t26 * t28 * t33 - 146.0 / 18225.0 * t119 + 0.96852413827153753492e-3 * t123;
        let t129 = piecewise3(t3, 0.0, -0.16551095363746320496e0 * t77 * t80 * t125);
        let tvsigma0 = 2.0 * rho[ip] * t129;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t131 = t117 * t39;
        let t140 = 73.0 / 2025.0 * t47 * t131 - 73.0 / 19440.0 * t54 * t27 / t19 / t57 * sigma[ip];
        let t144 = piecewise3(t3, 0.0, -0.16551095363746320496e0 * t77 * t80 * t140);
        let tvtau0 = 2.0 * rho[ip] * t144;
        vtau[ip] += tvtau0;
        let t151 = t72 * t79;
        let t156 = 1.0 / t78 / t64;
        let t157 = t19 * t156;
        let t158 = t106 * t106;
        let t163 = 1.0 / t31 / t57;
        let t164 = t29 * t163;
        let t165 = t26 * t164;
        let t167 = t91 * t91;
        let t173 = 10.0 / 9.0 * t26 * t37 * t83 - 11.0 / 324.0 * t165;
        let t176 = t173 * t21;
        let t177 = t176 * t25;
        let t184 = t57 * t81;
        let t186 = 1.0 / t19 / t184;
        let t190 = 110.0 / 2187.0 * t165 + 292.0 / 2025.0 * t167 + 292.0 / 2025.0 * t44 * t173 - 73.0 / 9720.0 * t177 * t34 + 146.0 / 3645.0 * t95 * t84 - 803.0 / 10935.0 * t48 * t164 + 0.15476460302838875858e-1 * t54 * t56 * t186;
        let t195 = piecewise3(t3, 0.0, t7 * t18 * t39 * t67 / 12.0 - 0.11034063575830880331e0 * t77 * t151 * t106 + 0.33102190727492640992e0 * t77 * t157 * t158 - 0.16551095363746320496e0 * t77 * t80 * t190);
        let tv2rho20 = 2.0 * rho[ip] * t195 + 4.0 * t111;
        v2rho2[ip] += tv2rho20;
        let t201 = t77 * t19;
        let t202 = t156 * t125;
        let t203 = t202 * t106;
        let t209 = t94 * t118;
        let t211 = t117 * t83;
        let t212 = t47 * t211;
        let t216 = t54 * t27 * t102 * sigma[ip];
        let t218 = -10.0 / 729.0 * t26 * t28 * t83 - 146.0 / 18225.0 * t209 + 1168.0 / 54675.0 * t212 - 0.51654620707815335196e-2 * t216;
        let t223 = piecewise3(t3, 0.0, -0.55170317879154401653e-1 * t77 * t151 * t125 + 0.33102190727492640992e0 * t201 * t203 - 0.16551095363746320496e0 * t77 * t80 * t218);
        let tv2rhosigma0 = 2.0 * rho[ip] * t223 + 2.0 * t129;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t229 = t156 * t140;
        let t230 = t229 * t106;
        let t237 = 73.0 / 2025.0 * t94 * t131 - 73.0 / 1215.0 * t119 + 949.0 / 58320.0 * t123;
        let t242 = piecewise3(t3, 0.0, -0.55170317879154401653e-1 * t77 * t151 * t140 + 0.33102190727492640992e0 * t201 * t230 - 0.16551095363746320496e0 * t77 * t80 * t237);
        let tv2rhotau0 = 2.0 * rho[ip] * t242 + 2.0 * t144;
        v2rhotau[ip] += tv2rhotau0;
        let t245 = t125 * t125;
        let t249 = 1.0 / t58;
        let t252 = t53 * t27;
        let t253 = t79 * t51 * t252;
        let t254 = t77 * t249 * t253;
        let t257 = piecewise3(t3, 0.0, 0.33102190727492640992e0 * t77 * t157 * t245 - 0.16950901996748250202e-3 * t254);
        let tv2sigma20 = 2.0 * rho[ip] * t257;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t259 = t229 * t125;
        let t262 = 1.0 / t57;
        let t264 = t77 * t262 * t253;
        let t267 = piecewise3(t3, 0.0, 0.33102190727492640992e0 * t201 * t259 + 0.66295196793057964127e-3 * t264);
        let tv2sigmatau0 = 2.0 * rho[ip] * t267;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t269 = t140 * t140;
        let t273 = 1.0 / t81;
        let t278 = piecewise3(t3, 0.0, 0.33102190727492640992e0 * t77 * t157 * t269 - 0.29832838556876083857e-2 * t77 * t273 * t253);
        let tv2tau20 = 2.0 * rho[ip] * t278;
        v2tau2[ip] += tv2tau20;
        let t285 = t39 * t79;
        let t289 = t72 * t156;
        let t296 = t78 * t78;
        let t297 = 1.0 / t296;
        let t298 = t19 * t297;
        let t299 = t158 * t106;
        let t303 = t156 * t106;
        let t304 = t303 * t190;
        let t308 = 1.0 / t31 / t58;
        let t309 = t29 * t308;
        let t310 = t26 * t309;
        let t318 = -110.0 / 27.0 * t26 * t37 * t163 + 77.0 / 486.0 * t310;
        let t321 = t318 * t21;
        let t322 = t321 * t25;
        let t331 = t57 * t57;
        let t333 = 1.0 / t19 / t331;
        let t337 = -1540.0 / 6561.0 * t310 + 292.0 / 675.0 * t91 * t173 + 292.0 / 2025.0 * t44 * t318 - 73.0 / 9720.0 * t322 * t34 + 73.0 / 1215.0 * t177 * t84 - 803.0 / 3645.0 * t95 * t164 + 11242.0 / 32805.0 * t48 * t309 - 0.11349404222081842296e0 * t54 * t56 * t333;
        let t342 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t18 * t33 * t67 + 0.11034063575830880331e0 * t77 * t285 * t106 + 0.33102190727492640993e0 * t77 * t289 * t158 - 0.16551095363746320496e0 * t77 * t151 * t190 - 0.99306572182477922976e0 * t77 * t298 * t299 + 0.99306572182477922976e0 * t201 * t304 - 0.16551095363746320496e0 * t77 * t80 * t337);
        let tv3rho30 = 2.0 * rho[ip] * t342 + 6.0 * t195;
        v3rho3[ip] += tv3rho30;
        let t349 = t77 * t72;
        let t355 = t297 * t125;
        let t356 = t355 * t158;
        let t359 = t156 * t218;
        let t360 = t359 * t106;
        let t363 = t202 * t190;
        let t369 = t176 * t118;
        let t371 = t94 * t211;
        let t373 = t117 * t163;
        let t374 = t47 * t373;
        let t378 = t54 * t27 * t186 * sigma[ip];
        let t380 = 110.0 / 2187.0 * t26 * t28 * t163 - 146.0 / 18225.0 * t369 + 2336.0 / 54675.0 * t371 - 12848.0 / 164025.0 * t374 + 0.32714593114949712291e-1 * t378;
        let t385 = piecewise3(t3, 0.0, 0.36780211919436267769e-1 * t77 * t285 * t125 + 0.22068127151661760662e0 * t349 * t203 - 0.11034063575830880331e0 * t77 * t151 * t218 - 0.99306572182477922976e0 * t201 * t356 + 0.66204381454985281984e0 * t201 * t360 + 0.33102190727492640992e0 * t201 * t363 - 0.16551095363746320496e0 * t77 * t80 * t380);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t385 + 4.0 * t223;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t397 = t297 * t140;
        let t398 = t397 * t158;
        let t401 = t156 * t237;
        let t402 = t401 * t106;
        let t405 = t229 * t190;
        let t413 = 73.0 / 2025.0 * t176 * t131 - 146.0 / 1215.0 * t209 + 584.0 / 3645.0 * t212 - 949.0 / 10935.0 * t216;
        let t418 = piecewise3(t3, 0.0, 0.36780211919436267769e-1 * t77 * t285 * t140 + 0.22068127151661760662e0 * t349 * t230 - 0.11034063575830880331e0 * t77 * t151 * t237 - 0.99306572182477922976e0 * t201 * t398 + 0.66204381454985281984e0 * t201 * t402 + 0.33102190727492640992e0 * t201 * t405 - 0.16551095363746320496e0 * t77 * t80 * t413);
        let tv3rho2tau0 = 2.0 * rho[ip] * t418 + 4.0 * t242;
        v3rho2tau[ip] += tv3rho2tau0;
        let t424 = t297 * t245;
        let t425 = t424 * t106;
        let t428 = t202 * t218;
        let t431 = 1.0 / t100;
        let t433 = t77 * t431 * t253;
        let t436 = t77 * t249 * t156;
        let t438 = t54 * t27 * t106;
        let t439 = t436 * t438;
        let t442 = piecewise3(t3, 0.0, 0.11034063575830880331e0 * t77 * t289 * t245 - 0.99306572182477922976e0 * t201 * t425 + 0.66204381454985281984e0 * t201 * t428 + 0.8475450998374125101e-3 * t433 + 0.33901803993496500404e-3 * t439);
        let tv3rhosigma20 = 2.0 * rho[ip] * t442 + 2.0 * t257;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t447 = t125 * t106;
        let t448 = t397 * t447;
        let t451 = t401 * t125;
        let t454 = t229 * t218;
        let t459 = t77 * t262 * t156;
        let t460 = t459 * t438;
        let t463 = piecewise3(t3, 0.0, 0.11034063575830880331e0 * t349 * t259 - 0.99306572182477922976e0 * t201 * t448 + 0.33102190727492640992e0 * t201 * t451 + 0.33102190727492640992e0 * t201 * t454 - 0.26518078717223185651e-2 * t254 - 0.13259039358611592825e-2 * t460);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t463 + 2.0 * t267;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t469 = t297 * t269;
        let t470 = t469 * t106;
        let t473 = t229 * t237;
        let t478 = t77 * t273 * t156;
        let t482 = piecewise3(t3, 0.0, 0.11034063575830880331e0 * t77 * t289 * t269 - 0.99306572182477922976e0 * t201 * t470 + 0.66204381454985281984e0 * t201 * t473 + 0.89498515670628251571e-2 * t264 + 0.59665677113752167714e-2 * t478 * t438);
        let tv3rhotau20 = 2.0 * rho[ip] * t482 + 2.0 * t278;
        v3rhotau2[ip] += tv3rhotau20;
        let t485 = t245 * t125;
        let t489 = t125 * t51;
        let t490 = t489 * t252;
        let t491 = t436 * t490;
        let t494 = piecewise3(t3, 0.0, -0.99306572182477922976e0 * t77 * t298 * t485 + 0.10170541198048950121e-2 * t491);
        let tv3sigma30 = 2.0 * rho[ip] * t494;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let t496 = t397 * t245;
        let t499 = t459 * t490;
        let t501 = t140 * t51;
        let t502 = t501 * t252;
        let t503 = t436 * t502;
        let t506 = piecewise3(t3, 0.0, -0.99306572182477922976e0 * t201 * t496 - 0.2651807871722318565e-2 * t499 + 0.33901803993496500404e-3 * t503);
        let tv3sigma2tau0 = 2.0 * rho[ip] * t506;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let t508 = t469 * t125;
        let t511 = t459 * t502;
        let t516 = piecewise3(t3, 0.0, -0.99306572182477922976e0 * t201 * t508 - 0.26518078717223185651e-2 * t511 + 0.59665677113752167714e-2 * t478 * t490);
        let tv3sigmatau20 = 2.0 * rho[ip] * t516;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let t518 = t269 * t140;
        let t525 = piecewise3(t3, 0.0, -0.99306572182477922976e0 * t77 * t298 * t518 + 0.17899703134125650314e-1 * t478 * t502);
        let tv3tau30 = 2.0 * rho[ip] * t525;
        v3tau3[ip] += tv3tau30;
        let t532 = t33 * t79;
        let t536 = t39 * t156;
        let t543 = t72 * t297;
        let t553 = 1.0 / t296 / t64;
        let t554 = t19 * t553;
        let t555 = t158 * t158;
        let t563 = t190 * t190;
        let t572 = t29 / t31 / t100;
        let t573 = t26 * t572;
        let t575 = t173 * t173;
        let t583 = 1540.0 / 81.0 * t26 * t37 * t308 - 1309.0 / 1458.0 * t573;
        let t600 = 1.0 / t19 / t331 / rho[ip];
        let t608 = 10.0 / 27.0 * t7 * t18 * t83 * t67 - 0.24520141279624178513e0 * t77 * t532 * t106 - 0.44136254303323521324e0 * t77 * t536 * t158 + 0.22068127151661760662e0 * t77 * t285 * t190 - 0.13240876290997056397e1 * t77 * t543 * t299 + 0.13240876290997056397e1 * t349 * t304 - 0.22068127151661760661e0 * t77 * t151 * t337 + 0.3972262887299116919e1 * t77 * t554 * t555 - 0.59583943309486753786e1 * t201 * t297 * t158 * t190 + 0.99306572182477922976e0 * t77 * t157 * t563 + 0.13240876290997056397e1 * t201 * t303 * t337 - 0.16551095363746320496e0 * t77 * t80 * (26180.0 / 19683.0 * t573 + 292.0 / 675.0 * t575 + 1168.0 / 2025.0 * t91 * t318 + 292.0 / 2025.0 * t44 * t583 - 73.0 / 9720.0 * t583 * t21 * t25 * t34 + 292.0 / 3645.0 * t322 * t84 - 1606.0 / 3645.0 * t177 * t164 + 44968.0 / 32805.0 * t95 * t309 - 191114.0 / 98415.0 * t48 * t572 + 0.945783685173486858e0 * t54 * t56 * t600);
        let t609 = piecewise3(t3, 0.0, t608);
        let tv4rho40 = 2.0 * rho[ip] * t609 + 8.0 * t342;
        v4rho4[ip] += tv4rho40;
        let t617 = t106 * t190;
        let t642 = t77 * t39;
        let t673 = 0.3972262887299116919e1 * t201 * t553 * t125 * t299 - 0.29791971654743376893e1 * t201 * t355 * t617 - 0.99306572182477922978e0 * t349 * t356 + 0.99306572182477922976e0 * t201 * t156 * t380 * t106 + 0.99306572182477922976e0 * t201 * t359 * t190 + 0.33102190727492640992e0 * t201 * t202 * t337 + 0.33102190727492640993e0 * t349 * t363 - 0.29791971654743376893e1 * t201 * t297 * t218 * t158 - 0.61300353199060446282e-1 * t77 * t532 * t125 - 0.22068127151661760662e0 * t642 * t203 + 0.66204381454985281985e0 * t349 * t360 - 0.16551095363746320496e0 * t77 * t80 * (-1540.0 / 6561.0 * t26 * t28 * t308 - 146.0 / 18225.0 * t321 * t118 + 1168.0 / 18225.0 * t176 * t211 - 12848.0 / 54675.0 * t94 * t373 + 179872.0 / 492075.0 * t47 * t117 * t308 - 0.23990701617629789013e0 * t54 * t27 * t333 * sigma[ip]) + 0.11034063575830880331e0 * t77 * t285 * t218 - 0.16551095363746320496e0 * t77 * t151 * t380;
        let t674 = piecewise3(t3, 0.0, t673);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t674 + 6.0 * t385;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = 0.0;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t684 = t297 * t237;
        let t688 = t156 * t413;
        let t717 = t553 * t140;
        let t726 = 0.99306572182477922976e0 * t201 * t401 * t190 + 0.33102190727492640992e0 * t201 * t229 * t337 - 0.29791971654743376893e1 * t201 * t684 * t158 + 0.99306572182477922976e0 * t201 * t688 * t106 - 0.61300353199060446282e-1 * t77 * t532 * t140 - 0.22068127151661760662e0 * t642 * t230 + 0.66204381454985281985e0 * t349 * t402 + 0.33102190727492640993e0 * t349 * t405 - 0.16551095363746320496e0 * t77 * t80 * (73.0 / 2025.0 * t321 * t131 - 73.0 / 405.0 * t369 + 584.0 / 1215.0 * t371 - 6424.0 / 10935.0 * t374 + 18031.0 / 32805.0 * t378) + 0.11034063575830880331e0 * t77 * t285 * t237 - 0.16551095363746320496e0 * t77 * t151 * t413 + 0.3972262887299116919e1 * t201 * t717 * t299 - 0.29791971654743376893e1 * t201 * t397 * t617 - 0.99306572182477922978e0 * t349 * t398;
        let t727 = piecewise3(t3, 0.0, t726);
        let tv4rho3tau0 = 2.0 * rho[ip] * t727 + 6.0 * t418;
        v4rho3tau[ip] += tv4rho3tau0;
        let t742 = t106 * t218;
        let t749 = t218 * t218;
        let t761 = t77 * t431 * t156;
        let t765 = t77 * t249 * t297;
        let t767 = t54 * t27 * t158;
        let t771 = t54 * t27 * t190;
        let t774 = -0.7356042383887253554e-1 * t77 * t536 * t245 - 0.66204381454985281985e0 * t349 * t425 + 0.44136254303323521323e0 * t349 * t428 + 0.3972262887299116919e1 * t201 * t553 * t245 * t158 - 0.3972262887299116919e1 * t201 * t355 * t742 - 0.99306572182477922976e0 * t201 * t424 * t190 + 0.66204381454985281984e0 * t77 * t157 * t749 + 0.66204381454985281984e0 * t201 * t202 * t380 - 0.50852705990244750606e-2 * t77 / t184 * t253 - 0.33901803993496500404e-2 * t761 * t438 - 0.10170541198048950121e-2 * t765 * t767 + 0.33901803993496500404e-3 * t436 * t771;
        let t775 = piecewise3(t3, 0.0, t774);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t775 + 4.0 * t442;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let t813 = t77 * t262 * t297;
        let t818 = -0.7356042383887253554e-1 * t642 * t259 - 0.66204381454985281985e0 * t349 * t448 + 0.22068127151661760662e0 * t349 * t451 + 0.22068127151661760662e0 * t349 * t454 + 0.3972262887299116919e1 * t201 * t717 * t125 * t158 - 0.19861314436495584595e1 * t201 * t684 * t447 - 0.19861314436495584595e1 * t201 * t397 * t742 - 0.99306572182477922976e0 * t201 * t397 * t125 * t190 + 0.33102190727492640992e0 * t201 * t688 * t125 + 0.66204381454985281984e0 * t201 * t401 * t218 + 0.33102190727492640992e0 * t201 * t229 * t380 + 0.13259039358611592826e-1 * t433 + 0.1060723148688927426e-1 * t439 + 0.39777118075834778475e-2 * t813 * t767 - 0.13259039358611592825e-2 * t459 * t771;
        let t819 = piecewise3(t3, 0.0, t818);
        let tv4rho2sigmatau0 = 2.0 * rho[ip] * t819 + 4.0 * t463;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let tv4rho2lapl20 = 0.0;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let t830 = t553 * t269;
        let t841 = t237 * t237;
        let t851 = t77 * t273 * t297;
        let t856 = -0.7356042383887253554e-1 * t77 * t536 * t269 - 0.66204381454985281985e0 * t349 * t470 + 0.44136254303323521323e0 * t349 * t473 + 0.3972262887299116919e1 * t201 * t830 * t158 - 0.3972262887299116919e1 * t201 * t397 * t106 * t237 - 0.99306572182477922976e0 * t201 * t469 * t190 + 0.66204381454985281984e0 * t77 * t157 * t841 + 0.66204381454985281984e0 * t201 * t229 * t413 - 0.35799406268251300628e-1 * t254 - 0.35799406268251300628e-1 * t460 - 0.17899703134125650314e-1 * t851 * t767 + 0.59665677113752167714e-2 * t478 * t771;
        let t857 = piecewise3(t3, 0.0, t856);
        let tv4rho2tau20 = 2.0 * rho[ip] * t857 + 4.0 * t482;
        v4rho2tau2[ip] += tv4rho2tau20;
        let t872 = t252 * t106;
        let t873 = t489 * t872;
        let t877 = t218 * t51 * t252;
        let t881 = piecewise3(t3, 0.0, -0.33102190727492640992e0 * t77 * t543 * t485 + 0.3972262887299116919e1 * t201 * t553 * t485 * t106 - 0.29791971654743376893e1 * t201 * t424 * t218 - 0.50852705990244750605e-2 * t761 * t490 - 0.30511623594146850363e-2 * t765 * t873 + 0.10170541198048950121e-2 * t436 * t877);
        let tv4rhosigma30 = 2.0 * rho[ip] * t881 + 2.0 * t494;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4rhosigma2lapl0 = 0.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let t904 = t501 * t872;
        let t908 = t237 * t51 * t252;
        let t912 = piecewise3(t3, 0.0, -0.33102190727492640992e0 * t349 * t496 + 0.3972262887299116919e1 * t201 * t717 * t245 * t106 - 0.99306572182477922976e0 * t201 * t684 * t245 - 0.19861314436495584595e1 * t201 * t397 * t125 * t218 + 0.1060723148688927426e-1 * t491 + 0.7955423615166955695e-2 * t813 * t873 - 0.2651807871722318565e-2 * t459 * t877 - 0.16950901996748250202e-2 * t761 * t502 - 0.10170541198048950121e-2 * t765 * t904 + 0.33901803993496500404e-3 * t436 * t908);
        let tv4rhosigma2tau0 = 2.0 * rho[ip] * t912 + 2.0 * t506;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let tv4rhosigmalapl20 = 0.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let t938 = piecewise3(t3, 0.0, -0.33102190727492640992e0 * t349 * t508 + 0.3972262887299116919e1 * t201 * t830 * t447 - 0.19861314436495584595e1 * t201 * t397 * t125 * t237 - 0.99306572182477922976e0 * t201 * t469 * t218 + 0.1060723148688927426e-1 * t503 + 0.79554236151669556953e-2 * t813 * t904 - 0.26518078717223185651e-2 * t459 * t908 - 0.17899703134125650314e-1 * t499 - 0.17899703134125650314e-1 * t851 * t873 + 0.59665677113752167714e-2 * t478 * t877);
        let tv4rhosigmatau20 = 2.0 * rho[ip] * t938 + 2.0 * t516;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let t944 = t553 * t518;
        let t957 = piecewise3(t3, 0.0, -0.33102190727492640992e0 * t77 * t543 * t518 + 0.3972262887299116919e1 * t201 * t944 * t106 - 0.29791971654743376893e1 * t201 * t469 * t237 - 0.53699109402376950942e-1 * t511 - 0.53699109402376950942e-1 * t851 * t904 + 0.17899703134125650314e-1 * t478 * t908);
        let tv4rhotau30 = 2.0 * rho[ip] * t957 + 2.0 * t525;
        v4rhotau3[ip] += tv4rhotau30;
        let t960 = t245 * t245;
        let t965 = t245 * t51 * t252;
        let t973 = t22 * t22;
        let t977 = t156 * t21 / t24 / t973 * t28;
        let t981 = piecewise3(t3, 0.0, 0.3972262887299116919e1 * t77 * t554 * t960 - 0.61023247188293700727e-2 * t765 * t965 + 0.62497318750145534195e-5 * t77 / t19 / t331 / t30 * t977);
        let tv4sigma40 = 2.0 * rho[ip] * t981;
        v4sigma4[ip] += tv4sigma40;
        let tv4sigma3lapl0 = 0.0;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let t990 = t140 * t125 * t54 * t27;
        let t997 = piecewise3(t3, 0.0, 0.3972262887299116919e1 * t201 * t717 * t485 + 0.11933135422750433543e-1 * t813 * t965 - 0.30511623594146850364e-2 * t765 * t990 - 0.24442782138520933624e-4 * t77 * t600 * t977);
        let tv4sigma3tau0 = 2.0 * rho[ip] * t997;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let tv4sigma2lapl20 = 0.0;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let t1005 = t269 * t51 * t252;
        let t1014 = piecewise3(t3, 0.0, 0.3972262887299116919e1 * t201 * t830 * t245 + 0.1591084723033391139e-1 * t813 * t990 - 0.10170541198048950121e-2 * t765 * t1005 + 0.10039486444588156678e-3 * t77 * t333 * t977 - 0.17899703134125650314e-1 * t851 * t965);
        let tv4sigma2tau20 = 2.0 * rho[ip] * t1014;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let tv4sigmalapl30 = 0.0;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let t1027 = piecewise3(t3, 0.0, 0.3972262887299116919e1 * t201 * t944 * t125 + 0.11933135422750433543e-1 * t813 * t1005 - 0.53699109402376950942e-1 * t851 * t990 - 0.43018216585717612277e-3 * t77 * t186 * t977);
        let tv4sigmatau30 = 2.0 * rho[ip] * t1027;
        v4sigmatau3[ip] += tv4sigmatau30;
        let tv4lapl40 = 0.0;
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let t1029 = t269 * t269;
        let t1039 = piecewise3(t3, 0.0, 0.3972262887299116919e1 * t77 * t554 * t1029 - 0.10739821880475390188e0 * t851 * t1005 + 0.19358197463572925525e-2 * t77 * t102 * t977);
        let tv4tau40 = 2.0 * rho[ip] * t1039;
        v4tau4[ip] += tv4tau40;
    }
}
