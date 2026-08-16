//! GGA_X_LSRPBE fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lsrpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lsrpbe_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_alpha: f64,
    param_kappa: f64,
    param_mu: f64,
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
        let t18 = piecewise5::<f64>(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = 1.0 / param_kappa;
        let t45 = f64::exp(-t34 * sigma0 * t39 * t41 / 24.0);
        let t48 = param_kappa + 1.0;
        let t49 = param_alpha * t28;
        let t50 = t33 * sigma0;
        let t54 = f64::exp(-t49 * t50 * t39 / 24.0);
        let t57 = 1.0 + param_kappa * (1.0 - t45) - t48 * (1.0 - t54);
        let t61 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t57);
        let t62 = rho1 <= dens_threshold;
        let t63 = -t16;
        let t65 = piecewise5::<f64>(t14, t11, t10, t15, t63 * t7);
        let t66 = 1.0 + t65;
        let t67 = t66 <= zeta_threshold;
        let t68 = pow_1_3::<f64>(t66);
        let t70 = piecewise3::<f64>(t67, t22, t68 * t66);
        let t71 = t70 * t26;
        let t72 = rho1 * rho1;
        let t73 = pow_1_3::<f64>(rho1);
        let t74 = t73 * t73;
        let t76 = 1.0 / t74 / t72;
        let t81 = f64::exp(-t34 * sigma2 * t76 * t41 / 24.0);
        let t84 = t33 * sigma2;
        let t88 = f64::exp(-t49 * t84 * t76 / 24.0);
        let t91 = 1.0 + param_kappa * (1.0 - t81) - t48 * (1.0 - t88);
        let t95 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t71 * t91);
        let tzk0 = t61 + t95;
        zk[ip] += tzk0;
        let t96 = t6 * t6;
        let t97 = 1.0 / t96;
        let t98 = t16 * t97;
        let t100 = piecewise5::<f64>(t10, 0.0, t14, 0.0, t7 - t98);
        let t103 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t100);
        let t104 = t103 * t26;
        let t108 = t26 * t26;
        let t109 = 1.0 / t108;
        let t110 = t25 * t109;
        let t113 = t5 * t110 * t57 / 8.0;
        let t114 = t35 * rho0;
        let t116 = 1.0 / t37 / t114;
        let t121 = t48 * param_alpha * t28;
        let t126 = t121 * t50 * t116 * t54 / 9.0 - t34 * sigma0 * t116 * t45 / 9.0;
        let t131 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t104 * t57 - t113 - 3.0 / 8.0 * t5 * t27 * t126);
        let t132 = t63 * t97;
        let t134 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -t7 - t132);
        let t137 = piecewise3::<f64>(t67, 0.0, 4.0 / 3.0 * t68 * t134);
        let t138 = t137 * t26;
        let t142 = t70 * t109;
        let t145 = t5 * t142 * t91 / 8.0;
        let t147 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t138 * t91 - t145);
        let tvrho0 = t61 + t95 + t6 * (t131 + t147);
        vrho[ip * 2] += tvrho0;
        let t151 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -t7 - t98);
        let t154 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t151);
        let t155 = t154 * t26;
        let t160 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t155 * t57 - t113);
        let t162 = piecewise5::<f64>(t14, 0.0, t10, 0.0, t7 - t132);
        let t165 = piecewise3::<f64>(t67, 0.0, 4.0 / 3.0 * t68 * t162);
        let t166 = t165 * t26;
        let t170 = t72 * rho1;
        let t172 = 1.0 / t74 / t170;
        let t180 = t121 * t84 * t172 * t88 / 9.0 - t34 * sigma2 * t172 * t81 / 9.0;
        let t185 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t166 * t91 - t145 - 3.0 / 8.0 * t5 * t71 * t180);
        let tvrho1 = t61 + t95 + t6 * (t160 + t185);
        vrho[ip * 2 + 1] += tvrho1;
        let t188 = t33 * t39;
        let t194 = -t121 * t188 * t54 / 24.0 + t29 * t188 * t45 / 24.0;
        let t198 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t194);
        let tvsigma0 = t6 * t198;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t199 = t33 * t76;
        let t205 = -t121 * t199 * t88 / 24.0 + t29 * t199 * t81 / 24.0;
        let t209 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t71 * t205);
        let tvsigma2 = t6 * t209;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t212 = t23 * t23;
        let t213 = 1.0 / t212;
        let t214 = t100 * t100;
        let t217 = t96 * t6;
        let t218 = 1.0 / t217;
        let t219 = t16 * t218;
        let t222 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -2.0 * t97 + 2.0 * t219);
        let t226 = piecewise3::<f64>(t20, 0.0, 4.0 / 9.0 * t213 * t214 + 4.0 / 3.0 * t23 * t222);
        let t227 = t226 * t26;
        let t231 = t103 * t109;
        let t233 = t5 * t231 * t57;
        let t239 = 1.0 / t108 / t6;
        let t240 = t25 * t239;
        let t243 = t5 * t240 * t57 / 12.0;
        let t245 = t5 * t110 * t126;
        let t247 = t35 * t35;
        let t249 = 1.0 / t37 / t247;
        let t254 = param_mu * param_mu;
        let t255 = t28 * t28;
        let t258 = 1.0 / t31 / t30;
        let t259 = t254 * t255 * t258;
        let t260 = sigma0 * sigma0;
        let t263 = 1.0 / t36 / t247 / t114;
        let t265 = t41 * t45;
        let t273 = param_alpha * param_alpha;
        let t275 = t48 * t273 * t255;
        let t276 = t258 * t260;
        let t281 = 11.0 / 27.0 * t34 * sigma0 * t249 * t45 - t259 * t260 * t263 * t265 / 81.0 - 11.0 / 27.0 * t121 * t50 * t249 * t54 + t275 * t276 * t263 * t54 / 81.0;
        let t286 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t227 * t57 - t233 / 4.0 - 3.0 / 4.0 * t5 * t104 * t126 + t243 - t245 / 4.0 - 3.0 / 8.0 * t5 * t27 * t281);
        let t287 = t68 * t68;
        let t288 = 1.0 / t287;
        let t289 = t134 * t134;
        let t292 = t63 * t218;
        let t295 = piecewise5::<f64>(t14, 0.0, t10, 0.0, 2.0 * t97 + 2.0 * t292);
        let t299 = piecewise3::<f64>(t67, 0.0, 4.0 / 9.0 * t288 * t289 + 4.0 / 3.0 * t68 * t295);
        let t300 = t299 * t26;
        let t304 = t137 * t109;
        let t306 = t5 * t304 * t91;
        let t308 = t70 * t239;
        let t311 = t5 * t308 * t91 / 12.0;
        let t313 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t300 * t91 - t306 / 4.0 + t311);
        let tv2rho20 = 2.0 * t131 + 2.0 * t147 + t6 * (t286 + t313);
        v2rho2[ip * 3] += tv2rho20;
        let t316 = t213 * t151;
        let t320 = piecewise5::<f64>(t10, 0.0, t14, 0.0, 2.0 * t219);
        let t324 = piecewise3::<f64>(t20, 0.0, 4.0 / 9.0 * t316 * t100 + 4.0 / 3.0 * t23 * t320);
        let t325 = t324 * t26;
        let t329 = t154 * t109;
        let t331 = t5 * t329 * t57;
        let t339 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t325 * t57 - t331 / 8.0 - 3.0 / 8.0 * t5 * t155 * t126 - t233 / 8.0 + t243 - t245 / 8.0);
        let t340 = t288 * t162;
        let t344 = piecewise5::<f64>(t14, 0.0, t10, 0.0, 2.0 * t292);
        let t348 = piecewise3::<f64>(t67, 0.0, 4.0 / 9.0 * t340 * t134 + 4.0 / 3.0 * t68 * t344);
        let t349 = t348 * t26;
        let t353 = t165 * t109;
        let t355 = t5 * t353 * t91;
        let t362 = t5 * t142 * t180;
        let t365 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t349 * t91 - t355 / 8.0 - t306 / 8.0 + t311 - 3.0 / 8.0 * t5 * t138 * t180 - t362 / 8.0);
        let tv2rho21 = t131 + t147 + t160 + t185 + t6 * (t339 + t365);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t370 = t151 * t151;
        let t375 = piecewise5::<f64>(t10, 0.0, t14, 0.0, 2.0 * t97 + 2.0 * t219);
        let t379 = piecewise3::<f64>(t20, 0.0, 4.0 / 9.0 * t213 * t370 + 4.0 / 3.0 * t23 * t375);
        let t380 = t379 * t26;
        let t386 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t380 * t57 - t331 / 4.0 + t243);
        let t387 = t162 * t162;
        let t392 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -2.0 * t97 + 2.0 * t292);
        let t396 = piecewise3::<f64>(t67, 0.0, 4.0 / 9.0 * t288 * t387 + 4.0 / 3.0 * t68 * t392);
        let t397 = t396 * t26;
        let t406 = t72 * t72;
        let t408 = 1.0 / t74 / t406;
        let t413 = sigma2 * sigma2;
        let t416 = 1.0 / t73 / t406 / t170;
        let t418 = t41 * t81;
        let t426 = t258 * t413;
        let t431 = 11.0 / 27.0 * t34 * sigma2 * t408 * t81 - t259 * t413 * t416 * t418 / 81.0 - 11.0 / 27.0 * t121 * t84 * t408 * t88 + t275 * t426 * t416 * t88 / 81.0;
        let t436 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t397 * t91 - t355 / 4.0 - 3.0 / 4.0 * t5 * t166 * t180 + t311 - t362 / 4.0 - 3.0 / 8.0 * t5 * t71 * t431);
        let tv2rho22 = 2.0 * t160 + 2.0 * t185 + t6 * (t386 + t436);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t444 = t5 * t110 * t194 / 8.0;
        let t445 = t33 * t116;
        let t449 = t247 * t35;
        let t451 = 1.0 / t36 / t449;
        let t459 = t258 * t451;
        let t460 = sigma0 * t54;
        let t464 = -t29 * t445 * t45 / 9.0 + t259 * t451 * sigma0 * t265 / 216.0 + t121 * t445 * t54 / 9.0 - t275 * t459 * t460 / 216.0;
        let t469 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t104 * t194 - t444 - 3.0 / 8.0 * t5 * t27 * t464);
        let tv2rhosigma0 = t6 * t469 + t198;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t476 = t5 * t142 * t205 / 8.0;
        let t478 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t138 * t205 - t476);
        let tv2rhosigma2 = t6 * t478 + t209;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t484 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t155 * t194 - t444);
        let tv2rhosigma3 = t6 * t484 + t198;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t489 = t33 * t172;
        let t493 = t406 * t72;
        let t495 = 1.0 / t73 / t493;
        let t503 = t258 * t495;
        let t504 = sigma2 * t88;
        let t508 = -t29 * t489 * t81 / 9.0 + t259 * t495 * sigma2 * t418 / 216.0 + t121 * t489 * t88 / 9.0 - t275 * t503 * t504 / 216.0;
        let t513 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t166 * t205 - t476 - 3.0 / 8.0 * t5 * t71 * t508);
        let tv2rhosigma5 = t6 * t513 + t209;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t515 = t247 * rho0;
        let t517 = 1.0 / t36 / t515;
        let t525 = t275 * t258 * t517 * t54 / 576.0 - t259 * t517 * t41 * t45 / 576.0;
        let t529 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t525);
        let tv2sigma20 = t6 * t529;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t530 = t406 * rho1;
        let t532 = 1.0 / t73 / t530;
        let t540 = t275 * t258 * t532 * t88 / 576.0 - t259 * t532 * t41 * t81 / 576.0;
        let t544 = piecewise3::<f64>(t62, 0.0, -3.0 / 8.0 * t5 * t71 * t540);
        let tv2sigma25 = t6 * t544;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
