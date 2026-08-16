//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1450/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1450<F: Float>(t10069: F, t18742: F, t10073: F, t18738: F, t10530: F, t18718: F, t2470: F, t18761: F, t874: F, t18750: F, t136: F, t2457: F, t2710: F, t6041: F) -> (F, F, F, F, F, F) {
    let t62651 = t10069 * t18742;
    let t62653 = t10073 * t18738;
    let t62665 = t10530 * t18718 * t2470;
    let t62670 = t874 * t18761 * t2470;
    let t62684 = t10073 * t18750;
    let t62716 = t2710 * t6041 * t136 * t2457;
    (t62651, t62653, t62665, t62670, t62684, t62716)
}
