//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2149/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2149<F: Float>(t15609: F, t4893: F, t3117: F, t4583: F, t4786: F, t3092: F, t3090: F, t4954: F) -> (F, F, F, F, F) {
    let t15610 = t4893 * t15609;
    let t15611 = t3117 * t15610;
    let t15614 = t4583 * t4786;
    let t15615 = t3092 * t15614;
    let t15618 = t4954 * t3090;
    (t15610, t15611, t15614, t15615, t15618)
}
