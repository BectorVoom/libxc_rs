//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 930/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk930<F: Float>(t21: F, t3328: F, t3787: F, t15609: F, t126: F, t277: F, t1038: F, t18105: F, t2763: F, t442: F, t966: F, t3074: F, t7592: F, t7877: F, t28415: F, t286: F) -> (F, F, F, F, F, F) {
    let t29568 = t3787 * t3328 * t21;
    let t29571 = t3787 * t15609;
    let t29576 = t277 * t126;
    let t29582 = t2763 * t966 * t1038 * t18105 * t442;
    let t29654 = t7592 * t3074 * t7877;
    let t29664 = t28415 * t286;
    (t29568, t29571, t29576, t29582, t29654, t29664)
}
