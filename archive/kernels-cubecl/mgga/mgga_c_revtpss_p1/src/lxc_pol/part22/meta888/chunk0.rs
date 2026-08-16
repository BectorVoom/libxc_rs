//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3076/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3076<F: Float>(t11506: F, t1626: F, t1609: F, t2924: F, t2942: F, t4644: F, t11408: F, t1614: F, t2967: F, t11449: F, t15373: F, t945: F) -> (F, F, F, F, F, F, F) {
    let t52642 = t1626 * t11506;
    let t52645 = t2924 * t1609;
    let t52809 = t4644 * t2942;
    let t52812 = t1614 * t11408;
    let t52820 = t4644 * t2967;
    let t52825 = t1614 * t11449;
    let t52830 = t15373 * t945;
    (t52642, t52645, t52809, t52812, t52820, t52825, t52830)
}
