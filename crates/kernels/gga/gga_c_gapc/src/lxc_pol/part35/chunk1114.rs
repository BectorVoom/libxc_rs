//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1114/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1114<F: Float>(t10292: F, t11684: F, t11691: F, t2165: F, t24761: F, t35682: F, t6857: F, t8452: F, t10110: F, t2493: F, t3243: F, t640: F, t10336: F, t11695: F, t3209: F, t35846: F, t923: F) -> (F, F, F, F, F, F, F) {
    let t35932 = t10292 * t11684;
    let t35934 = t2165 * t11691;
    let t35938 = t35682 * t24761 * t8452 * t6857;
    let t35940 = t10110 * t11691;
    let t35943 = t3243 * t640 * t2493;
    let t35945 = t10336 * t11695;
    let t35948 = t3209 * t35846 * t923;
    (t35932, t35934, t35938, t35940, t35943, t35945, t35948)
}
