//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1245/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1245(t13292: f64, t13295: f64, t20648: f64, t20651: f64, t20654: f64, t20656: f64, t20658: f64, t20660: f64, t20663: f64, t20666: f64, t20667: f64, t10134: f64, t20668: f64, t20670: f64, t20671: f64, t20673: f64, t20675: f64, t20677: f64, t20684: f64, t20689: f64, t20692: f64, t20694: f64, t20739: f64) -> (f64, f64) {
    let t22014 = t20648 + t20651 + t20654 + t20656 + t20658 + t20660 - t20663 + t20666 - t13292 - t13295 - t20667;
    let t22015 = -t20668 + t20670 - t20671 - t10134 + t20673 - t20675 - t20677 + t20684 + t20689 + t20692 + t20694 + t20739;
    (t22014, t22015)
}
