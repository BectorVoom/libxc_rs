//! GGA_X_PW91 fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw91.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pw91_fxc_unpol(
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
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t37 = f64::exp(-param_alpha * t20 * t25 * t34 / 24.0);
        let t40 = (param_d * t37 + param_c) * t20;
        let t41 = t40 * t25;
        let t44 = t20 * t20;
        let t45 = 1.0 / t23;
        let t46 = t44 * t45;
        let t47 = f64::sqrt(sigma[ip]);
        let t50 = 1.0 / t18 / rho[ip];
        let t51 = t47 * t27 * t50;
        let t54 = f64::powf(t46 * t51 / 12.0, param_expo);
        let t55 = param_f * t54;
        let t56 = t41 * t34 / 24.0 - t55;
        let t57 = t46 * t47;
        let t63 = f64::ln(param_b * t44 * t45 * t51 / 12.0 + f64::sqrt(pow_2(param_b * t44 * t45 * t51 / 12.0) + 1.0));
        let t64 = param_a * t63;
        let t65 = t27 * t50 * t64;
        let t68 = 1.0 + t57 * t65 / 12.0 + t55;
        let t69 = 1.0 / t68;
        let t71 = t56 * t69 + 1.0;
        let t75 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t71);
        let tzk0 = 2.0 * t75;
        zk[ip] += tzk0;
        let t77 = t17 / t31;
        let t81 = param_d * param_alpha;
        let t83 = 1.0 / t23 / t22;
        let t84 = t44 * t83;
        let t85 = t81 * t84;
        let t86 = sigma[ip] * sigma[ip];
        let t87 = t86 * t27;
        let t88 = t30 * t30;
        let t89 = t88 * t30;
        let t91 = 1.0 / t18 / t89;
        let t92 = t91 * t37;
        let t96 = t30 * rho[ip];
        let t98 = 1.0 / t31 / t96;
        let t102 = 1.0 / rho[ip];
        let t105 = 4.0 / 3.0 * t55 * param_expo * t102;
        let t106 = t85 * t87 * t92 / 108.0 - t41 * t29 * t98 / 9.0 + t105;
        let t108 = t68 * t68;
        let t109 = 1.0 / t108;
        let t110 = t56 * t109;
        let t114 = t27 / t18 / t30 * t64;
        let t117 = t20 * t25;
        let t118 = t117 * t29;
        let t120 = param_b * param_b;
        let t125 = 6.0 * t120 * t20 * t25 * t34 + 144.0;
        let t126 = f64::sqrt(t125);
        let t128 = param_b / t126;
        let t129 = t98 * param_a * t128;
        let t132 = -t57 * t114 / 9.0 - 2.0 / 3.0 * t118 * t129 - t105;
        let t134 = t106 * t69 - t110 * t132;
        let t139 = piecewise3(t2, 0.0, -t6 * t77 * t71 / 8.0 - 3.0 / 8.0 * t6 * t19 * t134);
        let tvrho0 = 2.0 * rho[ip] * t139 + 2.0 * t75;
        vrho[ip] += tvrho0;
        let t142 = t88 * rho[ip];
        let t144 = 1.0 / t18 / t142;
        let t145 = t27 * t144;
        let t146 = t37 * sigma[ip];
        let t150 = t25 * t28;
        let t154 = 1.0 / sigma[ip];
        let t157 = t55 * param_expo * t154 / 2.0;
        let t158 = -t85 * t145 * t146 / 288.0 + t40 * t150 * t33 / 24.0 - t157;
        let t161 = t46 / t47;
        let t164 = t117 * t28;
        let t166 = t33 * param_a * t128;
        let t169 = t161 * t65 / 24.0 + t164 * t166 / 4.0 + t157;
        let t171 = -t110 * t169 + t158 * t69;
        let t175 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t171);
        let tvsigma0 = 2.0 * rho[ip] * t175;
        vsigma[ip] += tvsigma0;
        let t180 = t17 / t31 / rho[ip];
        let t187 = t88 * t96;
        let t189 = 1.0 / t18 / t187;
        let t190 = t189 * t37;
        let t194 = param_alpha * param_alpha;
        let t195 = param_d * t194;
        let t196 = t22 * t22;
        let t197 = 1.0 / t196;
        let t198 = t195 * t197;
        let t199 = t86 * sigma[ip];
        let t200 = t88 * t88;
        let t201 = t200 * t30;
        let t202 = 1.0 / t201;
        let t208 = 1.0 / t31 / t88;
        let t212 = param_expo * param_expo;
        let t213 = 1.0 / t30;
        let t214 = t212 * t213;
        let t216 = 16.0 / 9.0 * t55 * t214;
        let t219 = 4.0 / 3.0 * t55 * param_expo * t213;
        let t220 = -t85 * t87 * t190 / 12.0 + t198 * t199 * t202 * t37 / 81.0 + 11.0 / 27.0 * t41 * t29 * t208 - t216 - t219;
        let t222 = t106 * t109;
        let t226 = 1.0 / t108 / t68;
        let t227 = t56 * t226;
        let t228 = t132 * t132;
        let t234 = t27 / t18 / t96 * t64;
        let t238 = t208 * param_a * t128;
        let t241 = t84 * t87;
        let t243 = t120 * param_b;
        let t245 = 1.0 / t126 / t125;
        let t246 = t243 * t245;
        let t247 = t189 * param_a * t246;
        let t250 = 7.0 / 27.0 * t57 * t234 + 10.0 / 3.0 * t118 * t238 - 32.0 / 3.0 * t241 * t247 + t216 + t219;
        let t252 = -t110 * t250 - 2.0 * t222 * t132 + t220 * t69 + 2.0 * t227 * t228;
        let t257 = piecewise3(t2, 0.0, t6 * t180 * t71 / 12.0 - t6 * t77 * t134 / 4.0 - 3.0 / 8.0 * t6 * t19 * t252);
        let tv2rho20 = 2.0 * rho[ip] * t257 + 4.0 * t139;
        v2rho2[ip] += tv2rho20;
        let t263 = t27 * t91;
        let t267 = t200 * rho[ip];
        let t268 = 1.0 / t267;
        let t276 = t212 * t102;
        let t279 = 2.0 / 3.0 * t55 * t276 * t154;
        let t280 = t85 * t263 * t146 / 36.0 - t198 * t268 * t86 * t37 / 216.0 - t40 * t150 * t98 / 9.0 + t279;
        let t282 = t158 * t109;
        let t285 = t169 * t132;
        let t294 = param_a * t243 * t245 * sigma[ip];
        let t297 = -t161 * t114 / 18.0 - t164 * t129 + 4.0 * t84 * t263 * t294 - t279;
        let t299 = -t110 * t297 - t282 * t132 - t222 * t169 + 2.0 * t227 * t285 + t280 * t69;
        let t304 = piecewise3(t2, 0.0, -t6 * t77 * t171 / 8.0 - 3.0 / 8.0 * t6 * t19 * t299);
        let tv2rhosigma0 = 2.0 * rho[ip] * t304 + 2.0 * t175;
        v2rhosigma[ip] += tv2rhosigma0;
        let t307 = 1.0 / t200;
        let t312 = t81 * t44;
        let t313 = t83 * t27;
        let t318 = 1.0 / t86;
        let t321 = t55 * t212 * t318 / 4.0;
        let t324 = t55 * param_expo * t318 / 2.0;
        let t325 = t198 * t307 * t37 * sigma[ip] / 576.0 - t312 * t313 * t144 * t37 / 144.0 - t321 + t324;
        let t329 = t169 * t169;
        let t334 = t46 / t47 / sigma[ip];
        let t338 = t117 * t154 * t28;
        let t341 = t84 * t27;
        let t343 = t144 * param_a * t246;
        let t346 = -t334 * t65 / 48.0 + t338 * t166 / 8.0 - 3.0 / 2.0 * t341 * t343 + t321 - t324;
        let t348 = -t110 * t346 - 2.0 * t282 * t169 + 2.0 * t227 * t329 + t325 * t69;
        let t352 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t348);
        let tv2sigma20 = 2.0 * rho[ip] * t352;
        v2sigma2[ip] += tv2sigma20;
    }
}
