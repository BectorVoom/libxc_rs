//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 839/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk839<F: Float>(t1111: F, t1992: F, t30147: F, t7586: F, t1165: F, t30209: F, t3044: F, t604: F, t2082: F, t31289: F, t2109: F, t7780: F, t1982: F, t2015: F, t14575: F, t7346: F) -> (F, F, F, F, F, F) {
    let t31708 = t30147 * t7586 * t1992 * t1111;
    let t31720 = t30209 * t1165 * t604 * t3044;
    let t31721 = 0.94344276868812456204e-3 * t31720;
    let t31750 = t31289 * t2082;
    let t31751 = 0.13505315707191967146e-1 * t31750;
    let t31752 = t7780 * t2109;
    let t31773 = t2015 * t1982;
    let t31797 = t7346 * t1165 * t604 * t14575;
    (t31708, t31721, t31751, t31752, t31773, t31797)
}
