//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1840/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1840<F: Float>(t11788: F, t366: F, t1053: F, t3223: F, t3215: F, t3224: F, t3111: F, t3188: F, t3211: F, t1026: F, t371: F, t676: F) -> (F, F, F, F, F, F) {
    let t11789 = t11788 * t366;
    let t11792 = t3223 * t1053;
    let t11795 = t3224 * t3215;
    let t11802 = t3188 * t3111;
    let t11814 = t3211 * t3215;
    let t11817 = t371 * t676 * t1026;
    (t11789, t11792, t11795, t11802, t11814, t11817)
}
