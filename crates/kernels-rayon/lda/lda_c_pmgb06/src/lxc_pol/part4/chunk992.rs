//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 992/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk992(t962: f64, t977: f64, t3741: f64, t696: f64, t8599: f64, t3760: f64, t971: f64, t138: f64, t28: f64, t4238: f64, t8333: f64, t3679: f64, t642: f64) -> (f64, f64, f64, f64, f64) {
    let t8688 = 1.0_f64 / t962 / t977;
    let t8692 = 12304.822629859687_f64 * t696 * t8688 * t8599 * t3741;
    let t8693 = t971 * t3760;
    let t8697 = t8333 * t28 * t4238 * t138;
    let t8699 = t3679 * t642;
    (t8688, t8692, t8693, t8697, t8699)
}
