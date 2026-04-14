//! MGGA_X_SCAN vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 86 shared lines across all orders.
//! Delta: 100 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_scan_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (86 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t20 * t20;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = t26 * t34;
        let t39 = 100.0 / 6561.0 / param_k1 - 73.0 / 648.0;
        let t40 = t21 * t21;
        let t42 = t23 * t22;
        let t43 = 1.0 / t42;
        let t44 = t39 * t40 * t43;
        let t45 = sigma[ip] * sigma[ip];
        let t46 = t45 * t27;
        let t47 = t30 * t30;
        let t48 = t47 * rho[ip];
        let t50 = 1.0 / t20 / t48;
        let t55 = f64::exp(-27.0 / 80.0 * t39 * t21 * t25 * t34);
        let t56 = t50 * t55;
        let t60 = f64::sqrt(146.0);
        let t61 = t60 * t21;
        let t62 = t61 * t25;
        let t65 = tau[ip] * t28;
        let t66 = t31 * rho[ip];
        let t67 = 1.0 / t66;
        let t73 = 5.0 / 9.0 * (t65 * t67 - t34 / 8.0) * t21 * t25;
        let t74 = 1.0 - t73;
        let t76 = t74 * t74;
        let t78 = f64::exp(-t76 / 2.0);
        let t81 = 7.0 / 12960.0 * t62 * t34 + t60 * t74 * t78 / 100.0;
        let t82 = t81 * t81;
        let t83 = param_k1 + 5.0 / 972.0 * t35 + t44 * t46 * t56 / 288.0 + t82;
        let t88 = 1.0 + param_k1 * (1.0 - param_k1 / t83);
        let t89 = t73 <= 1.0;
        let t90 = f64::ln(f64::EPSILON);
        let t93 = t90 / (-t90 + param_c1);
        let t94 = -t93 < t73;
        let t95 = t73 < -t93;
        let t96 = piecewise3(t95, t73, -t93);
        let t97 = param_c1 * t96;
        let t98 = 1.0 - t96;
        let t99 = 1.0 / t98;
        let t101 = f64::exp(-t97 * t99);
        let t102 = piecewise3(t94, 0.0, t101);
        let t103 = f64::abs(param_d);
        let t106 = f64::ln(f64::EPSILON / t103);
        let t109 = (-t106 + param_c2) / t106;
        let t110 = t73 < -t109;
        let t111 = piecewise3(t110, -t109, t73);
        let t112 = 1.0 - t111;
        let t115 = f64::exp(param_c2 / t112);
        let t117 = piecewise3(t110, 0.0, -param_d * t115);
        let t118 = piecewise3(t89, t102, t117);
        let t119 = 1.0 - t118;
        let t122 = t88 * t119 + 0.1174e1 * t118;
        let t124 = f64::sqrt(3.0);
        let t125 = 1.0 / t23;
        let t126 = t40 * t125;
        let t127 = f64::sqrt(sigma[ip]);
        let t128 = t127 * t27;
        let t130 = 1.0 / t20 / rho[ip];
        let t132 = t126 * t128 * t130;
        let t133 = f64::sqrt(t132);
        let t137 = f64::exp(-0.98958e1 * t124 / t133);
        let t138 = 1.0 - t137;
        let t142 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t122 * t138);
        let tzk0 = 2.0 * t142;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (100 lines) ---
        let t143 = 1.0 / t31;
        let t148 = param_k1 * param_k1;
        let t149 = t83 * t83;
        let t151 = t148 / t149;
        let t152 = t30 * rho[ip];
        let t154 = 1.0 / t31 / t152;
        let t155 = t29 * t154;
        let t158 = t47 * t30;
        let t160 = 1.0 / t20 / t158;
        let t161 = t160 * t55;
        let t165 = t39 * t39;
        let t166 = t22 * t22;
        let t167 = 1.0 / t166;
        let t168 = t165 * t167;
        let t169 = t45 * sigma[ip];
        let t170 = t47 * t47;
        let t171 = t170 * rho[ip];
        let t172 = 1.0 / t171;
        let t182 = -5.0 / 3.0 * t65 * t33 + t155 / 3.0;
        let t184 = t26 * t78;
        let t187 = t60 * t76;
        let t191 = -7.0 / 4860.0 * t62 * t155 - t60 * t182 * t184 / 180.0 + t187 * t182 * t184 / 180.0;
        let t194 = -10.0 / 729.0 * t26 * t155 - t44 * t46 * t161 / 54.0 + 3.0 / 80.0 * t168 * t169 * t172 * t55 + 2.0 * t81 * t191;
        let t195 = t194 * t119;
        let t197 = t182 * t21;
        let t199 = 5.0 / 9.0 * t197 * t25;
        let t200 = piecewise3(t95, t199, 0.0);
        let t203 = t98 * t98;
        let t204 = 1.0 / t203;
        let t205 = t204 * t200;
        let t207 = -param_c1 * t200 * t99 - t97 * t205;
        let t208 = t207 * t101;
        let t209 = piecewise3(t94, 0.0, t208);
        let t210 = param_d * param_c2;
        let t211 = t112 * t112;
        let t212 = 1.0 / t211;
        let t213 = piecewise3(t110, 0.0, t199);
        let t217 = piecewise3(t110, 0.0, -t210 * t212 * t213 * t115);
        let t218 = piecewise3(t89, t209, t217);
        let t221 = t151 * t195 - t88 * t218 + 0.1174e1 * t218;
        let t226 = f64::powf(3.0, 1.0 / 6.0);
        let t227 = t226 * t226;
        let t228 = t227 * t227;
        let t230 = t228 * t226 * t18;
        let t231 = 1.0 / t30;
        let t232 = t231 * t122;
        let t234 = 1.0 / t133 / t132;
        let t236 = t230 * t232 * t234;
        let t238 = t126 * t128 * t137;
        let t242 = piecewise3(t3, 0.0, -t19 * t143 * t122 * t138 / 8.0 - 3.0 / 8.0 * t19 * t20 * t221 * t138 - 0.16891736332904387511e1 * t236 * t238);
        let tvrho0 = 2.0 * rho[ip] * t242 + 2.0 * t142;
        vrho[ip] += tvrho0;
        let t245 = t28 * t33;
        let t246 = t245 * t26;
        let t248 = sigma[ip] * t27;
        let t252 = 1.0 / t170;
        let t257 = t25 * t28;
        let t261 = t60 * t28;
        let t262 = t261 * t33;
        let t263 = t262 * t184;
        let t265 = t187 * t28;
        let t267 = t25 * t78;
        let t269 = t265 * t33 * t21 * t267;
        let t271 = 7.0 / 12960.0 * t61 * t257 * t33 + t263 / 1440.0 - t269 / 1440.0;
        let t274 = 5.0 / 972.0 * t246 + t44 * t248 * t56 / 144.0 - 9.0 / 640.0 * t168 * t45 * t252 * t55 + 2.0 * t81 * t271;
        let t275 = t274 * t119;
        let t277 = 5.0 / 72.0 * t246;
        let t278 = piecewise3(t95, -t277, 0.0);
        let t279 = param_c1 * t278;
        let t281 = t204 * t278;
        let t283 = -t279 * t99 - t97 * t281;
        let t284 = t283 * t101;
        let t285 = piecewise3(t94, 0.0, t284);
        let t286 = piecewise3(t110, 0.0, -t277);
        let t290 = piecewise3(t110, 0.0, -t210 * t212 * t286 * t115);
        let t291 = piecewise3(t89, t285, t290);
        let t294 = t151 * t275 - t88 * t291 + 0.1174e1 * t291;
        let t299 = 1.0 / rho[ip];
        let t300 = t299 * t122;
        let t302 = t230 * t300 * t234;
        let t303 = 1.0 / t127;
        let t306 = t126 * t303 * t27 * t137;
        let t310 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t294 * t138 + 0.63344011248391453166e0 * t302 * t306);
        let tvsigma0 = 2.0 * rho[ip] * t310;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t312 = t261 * t67;
        let t318 = t265 * t67 * t21 * t267 / 180.0 - t312 * t184 / 180.0;
        let t319 = t81 * t318;
        let t323 = t28 * t67;
        let t325 = 5.0 / 9.0 * t323 * t26;
        let t326 = piecewise3(t95, t325, 0.0);
        let t327 = param_c1 * t326;
        let t331 = -t97 * t204 * t326 - t327 * t99;
        let t332 = t331 * t101;
        let t333 = piecewise3(t94, 0.0, t332);
        let t334 = piecewise3(t110, 0.0, t325);
        let t338 = piecewise3(t110, 0.0, -t210 * t212 * t334 * t115);
        let t339 = piecewise3(t89, t333, t338);
        let t342 = 2.0 * t151 * t319 * t119 - t88 * t339 + 0.1174e1 * t339;
        let t347 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t342 * t138);
        let tvtau0 = 2.0 * rho[ip] * t347;
        vtau[ip] += tvtau0;
    }
}
