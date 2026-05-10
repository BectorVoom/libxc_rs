//! LDA_C_HL fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 44 shared lines across all orders.
//! Delta: 21 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_HL fxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_hl_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
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
        // --- fxc delta (this level) (21 lines) ---
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
    }
}
