//! GGA_X_OL2 lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 29 shared lines across all orders.
//! Delta: 15 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_ol2_lxc_unpol(
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
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (29 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = param_bb * sigma[ip];
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t27 = t22 * t26;
        let t30 = f64::sqrt(sigma[ip]);
        let t31 = param_cc * t30;
        let t33 = 1.0 / t18 / rho[ip];
        let t38 = 4.0 * t30 * t21 * t33 + t21;
        let t39 = 1.0 / t38;
        let t40 = t21 * t33 * t39;
        let t42 = param_aa + 0.13888888888888888889e-1 * t20 * t27 + t31 * t40;
        let t46 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t42);
        let tzk0 = 2.0 * t46;
        zk[ip] += tzk0;
        // --- vxc delta (19 lines) ---
        let t48 = t17 / t24;
        let t52 = t23 * rho[ip];
        let t54 = 1.0 / t24 / t52;
        let t55 = t22 * t54;
        let t61 = t21 / t18 / t23 * t39;
        let t64 = param_cc * sigma[ip];
        let t65 = t38 * t38;
        let t66 = 1.0 / t65;
        let t67 = t55 * t66;
        let t70 = -0.37037037037037037037e-1 * t20 * t55 - 4.0 / 3.0 * t31 * t61 + 16.0 / 3.0 * t64 * t67;
        let t75 = piecewise3(t2, 0.0, -t6 * t48 * t42 / 8.0 - 3.0 / 8.0 * t6 * t19 * t70);
        let tvrho0 = 2.0 * rho[ip] * t75 + 2.0 * t46;
        vrho[ip] += tvrho0;
        let t78 = param_bb * t22;
        let t81 = 1.0 / t30;
        let t82 = param_cc * t81;
        let t85 = param_cc * t22;
        let t89 = 0.13888888888888888889e-1 * t78 * t26 + t82 * t40 / 2.0 - 2.0 * t85 * t26 * t66;
        let t93 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t89);
        let tvsigma0 = 2.0 * rho[ip] * t93;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (30 lines) ---
        let t98 = t17 / t24 / rho[ip];
        let t105 = t23 * t23;
        let t107 = 1.0 / t24 / t105;
        let t108 = t22 * t107;
        let t114 = t21 / t18 / t52 * t39;
        let t117 = t108 * t66;
        let t120 = t30 * sigma[ip];
        let t121 = param_cc * t120;
        let t122 = t105 * t23;
        let t123 = 1.0 / t122;
        let t125 = 1.0 / t65 / t38;
        let t126 = t123 * t125;
        let t129 = 0.13580246913580246914e0 * t20 * t108 + 28.0 / 9.0 * t31 * t114 - 80.0 / 3.0 * t64 * t117 + 1024.0 / 9.0 * t121 * t126;
        let t134 = piecewise3(t2, 0.0, t6 * t98 * t42 / 12.0 - t6 * t48 * t70 / 4.0 - 3.0 / 8.0 * t6 * t19 * t129);
        let tv2rho20 = 2.0 * rho[ip] * t134 + 4.0 * t75;
        v2rho2[ip] += tv2rho20;
        let t147 = t105 * rho[ip];
        let t148 = 1.0 / t147;
        let t150 = t125 * t30;
        let t153 = -0.37037037037037037037e-1 * t78 * t54 - 2.0 / 3.0 * t82 * t61 + 8.0 * t85 * t54 * t66 - 128.0 / 3.0 * param_cc * t148 * t150;
        let t158 = piecewise3(t2, 0.0, -t6 * t48 * t89 / 8.0 - 3.0 / 8.0 * t6 * t19 * t153);
        let tv2rhosigma0 = 2.0 * rho[ip] * t158 + 2.0 * t93;
        v2rhosigma[ip] += tv2rhosigma0;
        let t161 = 1.0 / t120;
        let t162 = param_cc * t161;
        let t165 = 1.0 / sigma[ip];
        let t166 = param_cc * t165;
        let t167 = t27 * t66;
        let t169 = 1.0 / t105;
        let t174 = -t162 * t40 / 4.0 - t166 * t167 + 16.0 * param_cc * t169 * t125 * t81;
        let t178 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t174);
        let tv2sigma20 = 2.0 * rho[ip] * t178;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (36 lines) ---
        let t181 = t17 * t26;
        let t192 = 1.0 / t24 / t147;
        let t193 = t22 * t192;
        let t199 = t21 / t18 / t105 * t39;
        let t205 = t105 * t52;
        let t206 = 1.0 / t205;
        let t210 = sigma[ip] * sigma[ip];
        let t211 = param_cc * t210;
        let t212 = t105 * t105;
        let t214 = 1.0 / t18 / t212;
        let t215 = t65 * t65;
        let t216 = 1.0 / t215;
        let t221 = -0.63374485596707818932e0 * t20 * t193 - 280.0 / 27.0 * t31 * t199 + 3808.0 / 27.0 * t64 * t193 * t66 - 11264.0 / 9.0 * t121 * t206 * t125 + 16384.0 / 9.0 * t211 * t214 * t216 * t21;
        let t226 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t181 * t42 + t6 * t98 * t70 / 4.0 - 3.0 / 8.0 * t6 * t48 * t129 - 3.0 / 8.0 * t6 * t19 * t221);
        let tv3rho30 = 2.0 * rho[ip] * t226 + 6.0 * t134;
        v3rho3[ip] += tv3rho30;
        let t248 = param_cc / t18 / t205;
        let t250 = t216 * sigma[ip] * t21;
        let t253 = 0.13580246913580246914e0 * t78 * t107 + 14.0 / 9.0 * t82 * t114 - 296.0 / 9.0 * t85 * t107 * t66 + 384.0 * param_cc * t123 * t150 - 2048.0 / 3.0 * t248 * t250;
        let t258 = piecewise3(t2, 0.0, t6 * t98 * t89 / 12.0 - t6 * t48 * t153 / 4.0 - 3.0 / 8.0 * t6 * t19 * t253);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t258 + 4.0 * t158;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t268 = t148 * t125;
        let t273 = param_cc / t18 / t122;
        let t274 = t216 * t21;
        let t277 = t162 * t61 / 3.0 + 4.0 / 3.0 * t166 * t67 - 256.0 / 3.0 * t82 * t268 + 256.0 * t273 * t274;
        let t282 = piecewise3(t2, 0.0, -t6 * t48 * t174 / 8.0 - 3.0 / 8.0 * t6 * t19 * t277);
        let tv3rhosigma20 = 2.0 * rho[ip] * t282 + 2.0 * t178;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t285 = t30 * t210;
        let t287 = param_cc / t285;
        let t290 = 1.0 / t210;
        let t291 = param_cc * t290;
        let t295 = 1.0 / t18 / t147;
        let t296 = param_cc * t295;
        let t298 = t216 * t165 * t21;
        let t301 = 3.0 / 8.0 * t287 * t40 + 3.0 / 2.0 * t291 * t167 - 96.0 * t296 * t298;
        let t305 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t301);
        let tv3sigma30 = 2.0 * rho[ip] * t305;
        v3sigma3[ip] += tv3sigma30;
        // --- lxc delta (this level) (15 lines) ---
        let t322 = 1.0 / t24 / t122;
        let t323 = t22 * t322;
        let t337 = t212 * rho[ip];
        let t349 = 1.0 / t215 / t38;
        let t359 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t54 * t42 - 5.0 / 9.0 * t6 * t181 * t70 + t6 * t98 * t129 / 2.0 - t6 * t48 * t221 / 2.0 - 3.0 / 8.0 * t6 * t19 * (0.35912208504801097395e1 * t20 * t323 + 3640.0 / 81.0 * t31 * t21 * t295 * t39 - 23072.0 / 27.0 * t64 * t323 * t66 + 953344.0 / 81.0 * t121 / t212 * t125 - 950272.0 / 27.0 * t211 / t18 / t337 * t216 * t21 + 1048576.0 / 27.0 * param_cc * t285 / t24 / t212 / t23 * t349 * t22));
        let tv4rho40 = 2.0 * rho[ip] * t359 + 8.0 * t226;
        v4rho4[ip] += tv4rho40;
        let t397 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t181 * t89 + t6 * t98 * t153 / 4.0 - 3.0 / 8.0 * t6 * t48 * t253 - 3.0 / 8.0 * t6 * t19 * (-0.63374485596707818932e0 * t78 * t192 - 140.0 / 27.0 * t82 * t199 + 1456.0 / 9.0 * t85 * t192 * t66 - 81152.0 / 27.0 * param_cc * t206 * t150 + 100352.0 / 9.0 * param_cc * t214 * t250 - 131072.0 / 9.0 * param_cc / t24 / t337 * t349 * t120 * t22));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t397 + 6.0 * t258;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t427 = piecewise3(t2, 0.0, t6 * t98 * t174 / 12.0 - t6 * t48 * t277 / 4.0 - 3.0 / 8.0 * t6 * t19 * (-7.0 / 9.0 * t162 * t114 - 28.0 / 9.0 * t166 * t117 + 4096.0 / 9.0 * t82 * t126 - 8960.0 / 3.0 * t248 * t274 + 16384.0 / 3.0 * param_cc / t24 / t212 * t349 * t22 * t30));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t427 + 4.0 * t282;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t453 = piecewise3(t2, 0.0, -t6 * t48 * t301 / 8.0 - 3.0 / 8.0 * t6 * t19 * (-t287 * t61 / 2.0 - 2.0 * t291 * t67 + 32.0 * t162 * t268 + 512.0 * t273 * t298 - 2048.0 * param_cc / t24 / t205 * t349 * t81 * t22));
        let tv4rhosigma30 = 2.0 * rho[ip] * t453 + 2.0 * t305;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t456 = t210 * sigma[ip];
        let t482 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * (-15.0 / 16.0 * param_cc / t30 / t456 * t40 - 15.0 / 4.0 * param_cc / t456 * t167 - 12.0 * t287 * t169 * t125 + 768.0 * param_cc * t322 * t349 * t161 * t22 + 96.0 * t296 * t216 * t290 * t21));
        let tv4sigma40 = 2.0 * rho[ip] * t482;
        v4sigma4[ip] += tv4sigma40;
    }
}
