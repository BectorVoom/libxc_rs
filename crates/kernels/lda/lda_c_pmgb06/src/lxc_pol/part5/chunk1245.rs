//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1245/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1245<F: Float>(t13292: F, t13295: F, t20648: F, t20651: F, t20654: F, t20656: F, t20658: F, t20660: F, t20663: F, t20666: F, t20667: F, t10134: F, t20668: F, t20670: F, t20671: F, t20673: F, t20675: F, t20677: F, t20684: F, t20689: F, t20692: F, t20694: F, t20739: F) -> (F, F) {
    let t22014 = t20648 + t20651 + t20654 + t20656 + t20658 + t20660 - t20663 + t20666 - t13292 - t13295 - t20667;
    let t22015 = -t20668 + t20670 - t20671 - t10134 + t20673 - t20675 - t20677 + t20684 + t20689 + t20692 + t20694 + t20739;
    (t22014, t22015)
}
