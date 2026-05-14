//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 998/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk998<F: Float>(t20676: F, t10134: F, t13292: F, t13295: F, t20663: F, t20666: F, t20667: F, t20668: F, t20670: F, t20671: F, t20673: F, t20675: F, t2002: F, t6551: F, t1592: F, t7801: F) -> (F, F, F, F) {
    let t20677 = 4.0 / 45.0 * t20676;
    let t20678 = -t20663 + t20666 - t13292 - t13295 - t20667 - t20668 + t20670 - t20671 - t10134 + t20673 - t20675 - t20677;
    let t20684 = 2.0 / 5.0 * t2002 * t6551;
    let t20685 = t1592 * t7801;
    (t20677, t20678, t20684, t20685)
}
