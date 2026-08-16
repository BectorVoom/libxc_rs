//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2959/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2959<F: Float>(t1408: F, t241: F, t820: F, t2482: F, t814: F, t9991: F, t13805: F, t13847: F, t13848: F, t46917: F, t5706: F, t47201: F) -> (F, F, F, F, F) {
    let t48712 = t820 * t1408 * t241;
    let t48731 = t2482 * t9991 * t814;
    let t48734 = t48731 * t13847 * t13848 * t13805;
    let t48756 = t46917 * t5706;
    let t48759 = t820 * t47201 * t241;
    (t48712, t48731, t48734, t48756, t48759)
}
