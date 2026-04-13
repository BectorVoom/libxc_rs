//! MGGA_X_R2SCAN vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r2scan.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_r2scan_vxc_unpol(
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
    param_dp2: f64,
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
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t7 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t22 = 20.0 / 27.0 + 5.0 / 3.0 * param_eta;
        let t23 = M_CBRT6;
        let t24 = t23 * t23;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t25;
        let t28 = 1.0 / t27;
        let t29 = t24 * t28;
        let t30 = sigma[ip] * sigma[ip];
        let t32 = M_CBRT2;
        let t33 = rho[ip] * rho[ip];
        let t34 = t33 * t33;
        let t35 = t34 * rho[ip];
        let t37 = 1.0 / t20 / t35;
        let t38 = t32 * t37;
        let t39 = param_dp2 * param_dp2;
        let t40 = t39 * t39;
        let t41 = 1.0 / t40;
        let t45 = f64::exp(-t29 * t30 * t38 * t41 / 288.0);
        let t49 = (-0.162742215233874e0 * t22 * t45 + 10.0 / 81.0) * t23;
        let t50 = t26 * t26;
        let t51 = 1.0 / t50;
        let t52 = t49 * t51;
        let t53 = t32 * t32;
        let t54 = sigma[ip] * t53;
        let t55 = t20 * t20;
        let t57 = 1.0 / t55 / t33;
        let t58 = t54 * t57;
        let t61 = param_k1 + t52 * t58 / 24.0;
        let t65 = param_k1 * (1.0 - param_k1 / t61);
        let t66 = tau[ip] * t53;
        let t67 = t55 * rho[ip];
        let t68 = 1.0 / t67;
        let t71 = t66 * t68 - t58 / 8.0;
        let t75 = t53 * t57;
        let t78 = 3.0 / 10.0 * t24 * t50 + param_eta * sigma[ip] * t75 / 8.0;
        let t79 = 1.0 / t78;
        let t80 = t71 * t79;
        let t81 = t80 <= 0.0;
        let t82 = 0.0 < t80;
        let t83 = piecewise3(t82, 0.0, t80);
        let t84 = param_c1 * t83;
        let t85 = 1.0 - t83;
        let t86 = 1.0 / t85;
        let t88 = f64::exp(-t84 * t86);
        let t89 = t80 <= 0.25e1;
        let t90 = 0.25e1 < t80;
        let t91 = piecewise3(t90, 0.25e1, t80);
        let t93 = t91 * t91;
        let t95 = t93 * t91;
        let t97 = t93 * t93;
        let t99 = t97 * t91;
        let t101 = t97 * t93;
        let t106 = piecewise3(t90, t80, 0.25e1);
        let t107 = 1.0 - t106;
        let t110 = f64::exp(param_c2 / t107);
        let t112 = piecewise5(t81, t88, t89, 1.0 - 0.667e0 * t91 - 0.4445555e0 * t93 - 0.663086601049e0 * t95 + 0.145129704449e1 * t97 - 0.887998041597e0 * t99 + 0.234528941479e0 * t101 - 0.23185843322e-1 * t97 * t95, -param_d * t110);
        let t113 = 0.174e0 - t65;
        let t115 = t112 * t113 + t65 + 1.0;
        let t117 = f64::sqrt(3.0);
        let t118 = 1.0 / t26;
        let t119 = t24 * t118;
        let t120 = f64::sqrt(sigma[ip]);
        let t121 = t120 * t32;
        let t123 = 1.0 / t20 / rho[ip];
        let t125 = t119 * t121 * t123;
        let t126 = f64::sqrt(t125);
        let t130 = f64::exp(-0.98958e1 * t117 / t126);
        let t131 = 1.0 - t130;
        let t135 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t115 * t131);
        let tzk0 = 2.0 * t135;
        zk[ip] += tzk0;
        let t136 = 1.0 / t55;
        let t141 = param_k1 * param_k1;
        let t142 = t61 * t61;
        let t143 = 1.0 / t142;
        let t144 = t141 * t143;
        let t145 = t30 * sigma[ip];
        let t146 = t22 * t145;
        let t147 = t34 * t34;
        let t148 = t147 * rho[ip];
        let t149 = 1.0 / t148;
        let t151 = t149 * t41 * t45;
        let t154 = t33 * rho[ip];
        let t156 = 1.0 / t55 / t154;
        let t157 = t54 * t156;
        let t160 = -0.15469524941471936742e-4 * t146 * t151 - t52 * t157 / 9.0;
        let t165 = -5.0 / 3.0 * t66 * t57 + t157 / 3.0;
        let t167 = t78 * t78;
        let t168 = 1.0 / t167;
        let t169 = t71 * t168;
        let t170 = t169 * param_eta;
        let t173 = t165 * t79 + t170 * t157 / 3.0;
        let t174 = piecewise3(t82, 0.0, t173);
        let t177 = t85 * t85;
        let t178 = 1.0 / t177;
        let t179 = t178 * t174;
        let t181 = -param_c1 * t174 * t86 - t84 * t179;
        let t182 = t181 * t88;
        let t183 = piecewise3(t90, 0.0, t173);
        let t185 = t91 * t183;
        let t187 = t93 * t183;
        let t189 = t95 * t183;
        let t191 = t97 * t183;
        let t193 = t99 * t183;
        let t198 = param_d * param_c2;
        let t199 = t107 * t107;
        let t200 = 1.0 / t199;
        let t201 = piecewise3(t90, t173, 0.0);
        let t205 = piecewise5(t81, t182, t89, -0.667e0 * t183 - 0.889111e0 * t185 - 0.1989259803147e1 * t187 + 0.580518817796e1 * t189 - 0.4439990207985e1 * t191 + 0.1407173648874e1 * t193 - 0.162300903254e0 * t101 * t183, -t198 * t200 * t201 * t110);
        let t207 = t112 * t141;
        let t208 = t143 * t160;
        let t210 = t205 * t113 + t144 * t160 - t207 * t208;
        let t215 = f64::powf(3.0, 1.0 / 6.0);
        let t216 = t215 * t215;
        let t217 = t216 * t216;
        let t219 = t217 * t215 * t18;
        let t220 = 1.0 / t33;
        let t221 = t220 * t115;
        let t223 = 1.0 / t126 / t125;
        let t225 = t219 * t221 * t223;
        let t226 = t121 * t130;
        let t227 = t119 * t226;
        let t231 = piecewise3(t3, 0.0, -t19 * t136 * t115 * t131 / 8.0 - 3.0 / 8.0 * t19 * t20 * t210 * t131 - 0.16891736332904387511e1 * t225 * t227);
        let tvrho0 = 2.0 * rho[ip] * t231 + 2.0 * t135;
        vrho[ip] += tvrho0;
        let t234 = t22 * t30;
        let t235 = 1.0 / t147;
        let t237 = t235 * t41 * t45;
        let t240 = t51 * t53;
        let t244 = 0.58010718530519762783e-5 * t234 * t237 + t49 * t240 * t57 / 24.0;
        let t246 = t75 * t79;
        let t247 = param_eta * t53;
        let t248 = t247 * t57;
        let t251 = -t169 * t248 / 8.0 - t246 / 8.0;
        let t252 = piecewise3(t82, 0.0, t251);
        let t253 = param_c1 * t252;
        let t255 = t178 * t252;
        let t257 = -t253 * t86 - t84 * t255;
        let t258 = t257 * t88;
        let t259 = piecewise3(t90, 0.0, t251);
        let t261 = t91 * t259;
        let t263 = t93 * t259;
        let t265 = t95 * t259;
        let t267 = t97 * t259;
        let t269 = t99 * t259;
        let t274 = piecewise3(t90, t251, 0.0);
        let t278 = piecewise5(t81, t258, t89, -0.667e0 * t259 - 0.889111e0 * t261 - 0.1989259803147e1 * t263 + 0.580518817796e1 * t265 - 0.4439990207985e1 * t267 + 0.1407173648874e1 * t269 - 0.162300903254e0 * t101 * t259, -t198 * t200 * t274 * t110);
        let t280 = t143 * t244;
        let t282 = t278 * t113 + t144 * t244 - t207 * t280;
        let t287 = 1.0 / rho[ip];
        let t288 = t287 * t115;
        let t290 = t219 * t288 * t223;
        let t291 = 1.0 / t120;
        let t293 = t291 * t32 * t130;
        let t294 = t119 * t293;
        let t298 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t282 * t131 + 0.63344011248391453166e0 * t290 * t294);
        let tvsigma0 = 2.0 * rho[ip] * t298;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t301 = t53 * t68 * t79;
        let t302 = piecewise3(t82, 0.0, t301);
        let t303 = param_c1 * t302;
        let t305 = t178 * t302;
        let t307 = -t303 * t86 - t84 * t305;
        let t308 = t307 * t88;
        let t309 = piecewise3(t90, 0.0, t301);
        let t311 = t91 * t309;
        let t313 = t93 * t309;
        let t315 = t95 * t309;
        let t317 = t97 * t309;
        let t319 = t99 * t309;
        let t324 = piecewise3(t90, t301, 0.0);
        let t328 = piecewise5(t81, t308, t89, -0.667e0 * t309 - 0.889111e0 * t311 - 0.1989259803147e1 * t313 + 0.580518817796e1 * t315 - 0.4439990207985e1 * t317 + 0.1407173648874e1 * t319 - 0.162300903254e0 * t101 * t309, -t198 * t200 * t324 * t110);
        let t329 = t20 * t328;
        let t330 = t113 * t131;
        let t334 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t329 * t330);
        let tvtau0 = 2.0 * rho[ip] * t334;
        vtau[ip] += tvtau0;
    }
}
