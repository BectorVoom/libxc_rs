//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 939/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk939<F: Float>(t1543: F, t2841: F, t2252: F, t2567: F, t1234: F, t2531: F, t560: F, t2654: F, t481: F, t277: F, t7194: F, t2719: F, t6212: F, t19790: F, t938: F, t2526: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24762 = t2841 * t1543;
    let t24786 = t2567 * t2252;
    let t24790 = t2567 * t1234;
    let t24814 = t2531 * t560;
    let t24831 = t2654 * t481;
    let t24877 = t277 * t7194;
    let t24902 = t6212 * t2719;
    let t24906 = t19790 * t938;
    let t24912 = t6212 * t2526;
    (t24762, t24786, t24790, t24814, t24831, t24877, t24902, t24906, t24912)
}
