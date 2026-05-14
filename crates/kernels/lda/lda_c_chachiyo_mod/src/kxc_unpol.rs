//! LDA_C_CHACHIYO_MOD kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo_mod.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_CHACHIYO_MOD kxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_chachiyo_mod_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
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
        let t37 = t36 * t36;
        let t38 = piecewise3(1.0 <= zeta_threshold, t37, 1.0);
        let t39 = t38 * t38;
        let t42 = -2.0 * t39 * t38 + 2.0;
        let t43 = (param_af * t32 - t24) * t42;
        let tzk0 = t24 + t43;
        zk[ip] += tzk0;
        let t45 = t8 / t18;
        let t49 = t17 / t9;
        let t52 = t3 * t45 / 9.0 + 2.0 / 9.0 * t13 * t49;
        let t54 = 1.0 / t22;
        let t55 = param_ap * t52 * t54;
        let t60 = t25 * t45 / 9.0 + 2.0 / 9.0 * t28 * t49;
        let t62 = 1.0 / t31;
        let t65 = (param_af * t60 * t62 - t55) * t42;
        let tvrho0 = t24 + t43 + rho[ip] * (t55 + t65);
        vrho[ip] += tvrho0;
        let t72 = t8 / t18 / rho[ip];
        let t76 = t17 / t9 / rho[ip];
        let t79 = -2.0 / 27.0 * t13 * t76 - 2.0 / 27.0 * t3 * t72;
        let t80 = param_ap * t79;
        let t81 = t80 * t54;
        let t82 = t52 * t52;
        let t84 = t22 * t22;
        let t85 = 1.0 / t84;
        let t86 = param_ap * t82 * t85;
        let t90 = -2.0 / 27.0 * t25 * t72 - 2.0 / 27.0 * t28 * t76;
        let t91 = param_af * t90;
        let t93 = t60 * t60;
        let t95 = t31 * t31;
        let t96 = 1.0 / t95;
        let t99 = (-param_af * t93 * t96 + t91 * t62 - t81 + t86) * t42;
        let tv2rho20 = 2.0 * t55 + 2.0 * t65 + rho[ip] * (t81 - t86 + t99);
        v2rho2[ip] += tv2rho20;
        let t105 = rho[ip] * rho[ip];
        let t108 = t8 / t18 / t105;
        let t113 = t17 / t9 / t105;
        let t117 = param_ap * (10.0 / 81.0 * t3 * t108 + 8.0 / 81.0 * t13 * t113);
        let t118 = t117 * t54;
        let t119 = t85 * t52;
        let t120 = t80 * t119;
        let t121 = 3.0 * t120;
        let t125 = 1.0 / t84 / t22;
        let t126 = param_ap * t82 * t52 * t125;
        let t127 = 2.0 * t126;
        let t133 = param_af * (10.0 / 81.0 * t25 * t108 + 8.0 / 81.0 * t28 * t113);
        let t135 = t96 * t60;
        let t141 = 1.0 / t95 / t31;
        let t145 = (2.0 * param_af * t93 * t60 * t141 + t133 * t62 - 3.0 * t91 * t135 - t118 + t121 - t127) * t42;
        let tv3rho30 = 3.0 * t81 - 3.0 * t86 + 3.0 * t99 + rho[ip] * (t118 - t121 + t127 + t145);
        v3rho3[ip] += tv3rho30;
    }
}
