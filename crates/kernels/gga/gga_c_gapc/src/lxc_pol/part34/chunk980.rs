//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 980/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk980<F: Float>(t1: F, t2546: F, t286: F, t3: F, t3074: F, t786: F, t10057: F, t7191: F, t21: F, t3328: F, t3787: F, t15609: F, t126: F, t277: F, t1038: F, t18105: F, t2763: F, t442: F, t966: F) -> (F, F, F, F, F, F) {
    let t29481 = t2546 * t3074 * t286 * t1 * t3 * t786;
    let t29516 = t10057 * t7191;
    let t29568 = t3787 * t3328 * t21;
    let t29571 = t3787 * t15609;
    let t29576 = t277 * t126;
    let t29582 = t2763 * t966 * t1038 * t18105 * t442;
    (t29481, t29516, t29568, t29571, t29576, t29582)
}
