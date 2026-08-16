//! MGGA_K_PGSLB fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pgslb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_k_pgslb_fxc_pol(
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
    param_pgslb_beta: f64,
    param_pgslb_mu: f64,
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
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5::<f64>(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3::<f64>(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3::<f64>(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3::<f64>(t22, t25, t27 * t21);
        let t30 = pow_1_3::<f64>(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3::<f64>(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3::<f64>(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t47 = param_pgslb_mu * t33;
        let t48 = t37 * sigma0;
        let t52 = f64::exp(-t47 * t48 * t43 / 24.0);
        let t53 = t33 * t33;
        let t54 = param_pgslb_beta * t53;
        let t56 = 1.0 / t35 / t34;
        let t57 = lapl0 * lapl0;
        let t58 = t56 * t57;
        let t59 = t39 * rho0;
        let t61 = 1.0 / t40 / t59;
        let t65 = 5.0 / 72.0 * t38 * sigma0 * t43 + t52 + t54 * t58 * t61 / 576.0;
        let t69 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t65);
        let t70 = rho1 <= dens_threshold;
        let t71 = -t18;
        let t73 = piecewise5::<f64>(t16, t13, t12, t17, t71 * t9);
        let t74 = 1.0 + t73;
        let t75 = t74 <= zeta_threshold;
        let t76 = pow_1_3::<f64>(t74);
        let t77 = t76 * t76;
        let t79 = piecewise3::<f64>(t75, t25, t77 * t74);
        let t80 = t79 * t31;
        let t81 = rho1 * rho1;
        let t82 = pow_1_3::<f64>(rho1);
        let t83 = t82 * t82;
        let t85 = 1.0 / t83 / t81;
        let t89 = t37 * sigma2;
        let t93 = f64::exp(-t47 * t89 * t85 / 24.0);
        let t94 = lapl1 * lapl1;
        let t95 = t56 * t94;
        let t96 = t81 * rho1;
        let t98 = 1.0 / t82 / t96;
        let t102 = 5.0 / 72.0 * t38 * sigma2 * t85 + t93 + t54 * t95 * t98 / 576.0;
        let t106 = piecewise3::<f64>(t70, 0.0, 3.0 / 20.0 * t7 * t80 * t102);
        let tzk0 = t69 + t106;
        zk[ip] += tzk0;
        let t107 = t8 * t8;
        let t108 = 1.0 / t107;
        let t109 = t18 * t108;
        let t111 = piecewise5::<f64>(t12, 0.0, t16, 0.0, t9 - t109);
        let t114 = piecewise3::<f64>(t22, 0.0, 5.0 / 3.0 * t27 * t111);
        let t115 = t114 * t31;
        let t119 = 1.0 / t30;
        let t120 = t29 * t119;
        let t123 = t7 * t120 * t65 / 10.0;
        let t125 = 1.0 / t41 / t59;
        let t126 = sigma0 * t125;
        let t129 = t47 * t37;
        let t133 = t39 * t39;
        let t135 = 1.0 / t40 / t133;
        let t139 = -5.0 / 27.0 * t38 * t126 + t129 * t126 * t52 / 9.0 - 5.0 / 864.0 * t54 * t58 * t135;
        let t144 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t115 * t65 + t123 + 3.0 / 20.0 * t7 * t32 * t139);
        let t145 = t71 * t108;
        let t147 = piecewise5::<f64>(t16, 0.0, t12, 0.0, -t9 - t145);
        let t150 = piecewise3::<f64>(t75, 0.0, 5.0 / 3.0 * t77 * t147);
        let t151 = t150 * t31;
        let t155 = t79 * t119;
        let t158 = t7 * t155 * t102 / 10.0;
        let t160 = piecewise3::<f64>(t70, 0.0, 3.0 / 20.0 * t7 * t151 * t102 + t158);
        let tvrho0 = t69 + t106 + t8 * (t144 + t160);
        vrho[ip * 2] += tvrho0;
        let t164 = piecewise5::<f64>(t12, 0.0, t16, 0.0, -t9 - t109);
        let t167 = piecewise3::<f64>(t22, 0.0, 5.0 / 3.0 * t27 * t164);
        let t168 = t167 * t31;
        let t173 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t168 * t65 + t123);
        let t175 = piecewise5::<f64>(t16, 0.0, t12, 0.0, t9 - t145);
        let t178 = piecewise3::<f64>(t75, 0.0, 5.0 / 3.0 * t77 * t175);
        let t179 = t178 * t31;
        let t184 = 1.0 / t83 / t96;
        let t185 = sigma2 * t184;
        let t191 = t81 * t81;
        let t193 = 1.0 / t82 / t191;
        let t197 = -5.0 / 27.0 * t38 * t185 + t129 * t185 * t93 / 9.0 - 5.0 / 864.0 * t54 * t95 * t193;
        let t202 = piecewise3::<f64>(t70, 0.0, 3.0 / 20.0 * t7 * t179 * t102 + t158 + 3.0 / 20.0 * t7 * t80 * t197);
        let tvrho1 = t69 + t106 + t8 * (t173 + t202);
        vrho[ip * 2 + 1] += tvrho1;
        let t211 = 5.0 / 72.0 * t38 * t43 - t47 * t37 * t43 * t52 / 24.0;
        let t215 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t211);
        let tvsigma0 = t8 * t215;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t222 = 5.0 / 72.0 * t38 * t85 - t47 * t37 * t85 * t93 / 24.0;
        let t226 = piecewise3::<f64>(t70, 0.0, 3.0 / 20.0 * t7 * t80 * t222);
        let tvsigma2 = t8 * t226;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t227 = t7 * t32;
        let t228 = t56 * lapl0;
        let t230 = t54 * t228 * t61;
        let t233 = piecewise3::<f64>(t2, 0.0, t227 * t230 / 1920.0);
        let tvlapl0 = t8 * t233;
        vlapl[ip * 2] += tvlapl0;
        let t234 = t7 * t80;
        let t235 = t56 * lapl1;
        let t237 = t54 * t235 * t98;
        let t240 = piecewise3::<f64>(t70, 0.0, t234 * t237 / 1920.0);
        let tvlapl1 = t8 * t240;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
        let t243 = 1.0 / t26;
        let t244 = t111 * t111;
        let t247 = t107 * t8;
        let t248 = 1.0 / t247;
        let t249 = t18 * t248;
        let t252 = piecewise5::<f64>(t12, 0.0, t16, 0.0, -2.0 * t108 + 2.0 * t249);
        let t256 = piecewise3::<f64>(t22, 0.0, 10.0 / 9.0 * t243 * t244 + 5.0 / 3.0 * t27 * t252);
        let t257 = t256 * t31;
        let t261 = t114 * t119;
        let t263 = t7 * t261 * t65;
        let t269 = 1.0 / t30 / t8;
        let t270 = t29 * t269;
        let t273 = t7 * t270 * t65 / 30.0;
        let t275 = t7 * t120 * t139;
        let t278 = 1.0 / t41 / t133;
        let t279 = sigma0 * t278;
        let t285 = param_pgslb_mu * param_pgslb_mu;
        let t286 = t285 * t53;
        let t287 = t286 * t56;
        let t288 = sigma0 * sigma0;
        let t291 = 1.0 / t40 / t133 / t59;
        let t296 = t133 * rho0;
        let t298 = 1.0 / t40 / t296;
        let t302 = 55.0 / 81.0 * t38 * t279 - 11.0 / 27.0 * t129 * t279 * t52 + t287 * t288 * t291 * t52 / 81.0 + 65.0 / 2592.0 * t54 * t58 * t298;
        let t307 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t257 * t65 + t263 / 5.0 + 3.0 / 10.0 * t7 * t115 * t139 - t273 + t275 / 5.0 + 3.0 / 20.0 * t7 * t32 * t302);
        let t308 = 1.0 / t76;
        let t309 = t147 * t147;
        let t312 = t71 * t248;
        let t315 = piecewise5::<f64>(t16, 0.0, t12, 0.0, 2.0 * t108 + 2.0 * t312);
        let t319 = piecewise3::<f64>(t75, 0.0, 10.0 / 9.0 * t308 * t309 + 5.0 / 3.0 * t77 * t315);
        let t320 = t319 * t31;
        let t324 = t150 * t119;
        let t326 = t7 * t324 * t102;
        let t328 = t79 * t269;
        let t331 = t7 * t328 * t102 / 30.0;
        let t333 = piecewise3::<f64>(t70, 0.0, 3.0 / 20.0 * t7 * t320 * t102 + t326 / 5.0 - t331);
        let tv2rho20 = 2.0 * t144 + 2.0 * t160 + t8 * (t307 + t333);
        v2rho2[ip * 3] += tv2rho20;
        let t336 = t243 * t164;
        let t340 = piecewise5::<f64>(t12, 0.0, t16, 0.0, 2.0 * t249);
        let t344 = piecewise3::<f64>(t22, 0.0, 10.0 / 9.0 * t336 * t111 + 5.0 / 3.0 * t27 * t340);
        let t345 = t344 * t31;
        let t349 = t167 * t119;
        let t351 = t7 * t349 * t65;
        let t359 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t345 * t65 + t351 / 10.0 + 3.0 / 20.0 * t7 * t168 * t139 + t263 / 10.0 - t273 + t275 / 10.0);
        let t360 = t308 * t175;
        let t364 = piecewise5::<f64>(t16, 0.0, t12, 0.0, 2.0 * t312);
        let t368 = piecewise3::<f64>(t75, 0.0, 10.0 / 9.0 * t360 * t147 + 5.0 / 3.0 * t77 * t364);
        let t369 = t368 * t31;
        let t373 = t178 * t119;
        let t375 = t7 * t373 * t102;
        let t382 = t7 * t155 * t197;
        let t385 = piecewise3::<f64>(t70, 0.0, 3.0 / 20.0 * t7 * t369 * t102 + t375 / 10.0 + t326 / 10.0 - t331 + 3.0 / 20.0 * t7 * t151 * t197 + t382 / 10.0);
        let tv2rho21 = t144 + t160 + t173 + t202 + t8 * (t359 + t385);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t390 = t164 * t164;
        let t395 = piecewise5::<f64>(t12, 0.0, t16, 0.0, 2.0 * t108 + 2.0 * t249);
        let t399 = piecewise3::<f64>(t22, 0.0, 10.0 / 9.0 * t243 * t390 + 5.0 / 3.0 * t27 * t395);
        let t400 = t399 * t31;
        let t406 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t400 * t65 + t351 / 5.0 - t273);
        let t407 = t175 * t175;
        let t412 = piecewise5::<f64>(t16, 0.0, t12, 0.0, -2.0 * t108 + 2.0 * t312);
        let t416 = piecewise3::<f64>(t75, 0.0, 10.0 / 9.0 * t308 * t407 + 5.0 / 3.0 * t77 * t412);
        let t417 = t416 * t31;
        let t427 = 1.0 / t83 / t191;
        let t428 = sigma2 * t427;
        let t434 = sigma2 * sigma2;
        let t437 = 1.0 / t82 / t191 / t96;
        let t442 = t191 * rho1;
        let t444 = 1.0 / t82 / t442;
        let t448 = 55.0 / 81.0 * t38 * t428 - 11.0 / 27.0 * t129 * t428 * t93 + t287 * t434 * t437 * t93 / 81.0 + 65.0 / 2592.0 * t54 * t95 * t444;
        let t453 = piecewise3::<f64>(t70, 0.0, 3.0 / 20.0 * t7 * t417 * t102 + t375 / 5.0 + 3.0 / 10.0 * t7 * t179 * t197 - t331 + t382 / 5.0 + 3.0 / 20.0 * t7 * t80 * t448);
        let tv2rho22 = 2.0 * t173 + 2.0 * t202 + t8 * (t406 + t453);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t461 = t7 * t120 * t211 / 10.0;
        let t468 = t133 * t39;
        let t470 = 1.0 / t40 / t468;
        let t475 = -5.0 / 27.0 * t38 * t125 + t47 * t37 * t125 * t52 / 9.0 - t287 * t470 * sigma0 * t52 / 216.0;
        let t480 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t115 * t211 + t461 + 3.0 / 20.0 * t7 * t32 * t475);
        let tv2rhosigma0 = t8 * t480 + t215;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t487 = t7 * t155 * t222 / 10.0;
        let t489 = piecewise3::<f64>(t70, 0.0, 3.0 / 20.0 * t7 * t151 * t222 + t487);
        let tv2rhosigma2 = t8 * t489 + t226;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t495 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t168 * t211 + t461);
        let tv2rhosigma3 = t8 * t495 + t215;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t506 = t191 * t81;
        let t508 = 1.0 / t82 / t506;
        let t513 = -5.0 / 27.0 * t38 * t184 + t47 * t37 * t184 * t93 / 9.0 - t287 * t508 * sigma2 * t93 / 216.0;
        let t518 = piecewise3::<f64>(t70, 0.0, 3.0 / 20.0 * t7 * t179 * t222 + t487 + 3.0 / 20.0 * t7 * t80 * t513);
        let tv2rhosigma5 = t8 * t518 + t226;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t520 = t7 * t115;
        let t523 = t7 * t120;
        let t525 = t523 * t230 / 2880.0;
        let t527 = t54 * t228 * t135;
        let t531 = piecewise3::<f64>(t2, 0.0, t520 * t230 / 1920.0 + t525 - t227 * t527 / 576.0);
        let tv2rholapl0 = t8 * t531 + t233;
        v2rholapl[ip * 4] += tv2rholapl0;
        let t533 = t7 * t151;
        let t536 = t7 * t155;
        let t538 = t536 * t237 / 2880.0;
        let t540 = piecewise3::<f64>(t70, 0.0, t533 * t237 / 1920.0 + t538);
        let tv2rholapl1 = t8 * t540 + t240;
        v2rholapl[ip * 4 + 1] += tv2rholapl1;
        let t542 = t7 * t168;
        let t546 = piecewise3::<f64>(t2, 0.0, t542 * t230 / 1920.0 + t525);
        let tv2rholapl2 = t8 * t546 + t233;
        v2rholapl[ip * 4 + 2] += tv2rholapl2;
        let t548 = t7 * t179;
        let t552 = t54 * t235 * t193;
        let t556 = piecewise3::<f64>(t70, 0.0, t548 * t237 / 1920.0 + t538 - t234 * t552 / 576.0);
        let tv2rholapl3 = t8 * t556 + t240;
        v2rholapl[ip * 4 + 3] += tv2rholapl3;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip * 4] += tv2rhotau0;
        let tv2rhotau1 = 0.0;
        v2rhotau[ip * 4 + 1] += tv2rhotau1;
        let tv2rhotau2 = 0.0;
        v2rhotau[ip * 4 + 2] += tv2rhotau2;
        let tv2rhotau3 = 0.0;
        v2rhotau[ip * 4 + 3] += tv2rhotau3;
        let t558 = t56 * t298;
        let t560 = t286 * t558 * t52;
        let t563 = piecewise3::<f64>(t2, 0.0, t227 * t560 / 3840.0);
        let tv2sigma20 = t8 * t563;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t564 = t56 * t444;
        let t566 = t286 * t564 * t93;
        let t569 = piecewise3::<f64>(t70, 0.0, t234 * t566 / 3840.0);
        let tv2sigma25 = t8 * t569;
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
        let t571 = t54 * t56 * t61;
        let t574 = piecewise3::<f64>(t2, 0.0, t227 * t571 / 1920.0);
        let tv2lapl20 = t8 * t574;
        v2lapl2[ip * 3] += tv2lapl20;
        let tv2lapl21 = 0.0;
        v2lapl2[ip * 3 + 1] += tv2lapl21;
        let t576 = t54 * t56 * t98;
        let t579 = piecewise3::<f64>(t70, 0.0, t234 * t576 / 1920.0);
        let tv2lapl22 = t8 * t579;
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
