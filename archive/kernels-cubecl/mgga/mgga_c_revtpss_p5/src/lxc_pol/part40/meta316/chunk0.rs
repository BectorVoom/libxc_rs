//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1090/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1090<F: Float>(t11773: F, t3114: F, t1024: F, t3230: F, t11213: F, t225: F, t366: F, t11223: F, t1053: F, t3223: F, t3215: F, t3224: F) -> (F, F, F, F, F, F, F) {
    let t11774 = t3114 * t11773;
    let t11779 = t1024 * t3230;
    let t11782 = t11213 * t225;
    let t11783 = t11782 * t366;
    let t11788 = t11223 * t225;
    let t11792 = t3223 * t1053;
    let t11795 = t3224 * t3215;
    (t11774, t11779, t11782, t11783, t11788, t11792, t11795)
}
