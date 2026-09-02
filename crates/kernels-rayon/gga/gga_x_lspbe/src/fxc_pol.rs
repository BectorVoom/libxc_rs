//! GGA_X_LSPBE fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lspbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lspbe_fxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_mu: f64,
    param_kappa: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = t34 * t39;
        let t43 = param_kappa + t29 * t40 / 24.0;
        let t48 = param_kappa + 1.0;
        let t49 = param_alpha * t28;
        let t52 = rmath::exp(-t49 * t40 / 24.0);
        let t55 = 1.0 + param_kappa * (1.0 - param_kappa / t43) - t48 * (1.0 - t52);
        let t59 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t16;
        let t63 = piecewise5(t14, t11, t10, t15, t61 * t7);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t22, t66 * t64);
        let t69 = t68 * t26;
        let t70 = t33 * sigma2;
        let t71 = rho1 * rho1;
        let t72 = pow_1_3(rho1);
        let t73 = t72 * t72;
        let t75 = 1.0 / t73 / t71;
        let t76 = t70 * t75;
        let t79 = param_kappa + t29 * t76 / 24.0;
        let t86 = rmath::exp(-t49 * t76 / 24.0);
        let t89 = 1.0 + param_kappa * (1.0 - param_kappa / t79) - t48 * (1.0 - t86);
        let t93 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t89);
        let tzk0 = t59 + t93;
        zk[ip] += tzk0;
        let t94 = t6 * t6;
        let t95 = 1.0 / t94;
        let t96 = t16 * t95;
        let t98 = piecewise5(t10, 0.0, t14, 0.0, t7 - t96);
        let t101 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t98);
        let t102 = t101 * t26;
        let t106 = t26 * t26;
        let t107 = 1.0 / t106;
        let t108 = t25 * t107;
        let t111 = t5 * t108 * t55 / 8.0;
        let t112 = param_kappa * param_kappa;
        let t113 = t43 * t43;
        let t116 = t112 / t113 * param_mu;
        let t117 = t28 * t33;
        let t118 = t35 * rho0;
        let t120 = 1.0 / t37 / t118;
        let t125 = t48 * param_alpha * t28;
        let t130 = -t116 * t117 * sigma0 * t120 / 9.0 + t125 * t34 * t120 * t52 / 9.0;
        let t135 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t102 * t55 - t111 - 3.0 / 8.0 * t5 * t27 * t130);
        let t136 = t61 * t95;
        let t138 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t136);
        let t141 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t138);
        let t142 = t141 * t26;
        let t146 = t68 * t107;
        let t149 = t5 * t146 * t89 / 8.0;
        let t151 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t142 * t89 - t149);
        let tvrho0 = t59 + t93 + t6 * (t135 + t151);
        vrho[ip * 2] += tvrho0;
        let t155 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t96);
        let t158 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t155);
        let t159 = t158 * t26;
        let t164 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t159 * t55 - t111);
        let t166 = piecewise5(t14, 0.0, t10, 0.0, t7 - t136);
        let t169 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t166);
        let t170 = t169 * t26;
        let t174 = t79 * t79;
        let t177 = t112 / t174 * param_mu;
        let t178 = t71 * rho1;
        let t180 = 1.0 / t73 / t178;
        let t188 = -t177 * t117 * sigma2 * t180 / 9.0 + t125 * t70 * t180 * t86 / 9.0;
        let t193 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t170 * t89 - t149 - 3.0 / 8.0 * t5 * t69 * t188);
        let tvrho1 = t59 + t93 + t6 * (t164 + t193);
        vrho[ip * 2 + 1] += tvrho1;
        let t202 = -t125 * t33 * t39 * t52 / 24.0 + t116 * t117 * t39 / 24.0;
        let t206 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t202);
        let tvsigma0 = t6 * t206;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t213 = -t125 * t33 * t75 * t86 / 24.0 + t177 * t117 * t75 / 24.0;
        let t217 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t213);
        let tvsigma2 = t6 * t217;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t220 = t23 * t23;
        let t221 = 1.0 / t220;
        let t222 = t98 * t98;
        let t225 = t94 * t6;
        let t226 = 1.0 / t225;
        let t227 = t16 * t226;
        let t230 = piecewise5(t10, 0.0, t14, 0.0, -2.0 * t95 + 2.0 * t227);
        let t234 = piecewise3(t20, 0.0, 4.0 / 9.0 * t221 * t222 + 4.0 / 3.0 * t23 * t230);
        let t235 = t234 * t26;
        let t239 = t101 * t107;
        let t241 = t5 * t239 * t55;
        let t247 = 1.0 / t106 / t6;
        let t248 = t25 * t247;
        let t251 = t5 * t248 * t55 / 12.0;
        let t253 = t5 * t108 * t130;
        let t258 = param_mu * param_mu;
        let t259 = t112 / t113 / t43 * t258;
        let t260 = t28 * t28;
        let t262 = 1.0 / t31 / t30;
        let t263 = t260 * t262;
        let t264 = sigma0 * sigma0;
        let t265 = t35 * t35;
        let t268 = 1.0 / t36 / t265 / t118;
        let t274 = 1.0 / t37 / t265;
        let t283 = param_alpha * param_alpha;
        let t285 = t48 * t283 * t260;
        let t286 = t262 * t264;
        let t291 = -2.0 / 81.0 * t259 * t263 * t264 * t268 + 11.0 / 27.0 * t116 * t117 * sigma0 * t274 - 11.0 / 27.0 * t125 * t34 * t274 * t52 + t285 * t286 * t268 * t52 / 81.0;
        let t296 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t235 * t55 - t241 / 4.0 - 3.0 / 4.0 * t5 * t102 * t130 + t251 - t253 / 4.0 - 3.0 / 8.0 * t5 * t27 * t291);
        let t297 = t66 * t66;
        let t298 = 1.0 / t297;
        let t299 = t138 * t138;
        let t302 = t61 * t226;
        let t305 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t95 + 2.0 * t302);
        let t309 = piecewise3(t65, 0.0, 4.0 / 9.0 * t298 * t299 + 4.0 / 3.0 * t66 * t305);
        let t310 = t309 * t26;
        let t314 = t141 * t107;
        let t316 = t5 * t314 * t89;
        let t318 = t68 * t247;
        let t321 = t5 * t318 * t89 / 12.0;
        let t323 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t310 * t89 - t316 / 4.0 + t321);
        let tv2rho20 = 2.0 * t135 + 2.0 * t151 + t6 * (t296 + t323);
        v2rho2[ip * 3] += tv2rho20;
        let t326 = t221 * t155;
        let t330 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t227);
        let t334 = piecewise3(t20, 0.0, 4.0 / 9.0 * t326 * t98 + 4.0 / 3.0 * t23 * t330);
        let t335 = t334 * t26;
        let t339 = t158 * t107;
        let t341 = t5 * t339 * t55;
        let t349 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t335 * t55 - t341 / 8.0 - 3.0 / 8.0 * t5 * t159 * t130 - t241 / 8.0 + t251 - t253 / 8.0);
        let t350 = t298 * t166;
        let t354 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t302);
        let t358 = piecewise3(t65, 0.0, 4.0 / 9.0 * t350 * t138 + 4.0 / 3.0 * t66 * t354);
        let t359 = t358 * t26;
        let t363 = t169 * t107;
        let t365 = t5 * t363 * t89;
        let t372 = t5 * t146 * t188;
        let t375 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t359 * t89 - t365 / 8.0 - t316 / 8.0 + t321 - 3.0 / 8.0 * t5 * t142 * t188 - t372 / 8.0);
        let tv2rho21 = t135 + t151 + t164 + t193 + t6 * (t349 + t375);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t380 = t155 * t155;
        let t385 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t95 + 2.0 * t227);
        let t389 = piecewise3(t20, 0.0, 4.0 / 9.0 * t221 * t380 + 4.0 / 3.0 * t23 * t385);
        let t390 = t389 * t26;
        let t396 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t390 * t55 - t341 / 4.0 + t251);
        let t397 = t166 * t166;
        let t402 = piecewise5(t14, 0.0, t10, 0.0, -2.0 * t95 + 2.0 * t302);
        let t406 = piecewise3(t65, 0.0, 4.0 / 9.0 * t298 * t397 + 4.0 / 3.0 * t66 * t402);
        let t407 = t406 * t26;
        let t419 = t112 / t174 / t79 * t258;
        let t420 = sigma2 * sigma2;
        let t421 = t71 * t71;
        let t424 = 1.0 / t72 / t421 / t178;
        let t430 = 1.0 / t73 / t421;
        let t439 = t262 * t420;
        let t444 = -2.0 / 81.0 * t419 * t263 * t420 * t424 + 11.0 / 27.0 * t177 * t117 * sigma2 * t430 - 11.0 / 27.0 * t125 * t70 * t430 * t86 + t285 * t439 * t424 * t86 / 81.0;
        let t449 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t407 * t89 - t365 / 4.0 - 3.0 / 4.0 * t5 * t170 * t188 + t321 - t372 / 4.0 - 3.0 / 8.0 * t5 * t69 * t444);
        let tv2rho22 = 2.0 * t164 + 2.0 * t193 + t6 * (t396 + t449);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t457 = t5 * t108 * t202 / 8.0;
        let t458 = t265 * t35;
        let t460 = 1.0 / t36 / t458;
        let t472 = t262 * t460;
        let t473 = sigma0 * t52;
        let t477 = t259 * t263 * t460 * sigma0 / 108.0 - t116 * t117 * t120 / 9.0 + t125 * t33 * t120 * t52 / 9.0 - t285 * t472 * t473 / 216.0;
        let t482 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t102 * t202 - t457 - 3.0 / 8.0 * t5 * t27 * t477);
        let tv2rhosigma0 = t6 * t482 + t206;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t489 = t5 * t146 * t213 / 8.0;
        let t491 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t142 * t213 - t489);
        let tv2rhosigma2 = t6 * t491 + t217;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t497 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t159 * t202 - t457);
        let tv2rhosigma3 = t6 * t497 + t206;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t502 = t421 * t71;
        let t504 = 1.0 / t72 / t502;
        let t516 = t262 * t504;
        let t517 = sigma2 * t86;
        let t521 = t419 * t263 * t504 * sigma2 / 108.0 - t177 * t117 * t180 / 9.0 + t125 * t33 * t180 * t86 / 9.0 - t285 * t516 * t517 / 216.0;
        let t526 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t170 * t213 - t489 - 3.0 / 8.0 * t5 * t69 * t521);
        let tv2rhosigma5 = t6 * t526 + t217;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t528 = t265 * rho0;
        let t530 = 1.0 / t36 / t528;
        let t538 = -t259 * t263 * t530 / 288.0 + t285 * t262 * t530 * t52 / 576.0;
        let t542 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t538);
        let tv2sigma20 = t6 * t542;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t543 = t421 * rho1;
        let t545 = 1.0 / t72 / t543;
        let t553 = -t419 * t263 * t545 / 288.0 + t285 * t262 * t545 * t86 / 576.0;
        let t557 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t553);
        let tv2sigma25 = t6 * t557;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
