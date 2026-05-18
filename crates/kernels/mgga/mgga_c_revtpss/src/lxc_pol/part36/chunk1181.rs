//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1181/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1181<F: Float>(t30031: F, t7296: F, t6846: F, t7264: F, t6880: F, t7271: F, t6856: F, t6876: F, t26017: F, t6850: F, t26028: F, t6871: F) -> (F, F, F, F, F, F, F) {
    let t30032 = t7296 * t30031;
    let t30035 = t7264 * t6846;
    let t30037 = t7271 * t6880;
    let t30039 = t7271 * t6856;
    let t30041 = t7264 * t6876;
    let t30043 = t26017 * t6850;
    let t30045 = t26028 * t6871;
    (t30032, t30035, t30037, t30039, t30041, t30043, t30045)
}
