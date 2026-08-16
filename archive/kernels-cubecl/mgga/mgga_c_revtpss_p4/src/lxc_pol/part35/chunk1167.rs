//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1167/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1167<F: Float>(t22399: F, t26265: F, t101970: F, t28154: F, t108879: F, t2047: F, t60673: F, t7342: F, t2247: F, t5819: F, t68: F, t1469: F, t603: F) -> (F, F, F, F, F, F) {
    let t109858 = t26265 * t22399;
    let t109892 = t28154 * t101970;
    let t109911 = t2047 * t108879;
    let t109926 = t60673 * t7342;
    let t109976 = t2247 * t5819 * t68;
    let t109980 = t603 * t1469 * t68;
    (t109858, t109892, t109911, t109926, t109976, t109980)
}
