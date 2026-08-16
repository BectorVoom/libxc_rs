//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1162/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1162(t2488: f64, t3194: f64, t493: f64, t2493: f64, t3177: f64, t14316: f64, t15257: f64, t15258: f64, t15259: f64, t15260: f64, t15261: f64, t15263: f64, t15268: f64, t15270: f64, t15273: f64, t15278: f64, t15280: f64, t15282: f64) -> (f64, f64, f64) {
    let t15285 = 2.0_f64 / 45.0_f64 * t493 * t3194 * t2488;
    let t15287 = 2.0_f64 / 45.0_f64 * t3177 * t2493;
    let t15288 = -0.022363485482220676_f64 * t14316 - t15257 + t15258 + t15259 - t15260 - t15261 + t15263 - t15268 + t15270 + t15273 + t15278 - t15280 - t15282 - t15285 - t15287;
    (t15285, t15287, t15288)
}
