//! MGGA_X_2D_PRP10 kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_vxc/mgga_x_2d_prp10.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::bessel::{xc_bessel_I0, xc_bessel_I1};
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_2d_prp10_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let t2 = rho[ip] * rho[ip];
        let t3 = 1.0 / t2;
        let t7 = 2.0 * tau[ip] * t3;
        let t9 = 1.0 / t2 / rho[ip];
        let t11 = sigma[ip] * t9 / 4.0;
        let t13 = 1.0 / M_PI;
        let t14 = (lapl[ip] * t3 / 2.0 - t7 + t11) * t13;
        let t15 = -0.9999999999e0 < t14;
        let t16 = piecewise3::<f64>(t15, t14, -0.9999999999e0);
        let t17 = f64::exp(-1.0);
        let t19 = lambert_w::<f64>(t16 * t17);
        let t20 = t19 + 1.0;
        let t21 = t20 / 2.0;
        let t22 = xc_bessel_I0::<f64>(t21);
        let t24 = t7 - t11;
        let t25 = 0.1e-9 < t24;
        let t26 = piecewise3::<f64>(t25, t24, 0.1e-9);
        let t27 = f64::sqrt(t26);
        let t31 = M_SQRT2;
        let t32 = (M_PI * t22 - 4.0 / 3.0 * t13 * t27) * t31;
        let t33 = f64::sqrt(rho[ip]);
        let tvrho0 = -t32 * t33 / 2.0;
        vrho[ip] += tvrho0;
        let t36 = xc_bessel_I1::<f64>(t21);
        let t37 = M_PI * t36;
        let t40 = 4.0 * tau[ip] * t9;
        let t41 = t2 * t2;
        let t42 = 1.0 / t41;
        let t44 = 3.0 / 4.0 * sigma[ip] * t42;
        let t47 = piecewise3::<f64>(t15, (-lapl[ip] * t9 + t40 - t44) * t13, 0.0);
        let t49 = 1.0 / t20;
        let t50 = t19 * t49;
        let t51 = 1.0 / t16;
        let t52 = t50 * t51;
        let t56 = t13 / t27;
        let t58 = piecewise3::<f64>(t25, -t40 + t44, 0.0);
        let t62 = (t37 * t47 * t52 / 2.0 - 2.0 / 3.0 * t56 * t58) * t31;
        let t65 = 1.0 / t33;
        let tv2rho20 = -t62 * t33 / 2.0 - t32 * t65 / 4.0;
        v2rho2[ip] += tv2rho20;
        let t68 = t9 * t13;
        let t70 = piecewise3::<f64>(t15, t68 / 4.0, 0.0);
        let t71 = t37 * t70;
        let t75 = piecewise3::<f64>(t25, -t9 / 4.0, 0.0);
        let t79 = (t71 * t52 / 2.0 - 2.0 / 3.0 * t56 * t75) * t31;
        let tv2rhosigma0 = -t79 * t33 / 2.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t82 = t3 * t13;
        let t84 = piecewise3::<f64>(t15, t82 / 2.0, 0.0);
        let t85 = t84 * t19;
        let t86 = t37 * t85;
        let t87 = t49 * t51;
        let t88 = t31 * t33;
        let t89 = t87 * t88;
        let tv2rholapl0 = -t86 * t89 / 4.0;
        v2rholapl[ip] += tv2rholapl0;
        let t93 = piecewise3::<f64>(t15, -2.0 * t82, 0.0);
        let t94 = t37 * t93;
        let t98 = piecewise3::<f64>(t25, 2.0 * t3, 0.0);
        let t102 = (t94 * t52 / 2.0 - 2.0 / 3.0 * t56 * t98) * t31;
        let tv2rhotau0 = -t102 * t33 / 2.0;
        v2rhotau[ip] += tv2rhotau0;
        let t105 = 1.0 / t21;
        let t107 = -t105 * t36 + t22;
        let t108 = M_PI * t107;
        let t109 = t47 * t47;
        let t110 = t108 * t109;
        let t111 = t19 * t19;
        let t112 = t20 * t20;
        let t113 = 1.0 / t112;
        let t114 = t111 * t113;
        let t115 = t16 * t16;
        let t116 = 1.0 / t115;
        let t117 = t114 * t116;
        let t123 = 12.0 * tau[ip] * t42;
        let t125 = 1.0 / t41 / rho[ip];
        let t127 = 3.0 * sigma[ip] * t125;
        let t130 = piecewise3::<f64>(t15, (3.0 * lapl[ip] * t42 - t123 + t127) * t13, 0.0);
        let t131 = t37 * t130;
        let t134 = t37 * t109;
        let t135 = t19 * t113;
        let t136 = t135 * t116;
        let t140 = 1.0 / t112 / t20;
        let t141 = t111 * t140;
        let t142 = t141 * t116;
        let t145 = t50 * t116;
        let t150 = t13 / t27 / t26;
        let t151 = t58 * t58;
        let t155 = piecewise3::<f64>(t25, t123 - t127, 0.0);
        let t159 = (t110 * t117 / 4.0 + t131 * t52 / 2.0 + t134 * t136 / 2.0 - t134 * t142 / 2.0 - t134 * t145 / 2.0 + t150 * t151 / 3.0 - 2.0 / 3.0 * t56 * t155) * t31;
        let t165 = 1.0 / t33 / rho[ip];
        let tv3rho30 = -t159 * t33 / 2.0 - t62 * t65 / 2.0 + t32 * t165 / 8.0;
        v3rho3[ip] += tv3rho30;
        let t168 = t108 * t47;
        let t169 = t116 * t70;
        let t170 = t114 * t169;
        let t173 = t42 * t13;
        let t175 = piecewise3::<f64>(t15, -3.0 / 4.0 * t173, 0.0);
        let t176 = t37 * t175;
        let t179 = t47 * t19;
        let t180 = t113 * t116;
        let t181 = t179 * t180;
        let t184 = t116 * t47;
        let t185 = t141 * t184;
        let t188 = t50 * t184;
        let t195 = piecewise3::<f64>(t25, 3.0 / 4.0 * t42, 0.0);
        let t199 = (t168 * t170 / 4.0 + t176 * t52 / 2.0 + t71 * t181 / 2.0 - t71 * t185 / 2.0 - t71 * t188 / 2.0 + t150 * t75 * t58 / 3.0 - 2.0 / 3.0 * t56 * t195) * t31;
        let tv3rho2sigma0 = -t199 * t33 / 2.0 - t79 * t65 / 4.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t204 = t47 * t111;
        let t205 = t108 * t204;
        let t206 = t84 * t31;
        let t207 = t206 * t33;
        let t208 = t180 * t207;
        let t211 = piecewise3::<f64>(t15, -t68, 0.0);
        let t212 = t211 * t19;
        let t213 = t37 * t212;
        let t216 = t84 * t47;
        let t217 = t37 * t216;
        let t218 = t116 * t31;
        let t220 = t135 * t218 * t33;
        let t223 = t84 * t111;
        let t224 = t37 * t223;
        let t225 = t140 * t116;
        let t226 = t88 * t47;
        let t227 = t225 * t226;
        let t230 = t49 * t116;
        let t231 = t230 * t226;
        let t234 = t31 * t65;
        let t235 = t87 * t234;
        let tv3rho2lapl0 = -t205 * t208 / 8.0 - t213 * t89 / 4.0 - t217 * t220 / 4.0 + t224 * t227 / 4.0 + t86 * t231 / 4.0 - t86 * t235 / 8.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t239 = t114 * t116 * t93;
        let t243 = piecewise3::<f64>(t15, 4.0 * t68, 0.0);
        let t244 = t37 * t243;
        let t257 = piecewise3::<f64>(t25, -4.0 * t9, 0.0);
        let t261 = (t168 * t239 / 4.0 + t244 * t52 / 2.0 + t94 * t181 / 2.0 - t94 * t185 / 2.0 - t94 * t188 / 2.0 + t150 * t98 * t58 / 3.0 - 2.0 / 3.0 * t56 * t257) * t31;
        let tv3rho2tau0 = -t261 * t33 / 2.0 - t102 * t65 / 4.0;
        v3rho2tau[ip] += tv3rho2tau0;
        let t266 = t70 * t70;
        let t267 = t108 * t266;
        let t270 = piecewise3::<f64>(t15, 0.0, 0.0);
        let t271 = t37 * t270;
        let t273 = t271 * t52 / 2.0;
        let t274 = t37 * t266;
        let t281 = t75 * t75;
        let t284 = piecewise3::<f64>(t25, 0.0, 0.0);
        let t286 = 2.0 / 3.0 * t56 * t284;
        let t288 = (t267 * t117 / 4.0 + t273 + t274 * t136 / 2.0 - t274 * t142 / 2.0 - t274 * t145 / 2.0 + t150 * t281 / 3.0 - t286) * t31;
        let tv3rhosigma20 = -t288 * t33 / 2.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t291 = t70 * t111;
        let t292 = t108 * t291;
        let t295 = t270 * t19;
        let t296 = t37 * t295;
        let t298 = t296 * t89 / 4.0;
        let t299 = t84 * t70;
        let t300 = t37 * t299;
        let t303 = t88 * t70;
        let t304 = t225 * t303;
        let t307 = t230 * t303;
        let tv3rhosigmalapl0 = -t292 * t208 / 8.0 - t298 - t300 * t220 / 4.0 + t224 * t304 / 4.0 + t86 * t307 / 4.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t310 = t108 * t70;
        let t313 = t70 * t19;
        let t314 = t313 * t180;
        let t317 = t141 * t169;
        let t320 = t50 * t169;
        let t323 = t98 * t75;
        let t327 = (t310 * t239 / 4.0 + t273 + t94 * t314 / 2.0 - t94 * t317 / 2.0 - t94 * t320 / 2.0 + t150 * t323 / 3.0 - t286) * t31;
        let tv3rhosigmatau0 = -t327 * t33 / 2.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t330 = t84 * t84;
        let t331 = t330 * t111;
        let t332 = t108 * t331;
        let t333 = t180 * t88;
        let t336 = t330 * t19;
        let t337 = t37 * t336;
        let t340 = t37 * t331;
        let tv3rholapl20 = -t332 * t333 / 8.0 - t298 - t337 * t333 / 4.0 + t340 * t225 * t88 / 4.0 + t337 * t230 * t88 / 4.0;
        v3rholapl2[ip] += tv3rholapl20;
        let t347 = t108 * t84;
        let t350 = t85 * t180;
        let t353 = t116 * t84;
        let t354 = t141 * t353;
        let t357 = t50 * t353;
        let t361 = (t347 * t239 / 4.0 + t273 + t94 * t350 / 2.0 - t94 * t354 / 2.0 - t94 * t357 / 2.0) * t31;
        let tv3rholapltau0 = -t361 * t33 / 2.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t364 = t93 * t93;
        let t365 = t108 * t364;
        let t368 = t37 * t364;
        let t375 = t98 * t98;
        let t379 = (t365 * t117 / 4.0 + t273 + t368 * t136 / 2.0 - t368 * t142 / 2.0 - t368 * t145 / 2.0 + t150 * t375 / 3.0 - t286) * t31;
        let tv3rhotau20 = -t379 * t33 / 2.0;
        v3rhotau2[ip] += tv3rhotau20;
    }
}
