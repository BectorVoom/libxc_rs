//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2633/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2633<F: Float>(t18719: F, t39609: F, t18761: F, t2470: F, t874: F, t14602: F, t2482: F, t2811: F, t5977: F, t2801: F, t879: F, t10073: F, t18750: F) -> (F, F, F, F, F) {
    let t62667 = t39609 * t18719;
    let t62670 = t874 * t18761 * t2470;
    let t62675 = t2482 * t2811 * t5977 * t14602;
    let t62682 = t2482 * t879 * t5977 * t2801;
    let t62684 = t10073 * t18750;
    (t62667, t62670, t62675, t62682, t62684)
}
