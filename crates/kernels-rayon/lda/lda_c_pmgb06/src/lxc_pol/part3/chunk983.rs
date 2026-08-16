//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 983/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk983(t8131: f64, t10500: f64, t10505: f64, t10509: f64, t10511: f64, t10515: f64, t10518: f64, t10520: f64, t10522: f64, t10525: f64, t10528: f64, t10531: f64, t10533: f64) -> f64 {
    let t11694 = 48.0_f64 * t8131;
    let t11695 = t10500 + t10505 + t10509 - t10511 + t10515 - t10518 - t10520 + t10522 + t10525 + t10528 - t10531 - t11694 + t10533;
    t11695
}
