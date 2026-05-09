//! GGA_C_WI fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 28 shared lines across all orders.
//! Delta: 98 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_wi_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_k: f64,
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
        // --- shared preamble (28 lines) ---
        let t2 = sigma0 + 2.0 * sigma1 + sigma2;
        let t3 = param_b * t2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = pow_1_3(t4);
        let t7 = t6 * t6;
        let t9 = 1.0 / t7 / t5;
        let t10 = param_k * t2;
        let t12 = f64::exp(-t10 * t9);
        let t15 = t3 * t9 * t12 + param_a;
        let t16 = M_CBRT3;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = t16 * t18;
        let t20 = M_CBRT4;
        let t21 = t20 * t20;
        let t25 = t16 * t16;
        let t26 = M_CBRTPI;
        let t28 = f64::sqrt(t2);
        let t29 = t28 * t2;
        let t30 = t5 * t5;
        let t31 = 1.0 / t30;
        let t34 = 1.0 / t6 / t4;
        let t35 = t28 * t34;
        let t36 = f64::sqrt(t35);
        let t41 = 1.0 + param_d * t20 * t25 * t26 * t36 * t29 * t31 / 3.0;
        let t45 = param_c + t19 * t21 / t6 * t41 / 4.0;
        let t46 = 1.0 / t45;
        let tzk0 = t15 * t46;
        zk[ip] += tzk0;
        // --- vxc delta (36 lines) ---
        let t47 = t5 * t4;
        let t49 = 1.0 / t7 / t47;
        let t52 = t2 * t2;
        let t53 = param_b * t52;
        let t54 = t30 * t5;
        let t56 = 1.0 / t6 / t54;
        let t61 = 8.0 / 3.0 * t53 * t56 * param_k * t12 - 8.0 / 3.0 * t3 * t49 * t12;
        let t62 = t4 * t61;
        let t64 = t4 * t15;
        let t65 = t45 * t45;
        let t66 = 1.0 / t65;
        let t74 = t36 * t2 * t9;
        let t75 = t26 * t74;
        let t76 = t75 * t28;
        let t79 = -t19 * t21 * t34 * t41 / 12.0 - 14.0 / 3.0 * t18 * t9 * param_d * t76;
        let t80 = t66 * t79;
        let tvrho0 = t62 * t46 - t64 * t80 + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t84 = t30 * t4;
        let t86 = 1.0 / t6 / t84;
        let t90 = -t3 * t86 * param_k * t12 + param_b * t9 * t12;
        let t91 = t4 * t90;
        let t93 = 1.0 / t7;
        let t94 = t93 * t15;
        let t95 = t66 * t18;
        let t96 = t94 * t95;
        let t97 = param_d * t26;
        let t98 = 1.0 / t28;
        let t99 = t74 * t98;
        let t100 = t97 * t99;
        let t101 = t96 * t100;
        let tvsigma0 = t91 * t46 - 7.0 / 4.0 * t101;
        vsigma[ip * 3] += tvsigma0;
        let t103 = 2.0 * t90;
        let t104 = t4 * t103;
        let tvsigma1 = t104 * t46 - 7.0 / 2.0 * t101;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        // --- fxc delta (this level) (98 lines) ---
        let t109 = t15 * t66;
        let t113 = 1.0 / t7 / t30;
        let t117 = t30 * t47;
        let t119 = 1.0 / t6 / t117;
        let t124 = t52 * t2;
        let t125 = param_b * t124;
        let t126 = t30 * t30;
        let t127 = t126 * t5;
        let t128 = 1.0 / t127;
        let t129 = param_k * param_k;
        let t134 = 88.0 / 9.0 * t3 * t113 * t12 - 24.0 * t53 * t119 * param_k * t12 + 64.0 / 9.0 * t125 * t128 * t129 * t12;
        let t135 = t4 * t134;
        let t140 = 1.0 / t65 / t45;
        let t141 = t79 * t79;
        let t142 = t140 * t141;
        let t155 = 1.0 / t84;
        let t158 = t36 * t35;
        let t159 = t26 * t158;
        let t160 = t159 * t2;
        let t163 = t19 * t21 / t6 / t5 * t41 / 9.0 + 14.0 * t18 * t49 * param_d * t76 + 140.0 / 9.0 * t18 * t155 * param_d * t160;
        let t164 = t66 * t163;
        let tv2rho20 = -2.0 * t109 * t79 + t135 * t46 + 2.0 * t64 * t142 - t64 * t164 + 2.0 * t61 * t46 - 2.0 * t62 * t80;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t168 = param_b * t49 * t12;
        let t170 = param_b * t56;
        let t171 = t10 * t12;
        let t172 = t170 * t171;
        let t174 = t126 * t4;
        let t175 = 1.0 / t174;
        let t178 = t53 * t175 * t129 * t12;
        let t180 = -8.0 / 3.0 * t168 + 8.0 * t172 - 8.0 / 3.0 * t178;
        let t181 = t4 * t180;
        let t185 = 1.0 / t7 / t4;
        let t186 = t185 * t15;
        let t187 = t186 * t95;
        let t188 = t187 * t100;
        let t190 = t93 * t61;
        let t191 = t190 * t95;
        let t192 = t191 * t100;
        let t194 = t140 * t18;
        let t195 = t94 * t194;
        let t197 = t97 * t99 * t79;
        let t198 = t195 * t197;
        let t200 = 1.0 / t47;
        let t201 = t200 * t15;
        let t203 = t18 * param_d;
        let t204 = t203 * t159;
        let t205 = t201 * t66 * t204;
        let tv2rhosigma0 = t90 * t46 + t181 * t46 - t91 * t80 + 7.0 / 6.0 * t188 - 7.0 / 4.0 * t192 + 7.0 / 2.0 * t198 + 35.0 / 6.0 * t205;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let t211 = -16.0 / 3.0 * t168 + 16.0 * t172 - 16.0 / 3.0 * t178;
        let t212 = t4 * t211;
        let tv2rhosigma1 = t103 * t46 + t212 * t46 - t104 * t80 + 7.0 / 3.0 * t188 - 7.0 / 2.0 * t192 + 7.0 * t198 + 35.0 / 3.0 * t205;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = tv2rhosigma0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = tv2rhosigma2;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = tv2rhosigma1;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let tv2rhosigma5 = tv2rhosigma3;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t220 = param_k * t12;
        let t221 = param_b * t86 * t220;
        let t223 = 1.0 / t126;
        let t226 = t3 * t223 * t129 * t12;
        let t227 = -2.0 * t221 + t226;
        let t228 = t4 * t227;
        let t230 = t93 * t90;
        let t231 = t230 * t95;
        let t233 = 7.0 / 2.0 * t231 * t100;
        let t234 = t175 * t15;
        let t235 = t234 * t140;
        let t236 = t18 * t18;
        let t237 = param_d * param_d;
        let t238 = t236 * t237;
        let t239 = t26 * t26;
        let t241 = t238 * t239 * t29;
        let t242 = t235 * t241;
        let t244 = 1.0 / t5;
        let t245 = t244 * t15;
        let t246 = t245 * t95;
        let t247 = 1.0 / t2;
        let t248 = t158 * t247;
        let t249 = t97 * t248;
        let t250 = t246 * t249;
        let t252 = 1.0 / t29;
        let t253 = t74 * t252;
        let t254 = t97 * t253;
        let t255 = t96 * t254;
        let tv2sigma20 = t228 * t46 - t233 + 49.0 / 8.0 * t242 - 35.0 / 16.0 * t250 + 7.0 / 8.0 * t255;
        v2sigma2[ip * 6] += tv2sigma20;
        let t259 = -4.0 * t221 + 2.0 * t226;
        let t260 = t4 * t259;
        let t262 = t93 * t103;
        let t263 = t262 * t95;
        let t264 = t263 * t100;
        let tv2sigma21 = t260 * t46 - 7.0 / 4.0 * t264 - t233 + 49.0 / 4.0 * t242 - 35.0 / 8.0 * t250 + 7.0 / 4.0 * t255;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = tv2sigma20;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let t271 = -8.0 * t221 + 4.0 * t226;
        let t272 = t4 * t271;
        let tv2sigma23 = t272 * t46 - 7.0 * t264 + 49.0 / 2.0 * t242 - 35.0 / 4.0 * t250 + 7.0 / 2.0 * t255;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = tv2sigma21;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = tv2sigma22;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
