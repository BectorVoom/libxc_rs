//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2094/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2094<F: Float>(t10073: F, t25937: F, t7282: F, t7910: F, t25899: F, t97899: F, t25953: F, t27899: F, t25981: F, t5677: F, t820: F, t844: F) -> (F, F, F, F) {
    let t98099 = t10073 * t7282 * t25937 * t7910;
    let t98101 = t25899 * t97899;
    let t98104 = t27899 * t25953;
    let t98108 = t820 * t25981 * t844 * t5677;
    (t98099, t98101, t98104, t98108)
}
