//! MGGA_X_GDME fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gdme.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_gdme_fxc_pol(
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
    param_AA: f64,
    param_BB: f64,
    param_a: f64,
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
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t31 = M_CBRT2;
        let t34 = pow_1_3(1.0 / M_PI);
        let t35 = 1.0 / t34;
        let t36 = M_CBRT4;
        let t37 = t35 * t36;
        let t38 = M_PI * M_PI;
        let t39 = pow_1_3(t38);
        let t40 = t39 * t39;
        let t44 = 2.0 / 9.0 * (param_AA + 3.0 / 5.0 * param_BB) * t31 * t37 / t40;
        let t46 = param_BB * t3 * t35;
        let t47 = t31 * t31;
        let t48 = t36 * t47;
        let t50 = 1.0 / t39 / t38;
        let t51 = param_a * param_a;
        let t52 = t51 - param_a + 1.0 / 2.0;
        let t53 = t52 * lapl0;
        let t54 = pow_1_3(rho0);
        let t55 = t54 * t54;
        let t57 = 1.0 / t55 / rho0;
        let t66 = t44 + t46 * t48 * t50 * (t53 * t57 - 2.0 * t57 * tau0) / 27.0;
        let t70 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t66);
        let t71 = rho1 <= dens_threshold;
        let t72 = -t17;
        let t74 = piecewise5(t15, t12, t11, t16, t72 * t8);
        let t75 = 1.0 + t74;
        let t76 = t75 <= zeta_threshold;
        let t77 = pow_1_3(t75);
        let t79 = piecewise3(t76, t23, t77 * t75);
        let t80 = t79 * t27;
        let t81 = t52 * lapl1;
        let t82 = pow_1_3(rho1);
        let t83 = t82 * t82;
        let t85 = 1.0 / t83 / rho1;
        let t94 = t44 + t46 * t48 * t50 * (t81 * t85 - 2.0 * t85 * tau1) / 27.0;
        let t98 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t80 * t94);
        let tzk0 = t70 + t98;
        zk[ip] += tzk0;
        let t99 = t7 * t7;
        let t100 = 1.0 / t99;
        let t101 = t17 * t100;
        let t103 = piecewise5(t11, 0.0, t15, 0.0, t8 - t101);
        let t106 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t103);
        let t107 = t106 * t27;
        let t111 = t27 * t27;
        let t112 = 1.0 / t111;
        let t113 = t26 * t112;
        let t116 = t6 * t113 * t66 / 8.0;
        let t117 = t3 * t3;
        let t118 = t117 * t5;
        let t120 = t118 * t28 * param_BB;
        let t121 = t47 * t50;
        let t122 = rho0 * rho0;
        let t124 = 1.0 / t55 / t122;
        let t131 = t37 * t121 * (-5.0 / 3.0 * t53 * t124 + 10.0 / 3.0 * tau0 * t124);
        let t135 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t107 * t66 - t116 - t120 * t131 / 72.0);
        let t136 = t72 * t100;
        let t138 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t136);
        let t141 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t138);
        let t142 = t141 * t27;
        let t146 = t79 * t112;
        let t149 = t6 * t146 * t94 / 8.0;
        let t151 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t142 * t94 - t149);
        let tvrho0 = t70 + t98 + t7 * (t135 + t151);
        vrho[ip * 2] += tvrho0;
        let t155 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t101);
        let t158 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t155);
        let t159 = t158 * t27;
        let t164 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t159 * t66 - t116);
        let t166 = piecewise5(t15, 0.0, t11, 0.0, t8 - t136);
        let t169 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t166);
        let t170 = t169 * t27;
        let t175 = t118 * t80 * param_BB;
        let t176 = rho1 * rho1;
        let t178 = 1.0 / t83 / t176;
        let t185 = t37 * t121 * (-5.0 / 3.0 * t81 * t178 + 10.0 / 3.0 * tau1 * t178);
        let t189 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t170 * t94 - t149 - t175 * t185 / 72.0);
        let tvrho1 = t70 + t98 + t7 * (t164 + t189);
        vrho[ip * 2 + 1] += tvrho1;
        let tvsigma0 = 0.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = 0.0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t192 = t37 * t47;
        let t193 = t50 * t52;
        let t195 = t192 * t193 * t57;
        let t198 = piecewise3(t2, 0.0, -t120 * t195 / 72.0);
        let tvlapl0 = t7 * t198;
        vlapl[ip * 2] += tvlapl0;
        let t200 = t192 * t193 * t85;
        let t203 = piecewise3(t71, 0.0, -t175 * t200 / 72.0);
        let tvlapl1 = t7 * t203;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t205 = t37 * t121 * t57;
        let t208 = piecewise3(t2, 0.0, t120 * t205 / 36.0);
        let tvtau0 = t7 * t208;
        vtau[ip * 2] += tvtau0;
        let t210 = t37 * t121 * t85;
        let t213 = piecewise3(t71, 0.0, t175 * t210 / 36.0);
        let tvtau1 = t7 * t213;
        vtau[ip * 2 + 1] += tvtau1;
        let t216 = t24 * t24;
        let t217 = 1.0 / t216;
        let t218 = t103 * t103;
        let t221 = t99 * t7;
        let t222 = 1.0 / t221;
        let t223 = t17 * t222;
        let t226 = piecewise5(t11, 0.0, t15, 0.0, -2.0 * t100 + 2.0 * t223);
        let t230 = piecewise3(t21, 0.0, 4.0 / 9.0 * t217 * t218 + 4.0 / 3.0 * t24 * t226);
        let t231 = t230 * t27;
        let t235 = t106 * t112;
        let t237 = t6 * t235 * t66;
        let t240 = t118 * t107 * param_BB;
        let t244 = 1.0 / t111 / t7;
        let t245 = t26 * t244;
        let t248 = t6 * t245 * t66 / 12.0;
        let t250 = t118 * t113 * param_BB;
        let t251 = t250 * t131;
        let t255 = 1.0 / t55 / t122 / rho0;
        let t262 = t37 * t121 * (40.0 / 9.0 * t53 * t255 - 80.0 / 9.0 * tau0 * t255);
        let t266 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t231 * t66 - t237 / 4.0 - t240 * t131 / 36.0 + t248 - t251 / 108.0 - t120 * t262 / 72.0);
        let t267 = t77 * t77;
        let t268 = 1.0 / t267;
        let t269 = t138 * t138;
        let t272 = t72 * t222;
        let t275 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t100 + 2.0 * t272);
        let t279 = piecewise3(t76, 0.0, 4.0 / 9.0 * t268 * t269 + 4.0 / 3.0 * t77 * t275);
        let t280 = t279 * t27;
        let t284 = t141 * t112;
        let t286 = t6 * t284 * t94;
        let t288 = t79 * t244;
        let t291 = t6 * t288 * t94 / 12.0;
        let t293 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t280 * t94 - t286 / 4.0 + t291);
        let tv2rho20 = 2.0 * t135 + 2.0 * t151 + t7 * (t266 + t293);
        v2rho2[ip * 3] += tv2rho20;
        let t296 = t217 * t155;
        let t300 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t223);
        let t304 = piecewise3(t21, 0.0, 4.0 / 9.0 * t296 * t103 + 4.0 / 3.0 * t24 * t300);
        let t305 = t304 * t27;
        let t309 = t158 * t112;
        let t311 = t6 * t309 * t66;
        let t314 = t118 * t159 * param_BB;
        let t320 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t305 * t66 - t311 / 8.0 - t314 * t131 / 72.0 - t237 / 8.0 + t248 - t251 / 216.0);
        let t321 = t268 * t166;
        let t325 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t272);
        let t329 = piecewise3(t76, 0.0, 4.0 / 9.0 * t321 * t138 + 4.0 / 3.0 * t77 * t325);
        let t330 = t329 * t27;
        let t334 = t169 * t112;
        let t336 = t6 * t334 * t94;
        let t340 = t118 * t142 * param_BB;
        let t344 = t118 * t146 * param_BB;
        let t345 = t344 * t185;
        let t348 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t330 * t94 - t336 / 8.0 - t286 / 8.0 + t291 - t340 * t185 / 72.0 - t345 / 216.0);
        let tv2rho21 = t135 + t151 + t164 + t189 + t7 * (t320 + t348);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t353 = t155 * t155;
        let t358 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t100 + 2.0 * t223);
        let t362 = piecewise3(t21, 0.0, 4.0 / 9.0 * t217 * t353 + 4.0 / 3.0 * t24 * t358);
        let t363 = t362 * t27;
        let t369 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t363 * t66 - t311 / 4.0 + t248);
        let t370 = t166 * t166;
        let t375 = piecewise5(t15, 0.0, t11, 0.0, -2.0 * t100 + 2.0 * t272);
        let t379 = piecewise3(t76, 0.0, 4.0 / 9.0 * t268 * t370 + 4.0 / 3.0 * t77 * t375);
        let t380 = t379 * t27;
        let t386 = t118 * t170 * param_BB;
        let t392 = 1.0 / t83 / t176 / rho1;
        let t399 = t37 * t121 * (40.0 / 9.0 * t81 * t392 - 80.0 / 9.0 * tau1 * t392);
        let t403 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t380 * t94 - t336 / 4.0 - t386 * t185 / 36.0 + t291 - t345 / 108.0 - t175 * t399 / 72.0);
        let tv2rho22 = 2.0 * t164 + 2.0 * t189 + t7 * (t369 + t403);
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
        let t409 = t250 * t195 / 216.0;
        let t411 = t192 * t193 * t124;
        let t415 = piecewise3(t2, 0.0, -t240 * t195 / 72.0 - t409 + 5.0 / 216.0 * t120 * t411);
        let tv2rholapl0 = t415 * t7 + t198;
        v2rholapl[ip * 4] += tv2rholapl0;
        let t420 = t344 * t200 / 216.0;
        let t422 = piecewise3(t71, 0.0, -t340 * t200 / 72.0 - t420);
        let tv2rholapl1 = t422 * t7 + t203;
        v2rholapl[ip * 4 + 1] += tv2rholapl1;
        let t427 = piecewise3(t2, 0.0, -t314 * t195 / 72.0 - t409);
        let tv2rholapl2 = t427 * t7 + t198;
        v2rholapl[ip * 4 + 2] += tv2rholapl2;
        let t432 = t192 * t193 * t178;
        let t436 = piecewise3(t71, 0.0, -t386 * t200 / 72.0 - t420 + 5.0 / 216.0 * t175 * t432);
        let tv2rholapl3 = t436 * t7 + t203;
        v2rholapl[ip * 4 + 3] += tv2rholapl3;
        let t441 = t250 * t205 / 108.0;
        let t443 = t37 * t121 * t124;
        let t447 = piecewise3(t2, 0.0, t240 * t205 / 36.0 + t441 - 5.0 / 108.0 * t120 * t443);
        let tv2rhotau0 = t447 * t7 + t208;
        v2rhotau[ip * 4] += tv2rhotau0;
        let t452 = t344 * t210 / 108.0;
        let t454 = piecewise3(t71, 0.0, t340 * t210 / 36.0 + t452);
        let tv2rhotau1 = t454 * t7 + t213;
        v2rhotau[ip * 4 + 1] += tv2rhotau1;
        let t459 = piecewise3(t2, 0.0, t314 * t205 / 36.0 + t441);
        let tv2rhotau2 = t459 * t7 + t208;
        v2rhotau[ip * 4 + 2] += tv2rhotau2;
        let t464 = t37 * t121 * t178;
        let t468 = piecewise3(t71, 0.0, t386 * t210 / 36.0 + t452 - 5.0 / 108.0 * t175 * t464);
        let tv2rhotau3 = t468 * t7 + t213;
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
        let tv2tau20 = 0.0;
        v2tau2[ip * 3] += tv2tau20;
        let tv2tau21 = 0.0;
        v2tau2[ip * 3 + 1] += tv2tau21;
        let tv2tau22 = 0.0;
        v2tau2[ip * 3 + 2] += tv2tau22;
    }
}
