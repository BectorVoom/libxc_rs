//! LDA_C_CHACHIYO lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 28 shared lines across all orders.
//! Delta: 15 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_CHACHIYO lxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_chachiyo_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (28 lines) ---
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = pow_1_3(rho[ip]);
        let t10 = t8 * t9;
        let t13 = param_cp * t1;
        let t14 = t5 * t5;
        let t16 = t7 * t7;
        let t17 = 1.0 / t14 * t16;
        let t18 = t9 * t9;
        let t19 = t17 * t18;
        let t22 = 1.0 + t3 * t10 / 3.0 + t13 * t19 / 3.0;
        let t23 = f64::ln(t22);
        let t24 = param_ap * t23;
        let t25 = param_bf * t2;
        let t28 = param_cf * t1;
        let t31 = 1.0 + t25 * t10 / 3.0 + t28 * t19 / 3.0;
        let t32 = f64::ln(t31);
        let t36 = pow_1_3(zeta_threshold);
        let t38 = piecewise3(1.0 <= zeta_threshold, t36 * zeta_threshold, 1.0);
        let t40 = 2.0 * t38 - 2.0;
        let t42 = M_CBRT2;
        let t45 = 1.0 / (2.0 * t42 - 2.0);
        let t46 = (param_af * t32 - t24) * t40 * t45;
        let tzk0 = t24 + t46;
        zk[ip] += tzk0;
        // --- vxc delta (9 lines) ---
        let t48 = t8 / t18;
        let t52 = t17 / t9;
        let t55 = t3 * t48 / 9.0 + 2.0 / 9.0 * t13 * t52;
        let t57 = 1.0 / t22;
        let t58 = param_ap * t55 * t57;
        let t63 = t25 * t48 / 9.0 + 2.0 / 9.0 * t28 * t52;
        let t65 = 1.0 / t31;
        let t69 = (param_af * t63 * t65 - t58) * t40 * t45;
        let tvrho0 = t24 + t46 + rho[ip] * (t58 + t69);
        vrho[ip] += tvrho0;
        // --- fxc delta (16 lines) ---
        let t76 = t8 / t18 / rho[ip];
        let t80 = t17 / t9 / rho[ip];
        let t83 = -2.0 / 27.0 * t13 * t80 - 2.0 / 27.0 * t3 * t76;
        let t84 = param_ap * t83;
        let t85 = t84 * t57;
        let t86 = t55 * t55;
        let t88 = t22 * t22;
        let t89 = 1.0 / t88;
        let t90 = param_ap * t86 * t89;
        let t94 = -2.0 / 27.0 * t25 * t76 - 2.0 / 27.0 * t28 * t80;
        let t95 = param_af * t94;
        let t97 = t63 * t63;
        let t99 = t31 * t31;
        let t100 = 1.0 / t99;
        let t104 = (-param_af * t97 * t100 + t95 * t65 - t85 + t90) * t40 * t45;
        let tv2rho20 = 2.0 * t58 + 2.0 * t69 + rho[ip] * (t85 - t90 + t104);
        v2rho2[ip] += tv2rho20;
        // --- kxc delta (16 lines) ---
        let t110 = rho[ip] * rho[ip];
        let t113 = t8 / t18 / t110;
        let t118 = t17 / t9 / t110;
        let t122 = param_ap * (10.0 / 81.0 * t3 * t113 + 8.0 / 81.0 * t13 * t118);
        let t123 = t122 * t57;
        let t124 = t89 * t55;
        let t125 = t84 * t124;
        let t126 = 3.0 * t125;
        let t130 = 1.0 / t88 / t22;
        let t131 = param_ap * t86 * t55 * t130;
        let t132 = 2.0 * t131;
        let t138 = param_af * (10.0 / 81.0 * t25 * t113 + 8.0 / 81.0 * t28 * t118);
        let t140 = t100 * t63;
        let t146 = 1.0 / t99 / t31;
        let t151 = (2.0 * param_af * t97 * t63 * t146 + t138 * t65 - 3.0 * t95 * t140 - t123 + t126 - t132) * t40 * t45;
        let tv3rho30 = 3.0 * t85 - 3.0 * t90 + 3.0 * t104 + rho[ip] * (t123 - t126 + t132 + t151);
        v3rho3[ip] += tv3rho30;
        // --- lxc delta (this level) (15 lines) ---
        let t158 = t110 * rho[ip];
        let t161 = t8 / t18 / t158;
        let t166 = t17 / t9 / t158;
        let t171 = param_ap * (-80.0 / 243.0 * t3 * t161 - 56.0 / 243.0 * t13 * t166) * t57;
        let t173 = 4.0 * t122 * t124;
        let t176 = 12.0 * t84 * t130 * t86;
        let t177 = t83 * t83;
        let t180 = 3.0 * param_ap * t177 * t89;
        let t181 = t86 * t86;
        let t183 = t88 * t88;
        let t186 = 6.0 * param_ap * t181 / t183;
        let t199 = t94 * t94;
        let t203 = t97 * t97;
        let t205 = t99 * t99;
        let tv4rho40 = 4.0 * t123 - 12.0 * t125 + 8.0 * t131 + 4.0 * t151 + rho[ip] * (t171 - t173 + t176 - t180 - t186 + (param_af * (-80.0 / 243.0 * t25 * t161 - 56.0 / 243.0 * t28 * t166) * t65 - 4.0 * t138 * t140 + 12.0 * t95 * t146 * t97 - 3.0 * param_af * t199 * t100 - 6.0 * param_af * t203 / t205 - t171 + t173 - t176 + t180 + t186) * t40 * t45);
        v4rho4[ip] += tv4rho40;
    }
}
