//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 216/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk216<F: Float>(t132: F, t250: F, t249: F, t67: F, t62: F, t256: F, t257: F, t663: F, t666: F, t669: F, t673: F, t675: F, t678: F, t70: F, t31: F, t4: F, t668: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t722 = t132 * t250;
    let t726 = t249 * t67;
    let t727 = 1.0 / t726;
    let t728 = t62 * t727;
    let t729 = t256 * t256;
    let t730 = t729 * t257;
    let t739 = -0.78438333333333333333e0 * t663 + 0.15687666666666666667e1 * t666 + 0.68863333333333333333e0 * t669 + 0.14025833333333333333e0 * t673 + 0.28051666666666666667e0 * t675 + 0.17365833333333333333e0 * t678;
    let t740 = t739 * t257;
    let t743 = t249 * t249;
    let t744 = 1.0 / t743;
    let t745 = t62 * t744;
    let t746 = t70 * t70;
    let t747 = 1.0 / t746;
    let t748 = t729 * t747;
    let t752 = t4 * t668 * t31;
    (t722, t727, t728, t729, t730, t739, t740, t743, t744, t745, t746, t747, t748, t752)
}
