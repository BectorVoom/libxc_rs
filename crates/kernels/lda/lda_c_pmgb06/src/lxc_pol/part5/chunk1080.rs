//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1080/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1080<F: Float>(t10134: F, t20668: F, t20670: F, t20671: F, t20673: F, t20675: F, t20677: F, t20684: F, t20689: F, t20692: F, t20694: F, t20739: F, t20741: F, t20745: F, t20746: F, t20748: F, t20759: F, t20762: F, t20764: F, t20767: F, t20768: F, t20773: F, t20778: F, t20780: F) -> (F, F) {
    let t22015 = -t20668 + t20670 - t20671 - t10134 + t20673 - t20675 - t20677 + t20684 + t20689 + t20692 + t20694 + t20739;
    let t22017 = -t20741 - t20745 - t20746 - t20748 + t20759 - t20762 + t20764 + t20767 - t20768 - t20773 - t20778 - t20780;
    (t22015, t22017)
}
