//! GGA_X_PBEINT kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbeint.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbeint_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    param_alpha: f64,
    param_kappa: f64,
    param_muGE: f64,
    param_muPBE: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t19 = t17 * t18;
        let t20 = param_muPBE - param_muGE;
        let t21 = t20 * param_alpha;
        let t22 = M_CBRT6;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3::<f64>(t23);
        let t25 = t24 * t24;
        let t26 = 1.0 / t25;
        let t27 = t22 * t26;
        let t28 = t21 * t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = sigma[ip] * t30;
        let t32 = rho[ip] * rho[ip];
        let t33 = t18 * t18;
        let t35 = 1.0 / t33 / t32;
        let t38 = t31 * t35;
        let t41 = 1.0 + param_alpha * t22 * t26 * t38 / 24.0;
        let t42 = 1.0 / t41;
        let t43 = t35 * t42;
        let t48 = (param_muGE + t28 * t31 * t43 / 24.0) * t22;
        let t49 = t48 * t26;
        let t52 = param_kappa + t49 * t38 / 24.0;
        let t57 = 1.0 + param_kappa * (1.0 - param_kappa / t52);
        let t61 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t57);
        let tzk0 = 2.0 * t61;
        zk[ip] += tzk0;
        let t62 = 1.0 / t33;
        let t63 = t17 * t62;
        let t67 = t6 * t17;
        let t68 = param_kappa * param_kappa;
        let t69 = t18 * t68;
        let t70 = t52 * t52;
        let t71 = 1.0 / t70;
        let t72 = t32 * rho[ip];
        let t74 = 1.0 / t33 / t72;
        let t75 = t74 * t42;
        let t79 = param_alpha * param_alpha;
        let t80 = t20 * t79;
        let t81 = t22 * t22;
        let t83 = 1.0 / t24 / t23;
        let t84 = t81 * t83;
        let t85 = t80 * t84;
        let t86 = sigma[ip] * sigma[ip];
        let t87 = t86 * t29;
        let t88 = t32 * t32;
        let t89 = t88 * t32;
        let t91 = 1.0 / t18 / t89;
        let t92 = t41 * t41;
        let t93 = 1.0 / t92;
        let t94 = t91 * t93;
        let t99 = (-t28 * t31 * t75 / 9.0 + t85 * t87 * t94 / 108.0) * t22;
        let t100 = t99 * t26;
        let t103 = t31 * t74;
        let t106 = t100 * t38 / 24.0 - t49 * t103 / 9.0;
        let t107 = t71 * t106;
        let t112 = piecewise3::<f64>(t2, 0.0, -t6 * t63 * t57 / 8.0 - 3.0 / 8.0 * t67 * t69 * t107);
        let tvrho0 = 2.0 * rho[ip] * t112 + 2.0 * t61;
        vrho[ip] += tvrho0;
        let t115 = t21 * t22;
        let t116 = t26 * t30;
        let t121 = t88 * rho[ip];
        let t124 = 1.0 / t18 / t121 * t93;
        let t129 = (t115 * t116 * t43 / 24.0 - t85 * sigma[ip] * t29 * t124 / 288.0) * t22;
        let t130 = t129 * t26;
        let t132 = t116 * t35;
        let t135 = t130 * t38 / 24.0 + t48 * t132 / 24.0;
        let t136 = t71 * t135;
        let t140 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t67 * t69 * t136);
        let tvsigma0 = 2.0 * rho[ip] * t140;
        vsigma[ip] += tvsigma0;
        let t144 = 1.0 / t33 / rho[ip];
        let t145 = t17 * t144;
        let t149 = t62 * t68;
        let t154 = 1.0 / t70 / t52;
        let t155 = t106 * t106;
        let t156 = t154 * t155;
        let t161 = 1.0 / t33 / t88;
        let t162 = t161 * t42;
        let t166 = t88 * t72;
        let t168 = 1.0 / t18 / t166;
        let t169 = t168 * t93;
        let t174 = t20 * t79 * param_alpha;
        let t175 = t23 * t23;
        let t176 = 1.0 / t175;
        let t177 = t174 * t176;
        let t178 = t86 * sigma[ip];
        let t179 = t88 * t88;
        let t180 = t179 * t32;
        let t181 = 1.0 / t180;
        let t184 = 1.0 / t92 / t41;
        let t189 = (11.0 / 27.0 * t28 * t31 * t162 - t85 * t87 * t169 / 12.0 + 2.0 / 81.0 * t177 * t178 * t181 * t184) * t22;
        let t190 = t189 * t26;
        let t195 = t31 * t161;
        let t198 = t190 * t38 / 24.0 - 2.0 / 9.0 * t100 * t103 + 11.0 / 27.0 * t49 * t195;
        let t199 = t71 * t198;
        let t204 = piecewise3::<f64>(t2, 0.0, t6 * t145 * t57 / 12.0 - t67 * t149 * t107 / 4.0 + 3.0 / 4.0 * t67 * t69 * t156 - 3.0 / 8.0 * t67 * t69 * t199);
        let tv2rho20 = 2.0 * rho[ip] * t204 + 4.0 * t112;
        v2rho2[ip] += tv2rho20;
        let t210 = t6 * t19;
        let t211 = t68 * t154;
        let t212 = t135 * t106;
        let t213 = t211 * t212;
        let t220 = t93 * sigma[ip];
        let t224 = t179 * rho[ip];
        let t225 = 1.0 / t224;
        let t231 = (-t115 * t116 * t75 / 9.0 + t85 * t29 * t91 * t220 / 36.0 - t177 * t86 * t225 * t184 / 108.0) * t22;
        let t232 = t231 * t26;
        let t239 = t116 * t74;
        let t242 = t232 * t38 / 24.0 - t130 * t103 / 9.0 + t99 * t132 / 24.0 - t48 * t239 / 9.0;
        let t243 = t71 * t242;
        let t248 = piecewise3::<f64>(t2, 0.0, -t67 * t149 * t136 / 8.0 + 3.0 / 4.0 * t210 * t213 - 3.0 / 8.0 * t67 * t69 * t243);
        let tv2rhosigma0 = 2.0 * rho[ip] * t248 + 2.0 * t140;
        v2rhosigma[ip] += tv2rhosigma0;
        let t251 = t135 * t135;
        let t252 = t154 * t251;
        let t256 = t80 * t81;
        let t257 = t83 * t29;
        let t261 = 1.0 / t179;
        let t267 = (-t256 * t257 * t124 / 144.0 + t177 * sigma[ip] * t261 * t184 / 288.0) * t22;
        let t268 = t267 * t26;
        let t273 = t268 * t38 / 24.0 + t129 * t132 / 12.0;
        let t274 = t71 * t273;
        let t279 = piecewise3::<f64>(t2, 0.0, 3.0 / 4.0 * t67 * t69 * t252 - 3.0 / 8.0 * t67 * t69 * t274);
        let tv2sigma20 = 2.0 * rho[ip] * t279;
        v2sigma2[ip] += tv2sigma20;
        let t286 = t144 * t68;
        let t296 = t70 * t70;
        let t297 = 1.0 / t296;
        let t298 = t155 * t106;
        let t299 = t297 * t298;
        let t304 = t211 * t106 * t198;
        let t308 = 1.0 / t33 / t121;
        let t309 = t308 * t42;
        let t314 = 1.0 / t18 / t179;
        let t319 = t179 * t72;
        let t320 = 1.0 / t319;
        let t325 = t79 * t79;
        let t326 = t20 * t325;
        let t327 = t86 * t86;
        let t328 = t176 * t327;
        let t329 = t326 * t328;
        let t330 = t179 * t121;
        let t332 = 1.0 / t33 / t330;
        let t333 = t92 * t92;
        let t334 = 1.0 / t333;
        let t336 = t27 * t30;
        let t341 = (-154.0 / 81.0 * t28 * t31 * t309 + 341.0 / 486.0 * t85 * t87 * t314 * t93 - 38.0 / 81.0 * t177 * t178 * t320 * t184 + 2.0 / 243.0 * t329 * t332 * t334 * t336) * t22;
        let t342 = t341 * t26;
        let t349 = t31 * t308;
        let t352 = t342 * t38 / 24.0 - t190 * t103 / 3.0 + 11.0 / 9.0 * t100 * t195 - 154.0 / 81.0 * t49 * t349;
        let t353 = t71 * t352;
        let t358 = piecewise3::<f64>(t2, 0.0, -5.0 / 36.0 * t6 * t17 * t35 * t57 + t67 * t286 * t107 / 4.0 + 3.0 / 4.0 * t67 * t149 * t156 - 3.0 / 8.0 * t67 * t149 * t199 - 9.0 / 4.0 * t67 * t69 * t299 + 9.0 / 4.0 * t210 * t304 - 3.0 / 8.0 * t67 * t69 * t353);
        let tv3rho30 = 2.0 * rho[ip] * t358 + 6.0 * t204;
        v3rho3[ip] += tv3rho30;
        let t365 = t6 * t63;
        let t371 = t68 * t297;
        let t373 = t371 * t135 * t155;
        let t377 = t211 * t242 * t106;
        let t381 = t211 * t135 * t198;
        let t391 = t181 * t184;
        let t395 = t176 * t178;
        let t397 = t179 * t88;
        let t399 = 1.0 / t33 / t397;
        let t405 = (11.0 / 27.0 * t115 * t116 * t162 - 65.0 / 324.0 * t85 * t29 * t168 * t220 + 17.0 / 108.0 * t177 * t391 * t86 - t326 * t395 * t399 * t334 * t336 / 324.0) * t22;
        let t406 = t405 * t26;
        let t417 = t116 * t161;
        let t420 = t406 * t38 / 24.0 - 2.0 / 9.0 * t232 * t103 + 11.0 / 27.0 * t130 * t195 + t189 * t132 / 24.0 - 2.0 / 9.0 * t99 * t239 + 11.0 / 27.0 * t48 * t417;
        let t421 = t71 * t420;
        let t426 = piecewise3::<f64>(t2, 0.0, t67 * t286 * t136 / 12.0 + t365 * t213 / 2.0 - t67 * t149 * t243 / 4.0 - 9.0 / 4.0 * t210 * t373 + 3.0 / 2.0 * t210 * t377 + 3.0 / 4.0 * t210 * t381 - 3.0 / 8.0 * t67 * t69 * t421);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t426 + 4.0 * t248;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t433 = t371 * t251 * t106;
        let t437 = t211 * t135 * t242;
        let t444 = t211 * t273 * t106;
        let t454 = t176 * t86;
        let t457 = 1.0 / t33 / t319;
        let t463 = (t256 * t257 * t94 / 27.0 - 5.0 / 108.0 * t177 * t225 * t184 * sigma[ip] + t326 * t454 * t457 * t334 * t336 / 864.0) * t22;
        let t464 = t463 * t26;
        let t473 = t464 * t38 / 24.0 - t268 * t103 / 9.0 + t231 * t132 / 12.0 - 2.0 / 9.0 * t129 * t239;
        let t474 = t71 * t473;
        let t479 = piecewise3::<f64>(t2, 0.0, t67 * t149 * t252 / 4.0 - 9.0 / 4.0 * t210 * t433 + 3.0 / 2.0 * t210 * t437 - t67 * t149 * t274 / 8.0 + 3.0 / 4.0 * t210 * t444 - 3.0 / 8.0 * t67 * t69 * t474);
        let tv3rhosigma20 = 2.0 * rho[ip] * t479 + 2.0 * t279;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t482 = t251 * t135;
        let t483 = t297 * t482;
        let t487 = t135 * t273;
        let t488 = t211 * t487;
        let t495 = t176 * sigma[ip];
        let t498 = 1.0 / t33 / t180;
        let t504 = (t174 * t176 * t261 * t184 / 96.0 - t326 * t495 * t498 * t334 * t336 / 2304.0) * t22;
        let t505 = t504 * t26;
        let t510 = t505 * t38 / 24.0 + t267 * t132 / 8.0;
        let t511 = t71 * t510;
        let t516 = piecewise3::<f64>(t2, 0.0, -9.0 / 4.0 * t67 * t69 * t483 + 9.0 / 4.0 * t210 * t488 - 3.0 / 8.0 * t67 * t69 * t511);
        let tv3sigma30 = 2.0 * rho[ip] * t516;
        v3sigma3[ip] += tv3sigma30;
    }
}
