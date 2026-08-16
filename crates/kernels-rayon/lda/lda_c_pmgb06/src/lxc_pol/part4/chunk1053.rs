//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1053/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1053(t247: f64, t28: f64, t769: f64, t8276: f64, t3615: f64, t63: f64, t370: f64, t38: f64, t8281: f64, t2195: f64, t642: f64, t2203: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11227 = t769 * t28 * t247;
    let t11228 = t8276 * t11227;
    let t11230 = t63 * t3615;
    let t11234 = t38 * t370;
    let t11237 = t8281 * t11227;
    let t11259 = 16.0_f64 * t2195 * t642;
    let t11282 = 16.0_f64 * t2203 * t642;
    (t11228, t11230, t11234, t11237, t11259, t11282)
}
