//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 516/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk516(t76: f64, t769: f64, t1246: f64, t348: f64, t773: f64, t350: f64, t342: f64, t38: f64, t776: f64, t1212: f64, t760: f64, t1: f64, t330: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2181 = t76 * t769;
    let t2185 = 0.48717083333333333_f64 * t1246;
    let t2186 = t348 * t773;
    let t2187 = t2186 * t350;
    let t2188 = 0.48717083333333333_f64 * t2187;
    let t2191 = 5.84605_f64 * t38 * t776 * t342;
    let t2192 = t1212 * t760;
    let t2195 = t330 * t1;
    (t2181, t2185, t2186, t2188, t2191, t2192, t2195)
}
