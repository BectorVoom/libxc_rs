//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1236/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1236<F: Float>(t28903: F, t8669: F, t20703: F, t20706: F, t20714: F, t24556: F, t24559: F, t24562: F, t28853: F, t28856: F, t28859: F, t796: F, t789: F, t24658: F, t24661: F, t24664: F, t24667: F, t24670: F, t24673: F) -> (F, F, F, F) {
    let t28907 = t8669 * t28903;
    let t28916 = t20714 - 56.0 / 27.0 * t20703 + 4.0 / 9.0 * t20706 - 56.0 / 27.0 * t24556 + 16.0 / 9.0 * t24559 - 2.0 / 3.0 * t24562 + 4.0 / 9.0 * t28859 - 2.0 / 3.0 * t28853 + t28856;
    let t28917 = t796 * t28916;
    let t28919 = t789 * t28916;
    let t28930 = 0.776775e1 * t28907 + 0.16504875e0 * t28917 + 0.258925e1 * t28919 - 0.18786444444444444444e1 * t24556 + 0.16102666666666666667e1 * t24559 - 0.60385e0 * t24562 + 0.11038e1 * t24658 + 0.11038e1 * t24661 - 0.14717333333333333333e1 * t24664 - 0.33114e0 * t24667 - 0.66228e0 * t24670 - 0.33114e0 * t24673;
    (t28907, t28917, t28919, t28930)
}
