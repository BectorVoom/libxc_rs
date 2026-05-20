//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3039/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3039<F: Float>(t1100: F, t1102: F, t198: F, t336: F, t5023: F, t78094: F, t78096: F, t78099: F, t78154: F, t78478: F, t78686: F, t78690: F, t78694: F, t78696: F, t78698: F, t80166: F, t80211: F, t80819: F, t80869: F, t80918: F, t80967: F, t81015: F, t81068: F) -> F {
    let t81075 = -t5023 * t78478 * t1100 + t78094 + t78096 + t78099 - t78154 + t198 * t336 * (t80166 + t80211 + t80819 + t80869 + t80918 + t80967 + t81015 + t81068) * t1102 + t78686 + t78690 - t78694 - t78696 + t78698;
    t81075
}
