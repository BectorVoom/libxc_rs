//! GGA_K_LC94 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 49 shared lines across all orders.
//! Delta: 65 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_lc94_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (49 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t22 / t34;
        let t37 = t33 * t36;
        let t40 = f64::exp(-param_alpha * t24 * t29 * t37 / 24.0);
        let t43 = (t40 * param_d + param_c) * t24;
        let t44 = t43 * t29;
        let t47 = t24 * t24;
        let t48 = 1.0 / t27;
        let t49 = t47 * t48;
        let t50 = f64::sqrt(sigma[ip]);
        let t53 = 1.0 / t21 / rho[ip];
        let t54 = t50 * t31 * t53;
        let t57 = f64::powf(t49 * t54 / 12.0, param_expo);
        let t58 = param_f * t57;
        let t59 = t44 * t37 / 24.0 - t58;
        let t60 = t49 * t50;
        let t66 = f64::ln(param_b * t47 * t48 * t54 / 12.0 + f64::sqrt(pow_2(param_b * t47 * t48 * t54 / 12.0) + 1.0));
        let t67 = param_a * t66;
        let t68 = t31 * t53 * t67;
        let t71 = 1.0 + t60 * t68 / 12.0 + t58;
        let t72 = 1.0 / t71;
        let t74 = t59 * t72 + 1.0;
        let t78 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        // --- vxc delta (47 lines) ---
        let t80 = t20 / t21;
        let t84 = param_d * param_alpha;
        let t86 = 1.0 / t27 / t26;
        let t87 = t47 * t86;
        let t88 = t84 * t87;
        let t89 = sigma[ip] * sigma[ip];
        let t90 = t89 * t31;
        let t91 = t34 * t34;
        let t92 = t91 * t34;
        let t94 = 1.0 / t21 / t92;
        let t95 = t94 * t40;
        let t99 = t34 * rho[ip];
        let t101 = 1.0 / t22 / t99;
        let t105 = 1.0 / rho[ip];
        let t108 = 4.0 / 3.0 * t58 * param_expo * t105;
        let t109 = t88 * t90 * t95 / 108.0 - t44 * t33 * t101 / 9.0 + t108;
        let t111 = t71 * t71;
        let t112 = 1.0 / t111;
        let t113 = t59 * t112;
        let t115 = 1.0 / t21 / t34;
        let t117 = t31 * t115 * t67;
        let t120 = t24 * t29;
        let t121 = t120 * t33;
        let t123 = param_b * param_b;
        let t128 = 6.0 * t123 * t24 * t29 * t37 + 144.0;
        let t129 = f64::sqrt(t128);
        let t131 = param_b / t129;
        let t132 = t101 * param_a * t131;
        let t135 = -t60 * t117 / 9.0 - 2.0 / 3.0 * t121 * t132 - t108;
        let t137 = t109 * t72 - t113 * t135;
        let t142 = piecewise3(t2, 0.0, t7 * t80 * t74 / 10.0 + 3.0 / 20.0 * t7 * t23 * t137);
        let tvrho0 = 2.0 * rho[ip] * t142 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t145 = t91 * rho[ip];
        let t147 = 1.0 / t21 / t145;
        let t148 = t31 * t147;
        let t149 = t40 * sigma[ip];
        let t153 = t29 * t32;
        let t157 = 1.0 / sigma[ip];
        let t160 = t58 * param_expo * t157 / 2.0;
        let t161 = -t88 * t148 * t149 / 288.0 + t43 * t153 * t36 / 24.0 - t160;
        let t164 = t49 / t50;
        let t167 = t120 * t32;
        let t169 = t36 * param_a * t131;
        let t172 = t164 * t68 / 24.0 + t167 * t169 / 4.0 + t160;
        let t174 = -t113 * t172 + t161 * t72;
        let t178 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t174);
        let tvsigma0 = 2.0 * rho[ip] * t178;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (65 lines) ---
        let t181 = t20 * t53;
        let t188 = t91 * t99;
        let t190 = 1.0 / t21 / t188;
        let t191 = t190 * t40;
        let t195 = param_alpha * param_alpha;
        let t196 = param_d * t195;
        let t197 = t26 * t26;
        let t198 = 1.0 / t197;
        let t199 = t196 * t198;
        let t200 = t89 * sigma[ip];
        let t201 = t91 * t91;
        let t202 = t201 * t34;
        let t203 = 1.0 / t202;
        let t209 = 1.0 / t22 / t91;
        let t213 = param_expo * param_expo;
        let t214 = 1.0 / t34;
        let t215 = t213 * t214;
        let t217 = 16.0 / 9.0 * t58 * t215;
        let t220 = 4.0 / 3.0 * t58 * param_expo * t214;
        let t221 = -t88 * t90 * t191 / 12.0 + t199 * t200 * t203 * t40 / 81.0 + 11.0 / 27.0 * t44 * t33 * t209 - t217 - t220;
        let t223 = t109 * t112;
        let t227 = 1.0 / t111 / t71;
        let t228 = t59 * t227;
        let t229 = t135 * t135;
        let t233 = 1.0 / t21 / t99;
        let t235 = t31 * t233 * t67;
        let t239 = t209 * param_a * t131;
        let t242 = t87 * t90;
        let t244 = t123 * param_b;
        let t246 = 1.0 / t129 / t128;
        let t247 = t244 * t246;
        let t248 = t190 * param_a * t247;
        let t251 = 7.0 / 27.0 * t60 * t235 + 10.0 / 3.0 * t121 * t239 - 32.0 / 3.0 * t242 * t248 + t217 + t220;
        let t253 = -t113 * t251 - 2.0 * t135 * t223 + t221 * t72 + 2.0 * t228 * t229;
        let t258 = piecewise3(t2, 0.0, -t7 * t181 * t74 / 30.0 + t7 * t80 * t137 / 5.0 + 3.0 / 20.0 * t7 * t23 * t253);
        let tv2rho20 = 2.0 * rho[ip] * t258 + 4.0 * t142;
        v2rho2[ip] += tv2rho20;
        let t264 = t31 * t94;
        let t268 = t201 * rho[ip];
        let t269 = 1.0 / t268;
        let t277 = t213 * t105;
        let t280 = 2.0 / 3.0 * t58 * t277 * t157;
        let t281 = t88 * t264 * t149 / 36.0 - t199 * t269 * t89 * t40 / 216.0 - t43 * t153 * t101 / 9.0 + t280;
        let t283 = t161 * t112;
        let t286 = t172 * t135;
        let t295 = param_a * t244 * t246 * sigma[ip];
        let t298 = -t164 * t117 / 18.0 - t167 * t132 + 4.0 * t87 * t264 * t295 - t280;
        let t300 = -t113 * t298 - t135 * t283 - t172 * t223 + 2.0 * t228 * t286 + t281 * t72;
        let t305 = piecewise3(t2, 0.0, t7 * t80 * t174 / 10.0 + 3.0 / 20.0 * t7 * t23 * t300);
        let tv2rhosigma0 = 2.0 * rho[ip] * t305 + 2.0 * t178;
        v2rhosigma[ip] += tv2rhosigma0;
        let t308 = 1.0 / t201;
        let t313 = t84 * t47;
        let t314 = t86 * t31;
        let t319 = 1.0 / t89;
        let t322 = t58 * t213 * t319 / 4.0;
        let t325 = t58 * param_expo * t319 / 2.0;
        let t326 = t199 * t308 * t40 * sigma[ip] / 576.0 - t313 * t314 * t147 * t40 / 144.0 - t322 + t325;
        let t330 = t172 * t172;
        let t335 = t49 / t50 / sigma[ip];
        let t339 = t120 * t157 * t32;
        let t342 = t87 * t31;
        let t344 = t147 * param_a * t247;
        let t347 = -t335 * t68 / 48.0 + t339 * t169 / 8.0 - 3.0 / 2.0 * t342 * t344 + t322 - t325;
        let t349 = -t113 * t347 - 2.0 * t172 * t283 + 2.0 * t228 * t330 + t326 * t72;
        let t353 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t349);
        let tv2sigma20 = 2.0 * rho[ip] * t353;
        v2sigma2[ip] += tv2sigma20;
    }
}
