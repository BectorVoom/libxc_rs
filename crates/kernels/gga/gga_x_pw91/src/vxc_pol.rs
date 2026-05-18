//! GGA_X_PW91 vxc pol kernel.
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
pub fn gga_x_pw91_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
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
        let t29 = param_alpha * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = t34 * t39;
        let t43 = f64::exp(-t29 * t40 / 24.0);
        let t46 = (param_d * t43 + param_c) * t28;
        let t49 = t28 * t28;
        let t50 = 1.0 / t31;
        let t51 = t49 * t50;
        let t52 = f64::sqrt(sigma0);
        let t54 = 1.0 / t36 / rho0;
        let t58 = f64::powf(t51 * t52 * t54 / 12.0, param_expo);
        let t59 = param_f * t58;
        let t60 = t46 * t40 / 24.0 - t59;
        let t61 = t51 * t52;
        let t63 = param_b * t49;
        let t68 = f64::ln(t63 * t50 * t52 * t54 / 12.0 + f64::sqrt(pow_2::<f64>(t63 * t50 * t52 * t54 / 12.0) + 1.0));
        let t69 = t54 * param_a * t68;
        let t72 = 1.0 + t61 * t69 / 12.0 + t59;
        let t73 = 1.0 / t72;
        let t75 = t60 * t73 + 1.0;
        let t79 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t75);
        let t80 = rho1 <= dens_threshold;
        let t81 = -t16;
        let t83 = piecewise5::<f64>(t14, t11, t10, t15, t81 * t7);
        let t84 = 1.0 + t83;
        let t85 = t84 <= zeta_threshold;
        let t86 = pow_1_3::<f64>(t84);
        let t88 = piecewise3::<f64>(t85, t22, t86 * t84);
        let t89 = t88 * t26;
        let t90 = t33 * sigma2;
        let t91 = rho1 * rho1;
        let t92 = pow_1_3::<f64>(rho1);
        let t93 = t92 * t92;
        let t95 = 1.0 / t93 / t91;
        let t96 = t90 * t95;
        let t99 = f64::exp(-t29 * t96 / 24.0);
        let t102 = (param_d * t99 + param_c) * t28;
        let t105 = f64::sqrt(sigma2);
        let t107 = 1.0 / t92 / rho1;
        let t111 = f64::powf(t51 * t105 * t107 / 12.0, param_expo);
        let t112 = param_f * t111;
        let t113 = t102 * t96 / 24.0 - t112;
        let t114 = t51 * t105;
        let t120 = f64::ln(t63 * t50 * t105 * t107 / 12.0 + f64::sqrt(pow_2::<f64>(t63 * t50 * t105 * t107 / 12.0) + 1.0));
        let t121 = t107 * param_a * t120;
        let t124 = 1.0 + t114 * t121 / 12.0 + t112;
        let t125 = 1.0 / t124;
        let t127 = t113 * t125 + 1.0;
        let t131 = piecewise3::<f64>(t80, 0.0, -3.0 / 8.0 * t5 * t89 * t127);
        let tzk0 = t79 + t131;
        zk[ip] += tzk0;
        let t132 = t6 * t6;
        let t133 = 1.0 / t132;
        let t134 = t16 * t133;
        let t136 = piecewise5::<f64>(t10, 0.0, t14, 0.0, t7 - t134);
        let t139 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t136);
        let t140 = t139 * t26;
        let t144 = t26 * t26;
        let t145 = 1.0 / t144;
        let t146 = t25 * t145;
        let t149 = t5 * t146 * t75 / 8.0;
        let t151 = param_d * param_alpha * t49;
        let t153 = 1.0 / t31 / t30;
        let t154 = sigma0 * sigma0;
        let t155 = t153 * t154;
        let t156 = t35 * t35;
        let t157 = t156 * t35;
        let t159 = 1.0 / t36 / t157;
        let t164 = t35 * rho0;
        let t166 = 1.0 / t37 / t164;
        let t170 = 1.0 / rho0;
        let t173 = 4.0 / 3.0 * t59 * param_expo * t170;
        let t174 = t151 * t155 * t159 * t43 / 216.0 - t46 * t34 * t166 / 9.0 + t173;
        let t176 = t72 * t72;
        let t177 = 1.0 / t176;
        let t178 = t60 * t177;
        let t182 = 1.0 / t36 / t35 * param_a * t68;
        let t185 = t28 * t33;
        let t186 = t185 * sigma0;
        let t188 = param_b * param_b;
        let t189 = t188 * t28;
        let t192 = 6.0 * t189 * t40 + 144.0;
        let t193 = f64::sqrt(t192);
        let t194 = 1.0 / t193;
        let t195 = param_b * t194;
        let t196 = t166 * param_a * t195;
        let t199 = -t61 * t182 / 9.0 - 2.0 / 3.0 * t186 * t196 - t173;
        let t201 = t174 * t73 - t178 * t199;
        let t206 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t140 * t75 - t149 - 3.0 / 8.0 * t5 * t27 * t201);
        let t207 = t81 * t133;
        let t209 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -t7 - t207);
        let t212 = piecewise3::<f64>(t85, 0.0, 4.0 / 3.0 * t86 * t209);
        let t213 = t212 * t26;
        let t217 = t88 * t145;
        let t220 = t5 * t217 * t127 / 8.0;
        let t222 = piecewise3::<f64>(t80, 0.0, -3.0 / 8.0 * t5 * t213 * t127 - t220);
        let tvrho0 = t79 + t131 + t6 * (t206 + t222);
        vrho[ip * 2] += tvrho0;
        let t226 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -t7 - t134);
        let t229 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t226);
        let t230 = t229 * t26;
        let t235 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t230 * t75 - t149);
        let t237 = piecewise5::<f64>(t14, 0.0, t10, 0.0, t7 - t207);
        let t240 = piecewise3::<f64>(t85, 0.0, 4.0 / 3.0 * t86 * t237);
        let t241 = t240 * t26;
        let t245 = sigma2 * sigma2;
        let t246 = t153 * t245;
        let t247 = t91 * t91;
        let t248 = t247 * t91;
        let t250 = 1.0 / t92 / t248;
        let t255 = t91 * rho1;
        let t257 = 1.0 / t93 / t255;
        let t261 = 1.0 / rho1;
        let t264 = 4.0 / 3.0 * t112 * param_expo * t261;
        let t265 = t151 * t246 * t250 * t99 / 216.0 - t102 * t90 * t257 / 9.0 + t264;
        let t267 = t124 * t124;
        let t268 = 1.0 / t267;
        let t269 = t113 * t268;
        let t273 = 1.0 / t92 / t91 * param_a * t120;
        let t276 = t185 * sigma2;
        let t280 = 6.0 * t189 * t96 + 144.0;
        let t281 = f64::sqrt(t280);
        let t282 = 1.0 / t281;
        let t283 = param_b * t282;
        let t284 = t257 * param_a * t283;
        let t287 = -t114 * t273 / 9.0 - 2.0 / 3.0 * t276 * t284 - t264;
        let t289 = t265 * t125 - t269 * t287;
        let t294 = piecewise3::<f64>(t80, 0.0, -3.0 / 8.0 * t5 * t241 * t127 - t220 - 3.0 / 8.0 * t5 * t89 * t289);
        let tvrho1 = t79 + t131 + t6 * (t235 + t294);
        vrho[ip * 2 + 1] += tvrho1;
        let t297 = t156 * rho0;
        let t299 = 1.0 / t36 / t297;
        let t300 = t153 * t299;
        let t301 = t43 * sigma0;
        let t308 = 1.0 / sigma0;
        let t311 = t59 * param_expo * t308 / 2.0;
        let t312 = -t151 * t300 * t301 / 576.0 + t46 * t33 * t39 / 24.0 - t311;
        let t315 = t51 / t52;
        let t319 = param_a * param_b;
        let t320 = t319 * t194;
        let t323 = t315 * t69 / 24.0 + t185 * t39 * t320 / 4.0 + t311;
        let t325 = -t178 * t323 + t312 * t73;
        let t329 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t325);
        let tvsigma0 = t6 * t329;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t330 = t247 * rho1;
        let t332 = 1.0 / t92 / t330;
        let t333 = t153 * t332;
        let t334 = t99 * sigma2;
        let t341 = 1.0 / sigma2;
        let t344 = t112 * param_expo * t341 / 2.0;
        let t345 = -t151 * t333 * t334 / 576.0 + t102 * t33 * t95 / 24.0 - t344;
        let t348 = t51 / t105;
        let t352 = t319 * t282;
        let t355 = t348 * t121 / 24.0 + t185 * t95 * t352 / 4.0 + t344;
        let t357 = t345 * t125 - t269 * t355;
        let t361 = piecewise3::<f64>(t80, 0.0, -3.0 / 8.0 * t5 * t89 * t357);
        let tvsigma2 = t6 * t361;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
