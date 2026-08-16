//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2632/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2632<F: Float>(t2782: F, t2797: F, t62637: F, t18615: F, t251: F, t231: F, t2783: F, t10069: F, t18738: F, t18742: F, t10073: F, t10530: F, t18718: F, t2470: F) -> (F, F, F, F, F, F, F) {
    let t62639 = t2782 * t2797 * t62637;
    let t62641 = t251 * t18615;
    let t62644 = t2782 * t2783 * t62641 * t231;
    let t62649 = t10069 * t18738;
    let t62651 = t10069 * t18742;
    let t62653 = t10073 * t18738;
    let t62665 = t10530 * t18718 * t2470;
    (t62639, t62641, t62644, t62649, t62651, t62653, t62665)
}
