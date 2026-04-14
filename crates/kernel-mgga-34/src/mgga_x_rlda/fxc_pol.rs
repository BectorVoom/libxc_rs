//! MGGA_X_RLDA fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 44 shared lines across all orders.
//! Delta: 153 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_rlda_fxc_pol(
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
    param_prefactor: f64,
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
        // --- shared preamble (44 lines) ---
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRTPI;
        let t4 = t3 * t3;
        let t5 = rho0 + rho1;
        let t6 = 1.0 / t5;
        let t9 = 2.0 * rho0 * t6 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t13 = 2.0 * rho1 * t6 <= zeta_threshold;
        let t14 = -t10;
        let t15 = rho0 - rho1;
        let t17 = piecewise5(t9, t10, t13, t14, t15 * t6);
        let t18 = 1.0 + t17;
        let t19 = t18 <= zeta_threshold;
        let t20 = pow_1_3(zeta_threshold);
        let t21 = t20 * zeta_threshold;
        let t22 = pow_1_3(t18);
        let t24 = piecewise3(t19, t21, t22 * t18);
        let t25 = t4 * t24;
        let t26 = pow_1_3(t5);
        let t29 = pow_1_3(1.0 / M_PI);
        let t30 = 1.0 / t29;
        let t31 = param_prefactor * t30;
        let t32 = M_CBRT4;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / rho0;
        let t41 = 2.0 * tau0 * t36 - lapl0 * t36 / 4.0;
        let t44 = t31 * t32 / t41;
        let t47 = piecewise3(t2, 0.0, -15.0 / 16.0 * t25 * t26 * t44);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t15;
        let t51 = piecewise5(t13, t10, t9, t14, t49 * t6);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t21, t54 * t52);
        let t57 = t4 * t56;
        let t59 = pow_1_3(rho1);
        let t60 = t59 * t59;
        let t62 = 1.0 / t60 / rho1;
        let t67 = 2.0 * tau1 * t62 - lapl1 * t62 / 4.0;
        let t70 = t31 * t32 / t67;
        let t73 = piecewise3(t48, 0.0, -15.0 / 16.0 * t57 * t26 * t70);
        let tzk0 = t47 + t73;
        zk[ip] += tzk0;
        // --- vxc delta (57 lines) ---
        let t74 = t5 * t5;
        let t75 = 1.0 / t74;
        let t76 = t15 * t75;
        let t78 = piecewise5(t9, 0.0, t13, 0.0, t6 - t76);
        let t81 = piecewise3(t19, 0.0, 4.0 / 3.0 * t22 * t78);
        let t82 = t4 * t81;
        let t86 = t26 * t26;
        let t87 = 1.0 / t86;
        let t90 = 5.0 / 16.0 * t25 * t87 * t44;
        let t91 = t26 * param_prefactor;
        let t92 = t25 * t91;
        let t93 = t30 * t32;
        let t94 = t41 * t41;
        let t95 = 1.0 / t94;
        let t96 = rho0 * rho0;
        let t98 = 1.0 / t34 / t96;
        let t103 = -10.0 / 3.0 * tau0 * t98 + 5.0 / 12.0 * lapl0 * t98;
        let t105 = t93 * t95 * t103;
        let t109 = piecewise3(t2, 0.0, -15.0 / 16.0 * t82 * t26 * t44 - t90 + 15.0 / 16.0 * t92 * t105);
        let t110 = t49 * t75;
        let t112 = piecewise5(t13, 0.0, t9, 0.0, -t6 - t110);
        let t115 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t112);
        let t116 = t4 * t115;
        let t122 = 5.0 / 16.0 * t57 * t87 * t70;
        let t124 = piecewise3(t48, 0.0, -15.0 / 16.0 * t116 * t26 * t70 - t122);
        let tvrho0 = t47 + t73 + t5 * (t109 + t124);
        vrho[ip * 2] += tvrho0;
        let t128 = piecewise5(t9, 0.0, t13, 0.0, -t6 - t76);
        let t131 = piecewise3(t19, 0.0, 4.0 / 3.0 * t22 * t128);
        let t132 = t4 * t131;
        let t137 = piecewise3(t2, 0.0, -15.0 / 16.0 * t132 * t26 * t44 - t90);
        let t139 = piecewise5(t13, 0.0, t9, 0.0, t6 - t110);
        let t142 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t139);
        let t143 = t4 * t142;
        let t147 = t57 * t91;
        let t148 = t67 * t67;
        let t149 = 1.0 / t148;
        let t150 = rho1 * rho1;
        let t152 = 1.0 / t60 / t150;
        let t157 = -10.0 / 3.0 * tau1 * t152 + 5.0 / 12.0 * lapl1 * t152;
        let t159 = t93 * t149 * t157;
        let t163 = piecewise3(t48, 0.0, -15.0 / 16.0 * t143 * t26 * t70 - t122 + 15.0 / 16.0 * t147 * t159);
        let tvrho1 = t47 + t73 + t5 * (t137 + t163);
        vrho[ip * 2 + 1] += tvrho1;
        let tvsigma0 = 0.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = 0.0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t167 = t93 * t95 * t36;
        let t168 = t92 * t167;
        let t170 = piecewise3(t2, 0.0, -15.0 / 64.0 * t168);
        let tvlapl0 = t5 * t170;
        vlapl[ip * 2] += tvlapl0;
        let t172 = t93 * t149 * t62;
        let t173 = t147 * t172;
        let t175 = piecewise3(t48, 0.0, -15.0 / 64.0 * t173);
        let tvlapl1 = t5 * t175;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t177 = piecewise3(t2, 0.0, 15.0 / 8.0 * t168);
        let tvtau0 = t5 * t177;
        vtau[ip * 2] += tvtau0;
        let t179 = piecewise3(t48, 0.0, 15.0 / 8.0 * t173);
        let tvtau1 = t5 * t179;
        vtau[ip * 2 + 1] += tvtau1;
        // --- fxc delta (this level) (153 lines) ---
        let t182 = t22 * t22;
        let t183 = 1.0 / t182;
        let t184 = t78 * t78;
        let t187 = t74 * t5;
        let t188 = 1.0 / t187;
        let t189 = t15 * t188;
        let t192 = piecewise5(t9, 0.0, t13, 0.0, -2.0 * t75 + 2.0 * t189);
        let t196 = piecewise3(t19, 0.0, 4.0 / 9.0 * t183 * t184 + 4.0 / 3.0 * t22 * t192);
        let t197 = t4 * t196;
        let t202 = t82 * t87 * t44;
        let t204 = t82 * t91;
        let t208 = 1.0 / t86 / t5;
        let t211 = 5.0 / 24.0 * t25 * t208 * t44;
        let t212 = t87 * param_prefactor;
        let t213 = t25 * t212;
        let t214 = t213 * t105;
        let t217 = 1.0 / t94 / t41;
        let t218 = t103 * t103;
        let t220 = t93 * t217 * t218;
        let t223 = t96 * rho0;
        let t225 = 1.0 / t34 / t223;
        let t230 = 80.0 / 9.0 * tau0 * t225 - 10.0 / 9.0 * lapl0 * t225;
        let t232 = t93 * t95 * t230;
        let t236 = piecewise3(t2, 0.0, -15.0 / 16.0 * t197 * t26 * t44 - 5.0 / 8.0 * t202 + 15.0 / 8.0 * t204 * t105 + t211 + 5.0 / 8.0 * t214 - 15.0 / 8.0 * t92 * t220 + 15.0 / 16.0 * t92 * t232);
        let t237 = t54 * t54;
        let t238 = 1.0 / t237;
        let t239 = t112 * t112;
        let t242 = t49 * t188;
        let t245 = piecewise5(t13, 0.0, t9, 0.0, 2.0 * t75 + 2.0 * t242);
        let t249 = piecewise3(t53, 0.0, 4.0 / 9.0 * t238 * t239 + 4.0 / 3.0 * t54 * t245);
        let t250 = t4 * t249;
        let t255 = t116 * t87 * t70;
        let t259 = 5.0 / 24.0 * t57 * t208 * t70;
        let t261 = piecewise3(t48, 0.0, -15.0 / 16.0 * t250 * t26 * t70 - 5.0 / 8.0 * t255 + t259);
        let tv2rho20 = 2.0 * t109 + 2.0 * t124 + t5 * (t236 + t261);
        v2rho2[ip * 3] += tv2rho20;
        let t264 = t183 * t128;
        let t268 = piecewise5(t9, 0.0, t13, 0.0, 2.0 * t189);
        let t272 = piecewise3(t19, 0.0, 4.0 / 9.0 * t264 * t78 + 4.0 / 3.0 * t22 * t268);
        let t273 = t4 * t272;
        let t278 = t132 * t87 * t44;
        let t280 = t132 * t91;
        let t286 = piecewise3(t2, 0.0, -15.0 / 16.0 * t273 * t26 * t44 - 5.0 / 16.0 * t278 + 15.0 / 16.0 * t280 * t105 - 5.0 / 16.0 * t202 + t211 + 5.0 / 16.0 * t214);
        let t287 = t238 * t139;
        let t291 = piecewise5(t13, 0.0, t9, 0.0, 2.0 * t242);
        let t295 = piecewise3(t53, 0.0, 4.0 / 9.0 * t287 * t112 + 4.0 / 3.0 * t54 * t291);
        let t296 = t4 * t295;
        let t301 = t143 * t87 * t70;
        let t304 = t116 * t91;
        let t307 = t57 * t212;
        let t308 = t307 * t159;
        let t311 = piecewise3(t48, 0.0, -15.0 / 16.0 * t296 * t26 * t70 - 5.0 / 16.0 * t301 - 5.0 / 16.0 * t255 + t259 + 15.0 / 16.0 * t304 * t159 + 5.0 / 16.0 * t308);
        let tv2rho21 = t109 + t124 + t137 + t163 + t5 * (t286 + t311);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t316 = t128 * t128;
        let t321 = piecewise5(t9, 0.0, t13, 0.0, 2.0 * t75 + 2.0 * t189);
        let t325 = piecewise3(t19, 0.0, 4.0 / 9.0 * t183 * t316 + 4.0 / 3.0 * t22 * t321);
        let t326 = t4 * t325;
        let t332 = piecewise3(t2, 0.0, -15.0 / 16.0 * t326 * t26 * t44 - 5.0 / 8.0 * t278 + t211);
        let t333 = t139 * t139;
        let t338 = piecewise5(t13, 0.0, t9, 0.0, -2.0 * t75 + 2.0 * t242);
        let t342 = piecewise3(t53, 0.0, 4.0 / 9.0 * t238 * t333 + 4.0 / 3.0 * t54 * t338);
        let t343 = t4 * t342;
        let t348 = t143 * t91;
        let t353 = 1.0 / t148 / t67;
        let t354 = t157 * t157;
        let t356 = t93 * t353 * t354;
        let t359 = t150 * rho1;
        let t361 = 1.0 / t60 / t359;
        let t366 = 80.0 / 9.0 * tau1 * t361 - 10.0 / 9.0 * lapl1 * t361;
        let t368 = t93 * t149 * t366;
        let t372 = piecewise3(t48, 0.0, -15.0 / 16.0 * t343 * t26 * t70 - 5.0 / 8.0 * t301 + 15.0 / 8.0 * t348 * t159 + t259 + 5.0 / 8.0 * t308 - 15.0 / 8.0 * t147 * t356 + 15.0 / 16.0 * t147 * t368);
        let tv2rho22 = 2.0 * t137 + 2.0 * t163 + t5 * (t332 + t372);
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
        let t375 = t204 * t167;
        let t377 = t213 * t167;
        let t378 = 5.0 / 64.0 * t377;
        let t379 = t217 * t36;
        let t381 = t93 * t379 * t103;
        let t382 = t92 * t381;
        let t385 = t93 * t95 * t98;
        let t386 = t92 * t385;
        let t389 = piecewise3(t2, 0.0, -15.0 / 64.0 * t375 - t378 + 15.0 / 32.0 * t382 + 25.0 / 64.0 * t386);
        let tv2rholapl0 = t5 * t389 + t170;
        v2rholapl[ip * 4] += tv2rholapl0;
        let t391 = t304 * t172;
        let t393 = t307 * t172;
        let t394 = 5.0 / 64.0 * t393;
        let t396 = piecewise3(t48, 0.0, -15.0 / 64.0 * t391 - t394);
        let tv2rholapl1 = t5 * t396 + t175;
        v2rholapl[ip * 4 + 1] += tv2rholapl1;
        let t398 = t280 * t167;
        let t401 = piecewise3(t2, 0.0, -15.0 / 64.0 * t398 - t378);
        let tv2rholapl2 = t5 * t401 + t170;
        v2rholapl[ip * 4 + 2] += tv2rholapl2;
        let t403 = t348 * t172;
        let t405 = t353 * t62;
        let t407 = t93 * t405 * t157;
        let t408 = t147 * t407;
        let t411 = t93 * t149 * t152;
        let t412 = t147 * t411;
        let t415 = piecewise3(t48, 0.0, -15.0 / 64.0 * t403 - t394 + 15.0 / 32.0 * t408 + 25.0 / 64.0 * t412);
        let tv2rholapl3 = t5 * t415 + t175;
        v2rholapl[ip * 4 + 3] += tv2rholapl3;
        let t418 = 5.0 / 8.0 * t377;
        let t422 = piecewise3(t2, 0.0, 15.0 / 8.0 * t375 + t418 - 15.0 / 4.0 * t382 - 25.0 / 8.0 * t386);
        let tv2rhotau0 = t5 * t422 + t177;
        v2rhotau[ip * 4] += tv2rhotau0;
        let t425 = 5.0 / 8.0 * t393;
        let t427 = piecewise3(t48, 0.0, 15.0 / 8.0 * t391 + t425);
        let tv2rhotau1 = t5 * t427 + t179;
        v2rhotau[ip * 4 + 1] += tv2rhotau1;
        let t431 = piecewise3(t2, 0.0, 15.0 / 8.0 * t398 + t418);
        let tv2rhotau2 = t5 * t431 + t177;
        v2rhotau[ip * 4 + 2] += tv2rhotau2;
        let t437 = piecewise3(t48, 0.0, 15.0 / 8.0 * t403 + t425 - 15.0 / 4.0 * t408 - 25.0 / 8.0 * t412);
        let tv2rhotau3 = t5 * t437 + t179;
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
        let t440 = 1.0 / t33 / t223;
        let t442 = t93 * t217 * t440;
        let t443 = t92 * t442;
        let t445 = piecewise3(t2, 0.0, -15.0 / 128.0 * t443);
        let tv2lapl20 = t5 * t445;
        v2lapl2[ip * 3] += tv2lapl20;
        let tv2lapl21 = 0.0;
        v2lapl2[ip * 3 + 1] += tv2lapl21;
        let t447 = 1.0 / t59 / t359;
        let t449 = t93 * t353 * t447;
        let t450 = t147 * t449;
        let t452 = piecewise3(t48, 0.0, -15.0 / 128.0 * t450);
        let tv2lapl22 = t5 * t452;
        v2lapl2[ip * 3 + 2] += tv2lapl22;
        let t454 = piecewise3(t2, 0.0, 15.0 / 16.0 * t443);
        let tv2lapltau0 = t5 * t454;
        v2lapltau[ip * 4] += tv2lapltau0;
        let tv2lapltau1 = 0.0;
        v2lapltau[ip * 4 + 1] += tv2lapltau1;
        let tv2lapltau2 = 0.0;
        v2lapltau[ip * 4 + 2] += tv2lapltau2;
        let t456 = piecewise3(t48, 0.0, 15.0 / 16.0 * t450);
        let tv2lapltau3 = t5 * t456;
        v2lapltau[ip * 4 + 3] += tv2lapltau3;
        let t458 = piecewise3(t2, 0.0, -15.0 / 2.0 * t443);
        let tv2tau20 = t5 * t458;
        v2tau2[ip * 3] += tv2tau20;
        let tv2tau21 = 0.0;
        v2tau2[ip * 3 + 1] += tv2tau21;
        let t460 = piecewise3(t48, 0.0, -15.0 / 2.0 * t450);
        let tv2tau22 = t5 * t460;
        v2tau2[ip * 3 + 2] += tv2tau22;
    }
}
