//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3307/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3307<F: Float>(t2482: F, t2801: F, t5977: F, t879: F, t10073: F, t18750: F, t231: F, t2782: F, t2783: F, t6041: F, t836: F, t61756: F) -> (F, F, F, F) {
    let t62682 = t2482 * t879 * t5977 * t2801;
    let t62684 = t10073 * t18750;
    let t62693 = t2782 * t2783 * t6041 * t836 * t231;
    let t62695 = t61756 * t231;
    (t62682, t62684, t62693, t62695)
}
