//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 647/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk647<F: Float>(t1280: F, t6573: F, t1287: F, t6688: F, t1774: F, t5486: F, t6587: F, t487: F, t6628: F, t3769: F, t1794: F, t1811: F, t6622: F, t3783: F, t489: F, t6695: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6714 = t1280 * t6573;
    let t6717 = t6688 * t1287;
    let t6720 = t5486 * t1774;
    let t6723 = t1280 * t6587;
    let t6726 = t487 * t6628;
    let t6727 = t6726 * t3769;
    let t6731 = t1811 * t1794 * t1287;
    let t6735 = t487 * t6622 * t1287;
    let t6738 = t6726 * t3783;
    let t6741 = t489 * t6695;
    (t6714, t6717, t6720, t6723, t6727, t6731, t6735, t6738, t6741)
}
