//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 192/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk192(t518: f64, t529: f64, t166: f64, t161: f64, t183: f64, t398: f64) -> (f64, f64, f64, f64) {
    let t530 = t518 * t529;
    let t531 = t166 * t530;
    let t533 = t161 * t531 / 30.0_f64;
    let t534 = t398 * t183;
    (t530, t531, t533, t534)
}
