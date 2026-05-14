//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 566/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk566<F: Float>(t1214: F, t1263: F, t1122: F, t1042: F, t1209: F, t1284: F, t3624: F, t482: F, t66: F, t828: F) -> (F, F, F, F, F, F, F) {
    let t3712 = t1263 * t1214;
    let t3713 = t3712 * t1122;
    let t3714 = t1042 * t3713;
    let t3717 = t1209 * t1284;
    let t3718 = t3717 * t3624;
    let t3719 = t66 * t482;
    let t3720 = t828 * t3719;
    (t3712, t3713, t3714, t3717, t3718, t3719, t3720)
}
