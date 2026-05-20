//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3311/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3311<F: Float>(t5977: F, t860: F, t231: F, t2782: F, t2783: F, t18657: F, t233: F, t689: F, t869: F, t10069: F, t18750: F, t822: F) -> (F, F, F, F, F) {
    let t62760 = t860 * t5977;
    let t62763 = t2782 * t2783 * t62760 * t231;
    let t62775 = t689 * t869 * t233 * t18657;
    let t62777 = t10069 * t18750;
    let t62788 = t822 * t18657;
    (t62760, t62763, t62775, t62777, t62788)
}
