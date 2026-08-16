//! MGGA_X_LTA fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_lta.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_lta_fxc_pol(
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
    param_ltafrac: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5::<f64>(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3::<f64>(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3::<f64>(t20);
        let t26 = piecewise3::<f64>(t21, t23, t24 * t20);
        let t27 = pow_1_3::<f64>(t7);
        let t28 = t26 * t27;
        let t29 = pow_1_3::<f64>(rho0);
        let t30 = t29 * t29;
        let t34 = M_CBRT6;
        let t35 = M_PI * M_PI;
        let t36 = pow_1_3::<f64>(t35);
        let t37 = t36 * t36;
        let t39 = t34 / t37;
        let t42 = 4.0 / 5.0 * param_ltafrac;
        let t43 = f64::powf(5.0 / 9.0 * tau0 / t30 / rho0 * t39, t42);
        let t47 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t43);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t17;
        let t51 = piecewise5::<f64>(t15, t12, t11, t16, t49 * t8);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3::<f64>(t52);
        let t56 = piecewise3::<f64>(t53, t23, t54 * t52);
        let t57 = t56 * t27;
        let t58 = pow_1_3::<f64>(rho1);
        let t59 = t58 * t58;
        let t65 = f64::powf(5.0 / 9.0 * tau1 / t59 / rho1 * t39, t42);
        let t69 = piecewise3::<f64>(t48, 0.0, -3.0 / 8.0 * t6 * t57 * t65);
        let tzk0 = t47 + t69;
        zk[ip] += tzk0;
        let t70 = t7 * t7;
        let t71 = 1.0 / t70;
        let t72 = t17 * t71;
        let t74 = piecewise5::<f64>(t11, 0.0, t15, 0.0, t8 - t72);
        let t77 = piecewise3::<f64>(t21, 0.0, 4.0 / 3.0 * t24 * t74);
        let t78 = t77 * t27;
        let t82 = t27 * t27;
        let t83 = 1.0 / t82;
        let t84 = t26 * t83;
        let t87 = t6 * t84 * t43 / 8.0;
        let t88 = t6 * t26;
        let t89 = t27 * t43;
        let t90 = 1.0 / rho0;
        let t91 = param_ltafrac * t90;
        let t92 = t89 * t91;
        let t96 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t78 * t43 - t87 + t88 * t92 / 2.0);
        let t97 = t49 * t71;
        let t99 = piecewise5::<f64>(t15, 0.0, t11, 0.0, -t8 - t97);
        let t102 = piecewise3::<f64>(t53, 0.0, 4.0 / 3.0 * t54 * t99);
        let t103 = t102 * t27;
        let t107 = t56 * t83;
        let t110 = t6 * t107 * t65 / 8.0;
        let t112 = piecewise3::<f64>(t48, 0.0, -3.0 / 8.0 * t6 * t103 * t65 - t110);
        let tvrho0 = t47 + t69 + t7 * (t96 + t112);
        vrho[ip * 2] += tvrho0;
        let t116 = piecewise5::<f64>(t11, 0.0, t15, 0.0, -t8 - t72);
        let t119 = piecewise3::<f64>(t21, 0.0, 4.0 / 3.0 * t24 * t116);
        let t120 = t119 * t27;
        let t125 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t120 * t43 - t87);
        let t127 = piecewise5::<f64>(t15, 0.0, t11, 0.0, t8 - t97);
        let t130 = piecewise3::<f64>(t53, 0.0, 4.0 / 3.0 * t54 * t127);
        let t131 = t130 * t27;
        let t135 = t6 * t56;
        let t136 = t27 * t65;
        let t137 = 1.0 / rho1;
        let t138 = param_ltafrac * t137;
        let t139 = t136 * t138;
        let t143 = piecewise3::<f64>(t48, 0.0, -3.0 / 8.0 * t6 * t131 * t65 - t110 + t135 * t139 / 2.0);
        let tvrho1 = t47 + t69 + t7 * (t125 + t143);
        vrho[ip * 2 + 1] += tvrho1;
        let tvsigma0 = 0.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = 0.0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t146 = 1.0 / tau0;
        let t147 = param_ltafrac * t146;
        let t148 = t89 * t147;
        let t151 = piecewise3::<f64>(t2, 0.0, -3.0 / 10.0 * t88 * t148);
        let tvtau0 = t7 * t151;
        vtau[ip * 2] += tvtau0;
        let t152 = 1.0 / tau1;
        let t153 = param_ltafrac * t152;
        let t154 = t136 * t153;
        let t157 = piecewise3::<f64>(t48, 0.0, -3.0 / 10.0 * t135 * t154);
        let tvtau1 = t7 * t157;
        vtau[ip * 2 + 1] += tvtau1;
        let t160 = t24 * t24;
        let t161 = 1.0 / t160;
        let t162 = t74 * t74;
        let t165 = t70 * t7;
        let t166 = 1.0 / t165;
        let t167 = t17 * t166;
        let t170 = piecewise5::<f64>(t11, 0.0, t15, 0.0, -2.0 * t71 + 2.0 * t167);
        let t174 = piecewise3::<f64>(t21, 0.0, 4.0 / 9.0 * t161 * t162 + 4.0 / 3.0 * t24 * t170);
        let t175 = t174 * t27;
        let t179 = t77 * t83;
        let t181 = t6 * t179 * t43;
        let t183 = t6 * t77;
        let t186 = 1.0 / t82 / t7;
        let t187 = t26 * t186;
        let t190 = t6 * t187 * t43 / 12.0;
        let t191 = t83 * t43;
        let t192 = t191 * t91;
        let t193 = t88 * t192;
        let t195 = param_ltafrac * param_ltafrac;
        let t196 = rho0 * rho0;
        let t197 = 1.0 / t196;
        let t198 = t195 * t197;
        let t199 = t89 * t198;
        let t202 = param_ltafrac * t197;
        let t203 = t89 * t202;
        let t207 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t175 * t43 - t181 / 4.0 + t183 * t92 + t190 + t193 / 3.0 - 2.0 / 3.0 * t88 * t199 - t88 * t203 / 2.0);
        let t208 = t54 * t54;
        let t209 = 1.0 / t208;
        let t210 = t99 * t99;
        let t213 = t49 * t166;
        let t216 = piecewise5::<f64>(t15, 0.0, t11, 0.0, 2.0 * t71 + 2.0 * t213);
        let t220 = piecewise3::<f64>(t53, 0.0, 4.0 / 9.0 * t209 * t210 + 4.0 / 3.0 * t54 * t216);
        let t221 = t220 * t27;
        let t225 = t102 * t83;
        let t227 = t6 * t225 * t65;
        let t229 = t56 * t186;
        let t232 = t6 * t229 * t65 / 12.0;
        let t234 = piecewise3::<f64>(t48, 0.0, -3.0 / 8.0 * t6 * t221 * t65 - t227 / 4.0 + t232);
        let tv2rho20 = 2.0 * t96 + 2.0 * t112 + t7 * (t207 + t234);
        v2rho2[ip * 3] += tv2rho20;
        let t237 = t161 * t116;
        let t241 = piecewise5::<f64>(t11, 0.0, t15, 0.0, 2.0 * t167);
        let t245 = piecewise3::<f64>(t21, 0.0, 4.0 / 9.0 * t237 * t74 + 4.0 / 3.0 * t24 * t241);
        let t246 = t245 * t27;
        let t250 = t119 * t83;
        let t252 = t6 * t250 * t43;
        let t254 = t6 * t119;
        let t260 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t246 * t43 - t252 / 8.0 + t254 * t92 / 2.0 - t181 / 8.0 + t190 + t193 / 6.0);
        let t261 = t209 * t127;
        let t265 = piecewise5::<f64>(t15, 0.0, t11, 0.0, 2.0 * t213);
        let t269 = piecewise3::<f64>(t53, 0.0, 4.0 / 9.0 * t261 * t99 + 4.0 / 3.0 * t54 * t265);
        let t270 = t269 * t27;
        let t274 = t130 * t83;
        let t276 = t6 * t274 * t65;
        let t279 = t6 * t102;
        let t282 = t83 * t65;
        let t283 = t282 * t138;
        let t284 = t135 * t283;
        let t287 = piecewise3::<f64>(t48, 0.0, -3.0 / 8.0 * t6 * t270 * t65 - t276 / 8.0 - t227 / 8.0 + t232 + t279 * t139 / 2.0 + t284 / 6.0);
        let tv2rho21 = t96 + t112 + t125 + t143 + t7 * (t260 + t287);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t292 = t116 * t116;
        let t297 = piecewise5::<f64>(t11, 0.0, t15, 0.0, 2.0 * t71 + 2.0 * t167);
        let t301 = piecewise3::<f64>(t21, 0.0, 4.0 / 9.0 * t161 * t292 + 4.0 / 3.0 * t24 * t297);
        let t302 = t301 * t27;
        let t308 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t302 * t43 - t252 / 4.0 + t190);
        let t309 = t127 * t127;
        let t314 = piecewise5::<f64>(t15, 0.0, t11, 0.0, -2.0 * t71 + 2.0 * t213);
        let t318 = piecewise3::<f64>(t53, 0.0, 4.0 / 9.0 * t209 * t309 + 4.0 / 3.0 * t54 * t314);
        let t319 = t318 * t27;
        let t324 = t6 * t130;
        let t327 = rho1 * rho1;
        let t328 = 1.0 / t327;
        let t329 = t195 * t328;
        let t330 = t136 * t329;
        let t333 = param_ltafrac * t328;
        let t334 = t136 * t333;
        let t338 = piecewise3::<f64>(t48, 0.0, -3.0 / 8.0 * t6 * t319 * t65 - t276 / 4.0 + t324 * t139 + t232 + t284 / 3.0 - 2.0 / 3.0 * t135 * t330 - t135 * t334 / 2.0);
        let tv2rho22 = 2.0 * t125 + 2.0 * t143 + t7 * (t308 + t338);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = 0.0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = 0.0;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let tv2rhosigma5 = 0.0;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip * 4] += tv2rholapl0;
        let tv2rholapl1 = 0.0;
        v2rholapl[ip * 4 + 1] += tv2rholapl1;
        let tv2rholapl2 = 0.0;
        v2rholapl[ip * 4 + 2] += tv2rholapl2;
        let tv2rholapl3 = 0.0;
        v2rholapl[ip * 4 + 3] += tv2rholapl3;
        let t343 = t191 * t147;
        let t345 = t88 * t343 / 10.0;
        let t346 = t6 * t28;
        let t347 = t43 * t195;
        let t349 = t347 * t90 * t146;
        let t353 = piecewise3::<f64>(t2, 0.0, -3.0 / 10.0 * t183 * t148 - t345 + 2.0 / 5.0 * t346 * t349);
        let tv2rhotau0 = t7 * t353 + t151;
        v2rhotau[ip * 4] += tv2rhotau0;
        let t357 = t282 * t153;
        let t359 = t135 * t357 / 10.0;
        let t361 = piecewise3::<f64>(t48, 0.0, -3.0 / 10.0 * t279 * t154 - t359);
        let tv2rhotau1 = t7 * t361 + t157;
        v2rhotau[ip * 4 + 1] += tv2rhotau1;
        let t366 = piecewise3::<f64>(t2, 0.0, -3.0 / 10.0 * t254 * t148 - t345);
        let tv2rhotau2 = t7 * t366 + t151;
        v2rhotau[ip * 4 + 2] += tv2rhotau2;
        let t370 = t6 * t57;
        let t371 = t65 * t195;
        let t373 = t371 * t137 * t152;
        let t377 = piecewise3::<f64>(t48, 0.0, -3.0 / 10.0 * t324 * t154 - t359 + 2.0 / 5.0 * t370 * t373);
        let tv2rhotau3 = t7 * t377 + t157;
        v2rhotau[ip * 4 + 3] += tv2rhotau3;
        let tv2sigma20 = 0.0;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = 0.0;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip * 6] += tv2sigmalapl0;
        let tv2sigmalapl1 = 0.0;
        v2sigmalapl[ip * 6 + 1] += tv2sigmalapl1;
        let tv2sigmalapl2 = 0.0;
        v2sigmalapl[ip * 6 + 2] += tv2sigmalapl2;
        let tv2sigmalapl3 = 0.0;
        v2sigmalapl[ip * 6 + 3] += tv2sigmalapl3;
        let tv2sigmalapl4 = 0.0;
        v2sigmalapl[ip * 6 + 4] += tv2sigmalapl4;
        let tv2sigmalapl5 = 0.0;
        v2sigmalapl[ip * 6 + 5] += tv2sigmalapl5;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip * 6] += tv2sigmatau0;
        let tv2sigmatau1 = 0.0;
        v2sigmatau[ip * 6 + 1] += tv2sigmatau1;
        let tv2sigmatau2 = 0.0;
        v2sigmatau[ip * 6 + 2] += tv2sigmatau2;
        let tv2sigmatau3 = 0.0;
        v2sigmatau[ip * 6 + 3] += tv2sigmatau3;
        let tv2sigmatau4 = 0.0;
        v2sigmatau[ip * 6 + 4] += tv2sigmatau4;
        let tv2sigmatau5 = 0.0;
        v2sigmatau[ip * 6 + 5] += tv2sigmatau5;
        let tv2lapl20 = 0.0;
        v2lapl2[ip * 3] += tv2lapl20;
        let tv2lapl21 = 0.0;
        v2lapl2[ip * 3 + 1] += tv2lapl21;
        let tv2lapl22 = 0.0;
        v2lapl2[ip * 3 + 2] += tv2lapl22;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip * 4] += tv2lapltau0;
        let tv2lapltau1 = 0.0;
        v2lapltau[ip * 4 + 1] += tv2lapltau1;
        let tv2lapltau2 = 0.0;
        v2lapltau[ip * 4 + 2] += tv2lapltau2;
        let tv2lapltau3 = 0.0;
        v2lapltau[ip * 4 + 3] += tv2lapltau3;
        let t379 = tau0 * tau0;
        let t380 = 1.0 / t379;
        let t381 = t195 * t380;
        let t382 = t89 * t381;
        let t385 = param_ltafrac * t380;
        let t386 = t89 * t385;
        let t390 = piecewise3::<f64>(t2, 0.0, -6.0 / 25.0 * t88 * t382 + 3.0 / 10.0 * t88 * t386);
        let tv2tau20 = t7 * t390;
        v2tau2[ip * 3] += tv2tau20;
        let tv2tau21 = 0.0;
        v2tau2[ip * 3 + 1] += tv2tau21;
        let t391 = tau1 * tau1;
        let t392 = 1.0 / t391;
        let t393 = t195 * t392;
        let t394 = t136 * t393;
        let t397 = param_ltafrac * t392;
        let t398 = t136 * t397;
        let t402 = piecewise3::<f64>(t48, 0.0, -6.0 / 25.0 * t135 * t394 + 3.0 / 10.0 * t135 * t398);
        let tv2tau22 = t7 * t402;
        v2tau2[ip * 3 + 2] += tv2tau22;
    }
}
