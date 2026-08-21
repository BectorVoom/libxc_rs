//! MGGA_X_PKZB kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pkzb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_pkzb_kxc_unpol(
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
        let t64 = 0.804 + 5.0 / 972.0 * t35 + 146.0 / 2025.0 * t45 - 73.0 / 9720.0 * t48 * t34 + 0.0004581846800182562 * t54 * t56 * t60;
        let t67 = 1.804 - 0.646416 / t64;
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
        let t106 = -10.0 / 729.0 * t85 + 292.0 / 2025.0 * t44 * t91 - 73.0 / 9720.0 * t95 * t34 + 73.0 / 3645.0 * t48 * t84 - 0.002443651626764033 * t54 * t56 * t102;
        let t111 = piecewise3(t3, 0.0, -t7 * t18 * t72 * t67 / 8.0 - 0.1655109536374632 * t77 * t80 * t106);
        let tvrho0 = 2.0 * rho[ip] * t111 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t117 = t25 * t28;
        let t118 = t117 * t33;
        let t119 = t47 * t118;
        let t123 = t54 * t27 * t60 * sigma[ip];
        let t125 = 5.0 / 972.0 * t26 * t28 * t33 - 146.0 / 18225.0 * t119 + 0.0009685241382715376 * t123;
        let t129 = piecewise3(t3, 0.0, -0.1655109536374632 * t77 * t80 * t125);
        let tvsigma0 = 2.0 * rho[ip] * t129;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t131 = t117 * t39;
        let t140 = 73.0 / 2025.0 * t47 * t131 - 73.0 / 19440.0 * t54 * t27 / t19 / t57 * sigma[ip];
        let t144 = piecewise3(t3, 0.0, -0.1655109536374632 * t77 * t80 * t140);
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
        let t190 = 110.0 / 2187.0 * t165 + 292.0 / 2025.0 * t167 + 292.0 / 2025.0 * t44 * t173 - 73.0 / 9720.0 * t177 * t34 + 146.0 / 3645.0 * t95 * t84 - 803.0 / 10935.0 * t48 * t164 + 0.015476460302838876 * t54 * t56 * t186;
        let t195 = piecewise3(t3, 0.0, t7 * t18 * t39 * t67 / 12.0 - 0.1103406357583088 * t77 * t151 * t106 + 0.3310219072749264 * t77 * t157 * t158 - 0.1655109536374632 * t77 * t80 * t190);
        let tv2rho20 = 2.0 * rho[ip] * t195 + 4.0 * t111;
        v2rho2[ip] += tv2rho20;
        let t201 = t77 * t19;
        let t202 = t156 * t125;
        let t203 = t202 * t106;
        let t209 = t94 * t118;
        let t211 = t117 * t83;
        let t212 = t47 * t211;
        let t216 = t54 * t27 * t102 * sigma[ip];
        let t218 = -10.0 / 729.0 * t26 * t28 * t83 - 146.0 / 18225.0 * t209 + 1168.0 / 54675.0 * t212 - 0.005165462070781533 * t216;
        let t223 = piecewise3(t3, 0.0, -0.0551703178791544 * t77 * t151 * t125 + 0.3310219072749264 * t201 * t203 - 0.1655109536374632 * t77 * t80 * t218);
        let tv2rhosigma0 = 2.0 * rho[ip] * t223 + 2.0 * t129;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t229 = t156 * t140;
        let t230 = t229 * t106;
        let t237 = 73.0 / 2025.0 * t94 * t131 - 73.0 / 1215.0 * t119 + 949.0 / 58320.0 * t123;
        let t242 = piecewise3(t3, 0.0, -0.0551703178791544 * t77 * t151 * t140 + 0.3310219072749264 * t201 * t230 - 0.1655109536374632 * t77 * t80 * t237);
        let tv2rhotau0 = 2.0 * rho[ip] * t242 + 2.0 * t144;
        v2rhotau[ip] += tv2rhotau0;
        let t245 = t125 * t125;
        let t249 = 1.0 / t58;
        let t252 = t53 * t27;
        let t253 = t79 * t51 * t252;
        let t254 = t77 * t249 * t253;
        let t257 = piecewise3(t3, 0.0, 0.3310219072749264 * t77 * t157 * t245 - 0.0001695090199674825 * t254);
        let tv2sigma20 = 2.0 * rho[ip] * t257;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t259 = t229 * t125;
        let t262 = 1.0 / t57;
        let t264 = t77 * t262 * t253;
        let t267 = piecewise3(t3, 0.0, 0.3310219072749264 * t201 * t259 + 0.0006629519679305796 * t264);
        let tv2sigmatau0 = 2.0 * rho[ip] * t267;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t269 = t140 * t140;
        let t273 = 1.0 / t81;
        let t278 = piecewise3(t3, 0.0, 0.3310219072749264 * t77 * t157 * t269 - 0.002983283855687608 * t77 * t273 * t253);
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
        let t337 = -1540.0 / 6561.0 * t310 + 292.0 / 675.0 * t91 * t173 + 292.0 / 2025.0 * t44 * t318 - 73.0 / 9720.0 * t322 * t34 + 73.0 / 1215.0 * t177 * t84 - 803.0 / 3645.0 * t95 * t164 + 11242.0 / 32805.0 * t48 * t309 - 0.11349404222081842 * t54 * t56 * t333;
        let t342 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t18 * t33 * t67 + 0.1103406357583088 * t77 * t285 * t106 + 0.3310219072749264 * t77 * t289 * t158 - 0.1655109536374632 * t77 * t151 * t190 - 0.9930657218247793 * t77 * t298 * t299 + 0.9930657218247793 * t201 * t304 - 0.1655109536374632 * t77 * t80 * t337);
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
        let t380 = 110.0 / 2187.0 * t26 * t28 * t163 - 146.0 / 18225.0 * t369 + 2336.0 / 54675.0 * t371 - 12848.0 / 164025.0 * t374 + 0.03271459311494971 * t378;
        let t385 = piecewise3(t3, 0.0, 0.03678021191943627 * t77 * t285 * t125 + 0.2206812715166176 * t349 * t203 - 0.1103406357583088 * t77 * t151 * t218 - 0.9930657218247793 * t201 * t356 + 0.6620438145498528 * t201 * t360 + 0.3310219072749264 * t201 * t363 - 0.1655109536374632 * t77 * t80 * t380);
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
        let t418 = piecewise3(t3, 0.0, 0.03678021191943627 * t77 * t285 * t140 + 0.2206812715166176 * t349 * t230 - 0.1103406357583088 * t77 * t151 * t237 - 0.9930657218247793 * t201 * t398 + 0.6620438145498528 * t201 * t402 + 0.3310219072749264 * t201 * t405 - 0.1655109536374632 * t77 * t80 * t413);
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
        let t442 = piecewise3(t3, 0.0, 0.1103406357583088 * t77 * t289 * t245 - 0.9930657218247793 * t201 * t425 + 0.6620438145498528 * t201 * t428 + 0.0008475450998374125 * t433 + 0.000339018039934965 * t439);
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
        let t463 = piecewise3(t3, 0.0, 0.1103406357583088 * t349 * t259 - 0.9930657218247793 * t201 * t448 + 0.3310219072749264 * t201 * t451 + 0.3310219072749264 * t201 * t454 - 0.0026518078717223184 * t254 - 0.0013259039358611592 * t460);
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
        let t482 = piecewise3(t3, 0.0, 0.1103406357583088 * t77 * t289 * t269 - 0.9930657218247793 * t201 * t470 + 0.6620438145498528 * t201 * t473 + 0.008949851567062826 * t264 + 0.005966567711375216 * t478 * t438);
        let tv3rhotau20 = 2.0 * rho[ip] * t482 + 2.0 * t278;
        v3rhotau2[ip] += tv3rhotau20;
        let t485 = t245 * t125;
        let t489 = t125 * t51;
        let t490 = t489 * t252;
        let t491 = t436 * t490;
        let t494 = piecewise3(t3, 0.0, -0.9930657218247793 * t77 * t298 * t485 + 0.001017054119804895 * t491);
        let tv3sigma30 = 2.0 * rho[ip] * t494;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let t496 = t397 * t245;
        let t499 = t459 * t490;
        let t501 = t140 * t51;
        let t502 = t501 * t252;
        let t503 = t436 * t502;
        let t506 = piecewise3(t3, 0.0, -0.9930657218247793 * t201 * t496 - 0.0026518078717223184 * t499 + 0.000339018039934965 * t503);
        let tv3sigma2tau0 = 2.0 * rho[ip] * t506;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let t508 = t469 * t125;
        let t511 = t459 * t502;
        let t516 = piecewise3(t3, 0.0, -0.9930657218247793 * t201 * t508 - 0.0026518078717223184 * t511 + 0.005966567711375216 * t478 * t490);
        let tv3sigmatau20 = 2.0 * rho[ip] * t516;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let t518 = t269 * t140;
        let t525 = piecewise3(t3, 0.0, -0.9930657218247793 * t77 * t298 * t518 + 0.01789970313412565 * t478 * t502);
        let tv3tau30 = 2.0 * rho[ip] * t525;
        v3tau3[ip] += tv3tau30;
    }
}
