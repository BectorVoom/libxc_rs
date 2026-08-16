//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1273/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1273(t2489: f64, t3223: f64, t5432: f64, t844: f64, t16724: f64, t16727: f64, t16729: f64, t16731: f64, t16734: f64, t16735: f64, t16736: f64, t16737: f64, t16738: f64, t16739: f64, t16742: f64, t16744: f64, t16748: f64) -> (f64, f64, f64) {
    let t16749 = t3223 * t2489;
    let t16750 = 4.0_f64 / 405.0_f64 * t16749;
    let t16752 = t5432 * t844 / 15.0_f64;
    let t16753 = t16724 + t16727 + t16729 + t16731 + t16734 + t16735 + t16736 + t16737 + t16738 + t16739 + t16742 + t16744 + t16748 + t16750 + t16752;
    (t16750, t16752, t16753)
}
