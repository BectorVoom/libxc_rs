//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1361/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1361<F: Float>(t2204: F, t6951: F, t31700: F, t575: F, t31737: F, t571: F, t1913: F, t8433: F, t1921: F, t8416: F, t118108: F, t118110: F, t118203: F, t2212: F, t22533: F, t31464: F, t5790: F, t6937: F, t8331: F, t8349: F) -> (F,) {
    let t118982 = t2204 * t6951;
    let t118983 = t31700 * t575;
    let t118984 = t571 * t31737;
    let t118985 = t1913 * t8433;
    let t118988 = t8416 * t1921;
    let t118990 = 2.0 * t1921 * t31464 + t2212 * t22533 + 2.0 * t5790 * t8433 + t6937 * t8349 + t6951 * t8331 + t118108 + t118110 + t118203 + t118982 + t118983 + t118984 + 2.0 * t118985 + 2.0 * t118988;
    (t118990,)
}
