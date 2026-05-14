//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 907/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk907<F: Float>(t2453: F, t3914: F, t1444: F, t2438: F, t138: F, t4075: F, t556: F, t786: F, t4077: F, t676: F, t123: F, t2434: F, t3915: F, t1359: F, t9292: F, t1363: F, t9288: F) -> (F, F, F, F, F, F, F) {
    let t9674 = t2453 * t3914;
    let t9675 = t2438 * t1444;
    let t9676 = t138 * t9675;
    let t9677 = t9674 * t9676;
    let t9679 = t556 * t4075;
    let t9680 = t786 * t9679;
    let t9681 = t676 * t4077;
    let t9682 = t123 * t9681;
    let t9683 = t9680 * t9682;
    let t9685 = t2434 * t1444;
    let t9686 = t123 * t9685;
    let t9687 = t3915 * t9686;
    let t9691 = 0.17073386770573548589e-1 * t9292 * t1359;
    let t9692 = t1363 * t9288;
    (t9674, t9677, t9680, t9683, t9687, t9691, t9692)
}
