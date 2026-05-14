//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1314/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1314<F: Float>(t2860: F, t7571: F, t17474: F, t17478: F, t1956: F, t3591: F, t730: F, t5893: F, t9351: F, t2782: F, t1855: F, t684: F, t17351: F, t17354: F, t17405: F, t17411: F, t17454: F, t20705: F, t25705: F, t25715: F, t25717: F, t25723: F, t25725: F) -> (F, F, F, F, F, F) {
    let t25959 = 0.70178683471615754484e1 * t2860 * t7571;
    let t25964 = 0.91082604192152556044e5 * t730 * t17474 * t3591 * t17478 * t1956;
    let t25967 = 0.17315859105681463759e2 * t730 * t9351 * t5893;
    let t25968 = t2782 * t2782;
    let t25971 = 4.0 * t1855 * t25968 * t684;
    let t25982 = -0.258925e1 * t25705 + 0.258925e1 * t25715 + 0.16504875e0 * t25717 - 0.14717333333333333333e1 * t17405 + 0.27595e0 * t17411 - 0.18786444444444444444e1 * t20705 + 0.776775e1 * t25723 - 0.16504875e0 * t25725 + t17454 - 0.18786444444444444445e1 * t17351 + 0.40256666666666666667e0 * t17354;
    (t25959, t25964, t25967, t25968, t25971, t25982)
}
