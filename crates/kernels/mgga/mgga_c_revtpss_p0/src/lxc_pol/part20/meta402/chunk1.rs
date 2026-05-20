//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1492/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1492<F: Float>(t225: F, t42277: F, t366: F, t11792: F, t3215: F, t11951: F, t3224: F, t1025: F, t11809: F, t127: F, t371: F, t1053: F, t11782: F) -> (F, F, F, F, F, F) {
    let t42278 = t42277 * t225;
    let t42279 = t42278 * t366;
    let t42282 = t11792 * t3215;
    let t42284 = t3224 * t11951;
    let t42288 = t1025 * t371 * t127 * t11809;
    let t42290 = t11782 * t1053;
    (t42278, t42279, t42282, t42284, t42288, t42290)
}
