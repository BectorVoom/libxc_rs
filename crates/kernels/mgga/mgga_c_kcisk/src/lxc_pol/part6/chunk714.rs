//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 714/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk714<F: Float>(t140: F, t2253: F, t430: F, t2257: F, t3783: F, t469: F, t6387: F, t4229: F, t5885: F, t2339: F, t4534: F, t13900: F, t2321: F, t1580: F, t4374: F, t442: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t21256 = t140 * t430 * t2253;
    let t21314 = t2257 * t3783;
    let t21315 = t21314 * sigma0;
    let t21321 = t6387 * t469;
    let t21331 = t5885 * t4229;
    let t21345 = t2339 * t4534;
    let t21620 = t13900 * t2321;
    let t21621 = t1580 * t21620;
    let t21651 = t4374 * t442;
    (t21256, t21315, t21321, t21331, t21345, t21621, t21651)
}
