//! LDA_C_HL lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 44 shared lines across all orders.
//! Delta: 14 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_HL lxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (44 lines) ---
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t4 = t2 / rho[ip];
        let t5 = param_hl_r_0;
        let t6 = t5 * t5;
        let t7 = t6 * t5;
        let t8 = 1.0 / t7;
        let t11 = 1.0 + 3.0 / 4.0 * t4 * t8;
        let t12 = M_CBRT3;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t2);
        let t15 = 1.0 / t14;
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t23 = 1.0 + t16 * t19 * t5 / 3.0;
        let t24 = f64::ln(t23);
        let t26 = t14 * t14;
        let t27 = t13 * t26;
        let t28 = t18 * t18;
        let t30 = t17 / t28;
        let t31 = 1.0 / t6;
        let t35 = t12 * t14;
        let t36 = t17 * t17;
        let t38 = t36 / t18;
        let t39 = 1.0 / t5;
        let t44 = t1 * (t11 * t24 - t27 * t30 * t31 / 4.0 + t35 * t38 * t39 / 8.0 - 1.0 / 3.0);
        let t46 = pow_1_3(zeta_threshold);
        let t48 = piecewise3(1.0 <= zeta_threshold, t46 * zeta_threshold, 1.0);
        let t51 = M_CBRT2;
        let t55 = (2.0 * t48 - 2.0) / (2.0 * t51 - 2.0);
        let t56 = param_hl_c_1;
        let t57 = param_hl_r_1;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = 1.0 / t59;
        let t63 = 1.0 + 3.0 / 4.0 * t4 * t60;
        let t67 = 1.0 + t16 * t19 * t57 / 3.0;
        let t68 = f64::ln(t67);
        let t70 = 1.0 / t58;
        let t74 = 1.0 / t57;
        let t81 = t55 * (-t56 * (t63 * t68 - t27 * t30 * t70 / 4.0 + t35 * t38 * t74 / 8.0 - 1.0 / 3.0) + t44);
        let tzk0 = -t44 + t81;
        zk[ip] += tzk0;
        // --- vxc delta (16 lines) ---
        let t82 = rho[ip] * rho[ip];
        let t83 = 1.0 / t82;
        let t84 = t2 * t83;
        let t85 = t8 * t24;
        let t89 = t11 * t13 * t15;
        let t90 = 1.0 / t23;
        let t91 = t5 * t90;
        let t97 = t17 / t28 / rho[ip];
        let t103 = t36 / t18 / rho[ip];
        let t108 = t1 * (-3.0 / 4.0 * t84 * t85 + t89 * t30 * t91 / 9.0 + t27 * t97 * t31 / 6.0 - t35 * t103 * t39 / 24.0);
        let t109 = t60 * t68;
        let t113 = t63 * t13 * t15;
        let t114 = 1.0 / t67;
        let t115 = t57 * t114;
        let t128 = t55 * (-t56 * (-3.0 / 4.0 * t84 * t109 + t113 * t30 * t115 / 9.0 + t27 * t97 * t70 / 6.0 - t35 * t103 * t74 / 24.0) + t108);
        let tvrho0 = -t44 + t81 + rho[ip] * (-t108 + t128);
        vrho[ip] += tvrho0;
        // --- fxc delta (21 lines) ---
        let t133 = t82 * rho[ip];
        let t134 = 1.0 / t133;
        let t135 = t2 * t134;
        let t139 = 1.0 / t28 / t82;
        let t140 = t2 * t139;
        let t143 = t16 * t17 * t90;
        let t150 = 1.0 / t26;
        let t151 = t11 * t12 * t150;
        let t152 = t23 * t23;
        let t153 = 1.0 / t152;
        let t154 = t6 * t153;
        let t158 = t17 * t139;
        let t164 = t36 / t18 / t82;
        let t169 = t1 * (3.0 / 2.0 * t135 * t85 - t140 * t31 * t143 / 6.0 - 2.0 / 27.0 * t89 * t97 * t91 - t151 * t103 * t154 / 27.0 - 5.0 / 18.0 * t27 * t158 * t31 + t35 * t164 * t39 / 18.0);
        let t174 = t16 * t17 * t114;
        let t181 = t63 * t12 * t150;
        let t182 = t67 * t67;
        let t183 = 1.0 / t182;
        let t184 = t58 * t183;
        let t197 = t55 * (-t56 * (3.0 / 2.0 * t135 * t109 - t140 * t70 * t174 / 6.0 - 2.0 / 27.0 * t113 * t97 * t115 - t181 * t103 * t184 / 27.0 - 5.0 / 18.0 * t27 * t158 * t70 + t35 * t164 * t74 / 18.0) + t169);
        let tv2rho20 = -2.0 * t108 + 2.0 * t128 + rho[ip] * (-t169 + t197);
        v2rho2[ip] += tv2rho20;
        // --- kxc delta (19 lines) ---
        let t202 = t82 * t82;
        let t203 = 1.0 / t202;
        let t204 = t2 * t203;
        let t208 = 1.0 / t28 / t133;
        let t209 = t2 * t208;
        let t214 = 1.0 / t18 / t133;
        let t215 = t2 * t214;
        let t217 = t12 * t150;
        let t219 = t217 * t36 * t153;
        let t228 = t11 * M_PI;
        let t231 = 1.0 / t152 / t23;
        let t235 = t17 * t208;
        let t239 = t36 * t214;
        let t244 = t1 * (-9.0 / 2.0 * t204 * t85 + 2.0 / 3.0 * t209 * t31 * t143 + t215 * t39 * t219 / 12.0 + 10.0 / 81.0 * t89 * t158 * t91 + 2.0 / 27.0 * t151 * t164 * t154 + 8.0 / 81.0 * t228 * t83 * t7 * t231 + 20.0 / 27.0 * t27 * t235 * t31 - 7.0 / 54.0 * t35 * t239 * t39);
        let t252 = t217 * t36 * t183;
        let t261 = t63 * M_PI;
        let t264 = 1.0 / t182 / t67;
        let t277 = t55 * (-t56 * (-9.0 / 2.0 * t204 * t109 + 2.0 / 3.0 * t209 * t70 * t174 + t215 * t74 * t252 / 12.0 + 10.0 / 81.0 * t113 * t158 * t115 + 2.0 / 27.0 * t181 * t164 * t184 + 8.0 / 81.0 * t261 * t83 * t59 * t264 + 20.0 / 27.0 * t27 * t235 * t70 - 7.0 / 54.0 * t35 * t239 * t74) + t244);
        let tv3rho30 = -3.0 * t169 + 3.0 * t197 + rho[ip] * (-t244 + t277);
        v3rho3[ip] += tv3rho30;
        // --- lxc delta (this level) (14 lines) ---
        let t284 = t2 / t202 / rho[ip];
        let t288 = 1.0 / t28 / t202;
        let t289 = t2 * t288;
        let t294 = 1.0 / t18 / t202;
        let t295 = t2 * t294;
        let t311 = t6 * t6;
        let t314 = t152 * t152;
        let t317 = t15 * t17;
        let t321 = t17 * t288;
        let t325 = t36 * t294;
        let t330 = t1 * (18.0 * t284 * t85 - 82.0 / 27.0 * t289 * t31 * t143 - 5.0 / 9.0 * t295 * t39 * t219 - 8.0 / 27.0 * t203 * t231 - 80.0 / 243.0 * t89 * t235 * t91 - 52.0 / 243.0 * t151 * t239 * t154 - 32.0 / 81.0 * t228 * t134 * t7 * t231 - 8.0 / 243.0 * t228 * t139 * t311 / t314 * t13 * t317 - 220.0 / 81.0 * t27 * t321 * t31 + 35.0 / 81.0 * t35 * t325 * t39);
        let t351 = t58 * t58;
        let t354 = t182 * t182;
        let tv4rho40 = -4.0 * t244 + 4.0 * t277 + rho[ip] * (-t330 + t55 * (-t56 * (18.0 * t284 * t109 - 82.0 / 27.0 * t289 * t70 * t174 - 5.0 / 9.0 * t295 * t74 * t252 - 8.0 / 27.0 * t203 * t264 - 80.0 / 243.0 * t113 * t235 * t115 - 52.0 / 243.0 * t181 * t239 * t184 - 32.0 / 81.0 * t261 * t134 * t59 * t264 - 8.0 / 243.0 * t261 * t139 * t351 / t354 * t13 * t317 - 220.0 / 81.0 * t27 * t321 * t70 + 35.0 / 81.0 * t35 * t325 * t74) + t330));
        v4rho4[ip] += tv4rho40;
    }
}
