//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 248/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk248(t769: f64, t77: f64, t56: f64, t38: f64, t64: f64) -> (f64, f64, f64, f64) {
    let t770 = t77 * t769;
    let t773 = t56 * t769;
    let t775 = 2.923025_f64 * t38 * t773;
    let t776 = t64 * t769;
    (t770, t773, t775, t776)
}
