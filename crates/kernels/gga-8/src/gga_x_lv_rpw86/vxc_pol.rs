//! GGA_X_LV_RPW86 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 84 shared lines across all orders.
//! Delta: 103 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_lv_rpw86_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
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
        // --- shared preamble (84 lines) ---
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
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t33 = t28 / t31;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t40 = t33 * sigma0 * t38;
        let t42 = 1.0 + 0.39310185185185185185e-2 * t40;
        let t43 = sigma0 * sigma0;
        let t44 = t43 * sigma0;
        let t45 = t34 * t34;
        let t46 = t45 * t45;
        let t47 = 1.0 / t46;
        let t48 = t44 * t47;
        let t49 = 0.9704561350131285608e-7 * t48;
        let t50 = 1.0 + t49;
        let t51 = 1.0 / t50;
        let t54 = t28 * t28;
        let t57 = t54 / t30 / t29;
        let t58 = t45 * rho0;
        let t60 = 1.0 / t35 / t58;
        let t65 = 1.0 + 0.77125000000000000002e-1 * t40 + 0.30086805555555555556e-1 * t57 * t43 * t60 + 0.72628259874719906066e-6 * t48;
        let t66 = f64::powf(t65, 1.0 / 15.0);
        let t67 = 0.115e1 + t49;
        let t68 = 1.0 / t67;
        let t69 = t66 * t68;
        let t72 = t42 * t51 + 0.9704561350131285608e-7 * t48 * t69;
        let t76 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t72);
        let t77 = rho1 <= dens_threshold;
        let t78 = -t16;
        let t80 = piecewise5(t14, t11, t10, t15, t78 * t7);
        let t81 = 1.0 + t80;
        let t82 = t81 <= zeta_threshold;
        let t83 = pow_1_3(t81);
        let t85 = piecewise3(t82, t22, t83 * t81);
        let t86 = t85 * t26;
        let t87 = rho1 * rho1;
        let t88 = pow_1_3(rho1);
        let t89 = t88 * t88;
        let t91 = 1.0 / t89 / t87;
        let t93 = t33 * sigma2 * t91;
        let t95 = 1.0 + 0.39310185185185185185e-2 * t93;
        let t96 = sigma2 * sigma2;
        let t97 = t96 * sigma2;
        let t98 = t87 * t87;
        let t99 = t98 * t98;
        let t100 = 1.0 / t99;
        let t101 = t97 * t100;
        let t102 = 0.9704561350131285608e-7 * t101;
        let t103 = 1.0 + t102;
        let t104 = 1.0 / t103;
        let t107 = t98 * rho1;
        let t109 = 1.0 / t88 / t107;
        let t114 = 1.0 + 0.77125000000000000002e-1 * t93 + 0.30086805555555555556e-1 * t57 * t96 * t109 + 0.72628259874719906066e-6 * t101;
        let t115 = f64::powf(t114, 1.0 / 15.0);
        let t116 = 0.115e1 + t102;
        let t117 = 1.0 / t116;
        let t118 = t115 * t117;
        let t121 = t95 * t104 + 0.9704561350131285608e-7 * t101 * t118;
        let t125 = piecewise3(t77, 0.0, -3.0 / 8.0 * t5 * t86 * t121);
        let tzk0 = t76 + t125;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (103 lines) ---
        let t126 = t6 * t6;
        let t127 = 1.0 / t126;
        let t128 = t16 * t127;
        let t130 = piecewise5(t10, 0.0, t14, 0.0, t7 - t128);
        let t133 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t130);
        let t134 = t133 * t26;
        let t138 = t26 * t26;
        let t139 = 1.0 / t138;
        let t140 = t25 * t139;
        let t143 = t5 * t140 * t72 / 8.0;
        let t144 = t34 * rho0;
        let t146 = 1.0 / t36 / t144;
        let t147 = sigma0 * t146;
        let t151 = t50 * t50;
        let t152 = 1.0 / t151;
        let t153 = t42 * t152;
        let t154 = t46 * rho0;
        let t155 = 1.0 / t154;
        let t156 = t44 * t155;
        let t161 = t66 * t66;
        let t162 = t161 * t161;
        let t164 = t162 * t162;
        let t165 = t164 * t162 * t161;
        let t166 = 1.0 / t165;
        let t167 = t166 * t68;
        let t170 = t45 * t34;
        let t172 = 1.0 / t35 / t170;
        let t177 = -0.20566666666666666667e0 * t33 * t147 - 0.16046296296296296297e0 * t57 * t43 * t172 - 0.58102607899775924853e-5 * t156;
        let t178 = t167 * t177;
        let t181 = t43 * t43;
        let t182 = t181 * t43;
        let t183 = t46 * t46;
        let t185 = 1.0 / t183 / rho0;
        let t186 = t182 * t185;
        let t187 = t67 * t67;
        let t188 = 1.0 / t187;
        let t189 = t66 * t188;
        let t192 = -0.10482716049382716049e-1 * t33 * t147 * t51 + 0.77636490801050284864e-6 * t153 * t156 - 0.77636490801050284864e-6 * t156 * t69 + 0.64697075667541904053e-8 * t48 * t178 + 0.7534280879876956878e-13 * t186 * t189;
        let t197 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t134 * t72 - t143 - 3.0 / 8.0 * t5 * t27 * t192);
        let t198 = t78 * t127;
        let t200 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t198);
        let t203 = piecewise3(t82, 0.0, 4.0 / 3.0 * t83 * t200);
        let t204 = t203 * t26;
        let t208 = t85 * t139;
        let t211 = t5 * t208 * t121 / 8.0;
        let t213 = piecewise3(t77, 0.0, -3.0 / 8.0 * t5 * t204 * t121 - t211);
        let tvrho0 = t76 + t125 + t6 * (t197 + t213);
        vrho[ip * 2] += tvrho0;
        let t217 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t128);
        let t220 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t217);
        let t221 = t220 * t26;
        let t226 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t221 * t72 - t143);
        let t228 = piecewise5(t14, 0.0, t10, 0.0, t7 - t198);
        let t231 = piecewise3(t82, 0.0, 4.0 / 3.0 * t83 * t228);
        let t232 = t231 * t26;
        let t236 = t87 * rho1;
        let t238 = 1.0 / t89 / t236;
        let t239 = sigma2 * t238;
        let t243 = t103 * t103;
        let t244 = 1.0 / t243;
        let t245 = t95 * t244;
        let t246 = t99 * rho1;
        let t247 = 1.0 / t246;
        let t248 = t97 * t247;
        let t253 = t115 * t115;
        let t254 = t253 * t253;
        let t256 = t254 * t254;
        let t257 = t256 * t254 * t253;
        let t258 = 1.0 / t257;
        let t259 = t258 * t117;
        let t262 = t98 * t87;
        let t264 = 1.0 / t88 / t262;
        let t269 = -0.20566666666666666667e0 * t33 * t239 - 0.16046296296296296297e0 * t57 * t96 * t264 - 0.58102607899775924853e-5 * t248;
        let t270 = t259 * t269;
        let t273 = t96 * t96;
        let t274 = t273 * t96;
        let t275 = t99 * t99;
        let t277 = 1.0 / t275 / rho1;
        let t278 = t274 * t277;
        let t279 = t116 * t116;
        let t280 = 1.0 / t279;
        let t281 = t115 * t280;
        let t284 = -0.10482716049382716049e-1 * t33 * t239 * t104 + 0.77636490801050284864e-6 * t245 * t248 - 0.77636490801050284864e-6 * t248 * t118 + 0.64697075667541904053e-8 * t101 * t270 + 0.7534280879876956878e-13 * t278 * t281;
        let t289 = piecewise3(t77, 0.0, -3.0 / 8.0 * t5 * t232 * t121 - t211 - 3.0 / 8.0 * t5 * t86 * t284);
        let tvrho1 = t76 + t125 + t6 * (t226 + t289);
        vrho[ip * 2 + 1] += tvrho1;
        let t295 = t43 * t47;
        let t306 = 0.77125000000000000002e-1 * t33 * t38 + 0.60173611111111111112e-1 * t57 * sigma0 * t60 + 0.2178847796241597182e-5 * t295;
        let t307 = t167 * t306;
        let t310 = t181 * sigma0;
        let t311 = 1.0 / t183;
        let t312 = t310 * t311;
        let t315 = 0.39310185185185185185e-2 * t33 * t38 * t51 - 0.29113684050393856824e-6 * t153 * t295 + 0.29113684050393856824e-6 * t295 * t69 + 0.64697075667541904053e-8 * t48 * t307 - 0.28253553299538588292e-13 * t312 * t189;
        let t319 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t315);
        let tvsigma0 = t6 * t319;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t323 = t96 * t100;
        let t334 = 0.77125000000000000002e-1 * t33 * t91 + 0.60173611111111111112e-1 * t57 * sigma2 * t109 + 0.2178847796241597182e-5 * t323;
        let t335 = t259 * t334;
        let t338 = t273 * sigma2;
        let t339 = 1.0 / t275;
        let t340 = t338 * t339;
        let t343 = 0.39310185185185185185e-2 * t33 * t91 * t104 - 0.29113684050393856824e-6 * t245 * t323 + 0.29113684050393856824e-6 * t323 * t118 + 0.64697075667541904053e-8 * t101 * t335 - 0.28253553299538588292e-13 * t340 * t281;
        let t347 = piecewise3(t77, 0.0, -3.0 / 8.0 * t5 * t86 * t343);
        let tvsigma2 = t6 * t347;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
