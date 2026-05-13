//! MGGA_X_RPPSCAN vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rppscan.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_rppscan_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_c2: f64,
    param_d: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t70 = t65 * t67 - t34 / 8.0;
        let t73 = param_eta * sigma[ip];
        let t74 = t28 * t33;
        let t77 = 3.0 / 10.0 * t40 * t24 + t73 * t74 / 8.0;
        let t78 = 1.0 / t77;
        let t79 = t70 * t78;
        let t80 = 1.0 - t79;
        let t82 = t80 * t80;
        let t84 = f64::exp(-t82 / 2.0);
        let t87 = 7.0 / 12960.0 * t62 * t34 + t60 * t80 * t84 / 100.0;
        let t88 = t87 * t87;
        let t89 = param_k1 + 5.0 / 972.0 * t35 + t44 * t46 * t56 / 288.0 + t88;
        let t94 = 1.0 + param_k1 * (1.0 - param_k1 / t89);
        let t95 = t79 <= 0.25e1;
        let t96 = 0.25e1 < t79;
        let t97 = piecewise3(t96, 0.25e1, t79);
        let t99 = t97 * t97;
        let t101 = t99 * t97;
        let t103 = t99 * t99;
        let t105 = t103 * t97;
        let t107 = t103 * t99;
        let t112 = piecewise3(t96, t79, 0.25e1);
        let t113 = 1.0 - t112;
        let t116 = f64::exp(param_c2 / t113);
        let t118 = piecewise3(t95, 1.0 - 0.667e0 * t97 - 0.4445555e0 * t99 - 0.663086601049e0 * t101 + 0.145129704449e1 * t103 - 0.887998041597e0 * t105 + 0.234528941479e0 * t107 - 0.23185843322e-1 * t103 * t101, -param_d * t116);
        let t119 = 1.0 - t118;
        let t122 = t94 * t119 + 0.1174e1 * t118;
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
        let t143 = 1.0 / t31;
        let t148 = param_k1 * param_k1;
        let t149 = t89 * t89;
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
        let t184 = t77 * t77;
        let t185 = 1.0 / t184;
        let t186 = t70 * t185;
        let t187 = t186 * param_eta;
        let t190 = -t182 * t78 - t187 * t155 / 3.0;
        let t194 = t60 * t82;
        let t195 = t190 * t84;
        let t198 = -7.0 / 4860.0 * t62 * t155 + t60 * t190 * t84 / 100.0 - t194 * t195 / 100.0;
        let t201 = -10.0 / 729.0 * t26 * t155 - t44 * t46 * t161 / 54.0 + 3.0 / 80.0 * t168 * t169 * t172 * t55 + 2.0 * t87 * t198;
        let t202 = t201 * t119;
        let t204 = -t190;
        let t205 = piecewise3(t96, 0.0, t204);
        let t207 = t97 * t205;
        let t209 = t99 * t205;
        let t211 = t101 * t205;
        let t213 = t103 * t205;
        let t215 = t105 * t205;
        let t220 = param_d * param_c2;
        let t221 = t113 * t113;
        let t222 = 1.0 / t221;
        let t223 = piecewise3(t96, t204, 0.0);
        let t227 = piecewise3(t95, -0.667e0 * t205 - 0.889111e0 * t207 - 0.1989259803147e1 * t209 + 0.580518817796e1 * t211 - 0.4439990207985e1 * t213 + 0.1407173648874e1 * t215 - 0.162300903254e0 * t107 * t205, -t220 * t222 * t223 * t116);
        let t230 = t151 * t202 - t94 * t227 + 0.1174e1 * t227;
        let t235 = f64::powf(3.0, 1.0 / 6.0);
        let t236 = t235 * t235;
        let t237 = t236 * t236;
        let t239 = t237 * t235 * t18;
        let t240 = 1.0 / t30;
        let t241 = t240 * t122;
        let t243 = 1.0 / t133 / t132;
        let t245 = t239 * t241 * t243;
        let t247 = t126 * t128 * t137;
        let t251 = piecewise3(t3, 0.0, -t19 * t143 * t122 * t138 / 8.0 - 3.0 / 8.0 * t19 * t20 * t230 * t138 - 0.16891736332904387511e1 * t245 * t247);
        let tvrho0 = 2.0 * rho[ip] * t251 + 2.0 * t142;
        vrho[ip] += tvrho0;
        let t256 = sigma[ip] * t27;
        let t260 = 1.0 / t170;
        let t265 = t25 * t28;
        let t269 = t74 * t78;
        let t270 = param_eta * t28;
        let t271 = t270 * t33;
        let t274 = t186 * t271 / 8.0 + t269 / 8.0;
        let t275 = t60 * t274;
        let t278 = t274 * t84;
        let t281 = 7.0 / 12960.0 * t61 * t265 * t33 + t275 * t84 / 100.0 - t194 * t278 / 100.0;
        let t284 = 5.0 / 972.0 * t26 * t74 + t44 * t256 * t56 / 144.0 - 9.0 / 640.0 * t168 * t45 * t260 * t55 + 2.0 * t87 * t281;
        let t285 = t284 * t119;
        let t287 = -t274;
        let t288 = piecewise3(t96, 0.0, t287);
        let t290 = t97 * t288;
        let t292 = t99 * t288;
        let t294 = t101 * t288;
        let t296 = t103 * t288;
        let t298 = t105 * t288;
        let t303 = piecewise3(t96, t287, 0.0);
        let t307 = piecewise3(t95, -0.667e0 * t288 - 0.889111e0 * t290 - 0.1989259803147e1 * t292 + 0.580518817796e1 * t294 - 0.4439990207985e1 * t296 + 0.1407173648874e1 * t298 - 0.162300903254e0 * t107 * t288, -t220 * t222 * t303 * t116);
        let t310 = t151 * t285 - t94 * t307 + 0.1174e1 * t307;
        let t315 = 1.0 / rho[ip];
        let t316 = t315 * t122;
        let t318 = t239 * t316 * t243;
        let t319 = 1.0 / t127;
        let t322 = t126 * t319 * t27 * t137;
        let t326 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t310 * t138 + 0.63344011248391453166e0 * t318 * t322);
        let tvsigma0 = 2.0 * rho[ip] * t326;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t328 = t60 * t28;
        let t329 = t67 * t78;
        let t330 = t329 * t84;
        let t332 = t194 * t28;
        let t335 = -t328 * t330 / 100.0 + t332 * t330 / 100.0;
        let t336 = t87 * t335;
        let t340 = t28 * t67;
        let t341 = t340 * t78;
        let t342 = piecewise3(t96, 0.0, t341);
        let t344 = t97 * t342;
        let t346 = t99 * t342;
        let t348 = t101 * t342;
        let t350 = t103 * t342;
        let t352 = t105 * t342;
        let t357 = piecewise3(t96, t341, 0.0);
        let t361 = piecewise3(t95, -0.667e0 * t342 - 0.889111e0 * t344 - 0.1989259803147e1 * t346 + 0.580518817796e1 * t348 - 0.4439990207985e1 * t350 + 0.1407173648874e1 * t352 - 0.162300903254e0 * t107 * t342, -t220 * t222 * t357 * t116);
        let t364 = 2.0 * t151 * t336 * t119 - t94 * t361 + 0.1174e1 * t361;
        let t369 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t364 * t138);
        let tvtau0 = 2.0 * rho[ip] * t369;
        vtau[ip] += tvtau0;
    }
}
