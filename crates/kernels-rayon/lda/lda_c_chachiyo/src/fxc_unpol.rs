//! LDA_C_CHACHIYO fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_chachiyo_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t23 = rmath::ln(t22);
        let t24 = param_ap * t23;
        let t25 = param_bf * t2;
        let t28 = param_cf * t1;
        let t31 = 1.0 + t25 * t10 / 3.0 + t28 * t19 / 3.0;
        let t32 = rmath::ln(t31);
        let t36 = pow_1_3(zeta_threshold);
        let t38 = piecewise3(1.0 <= zeta_threshold, t36 * zeta_threshold, 1.0);
        let t40 = 2.0 * t38 - 2.0;
        let t42 = M_CBRT2;
        let t45 = 1.0 / (2.0 * t42 - 2.0);
        let t46 = (param_af * t32 - t24) * t40 * t45;
        let tzk0 = t24 + t46;
        zk[ip] += tzk0;
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
    }
}
