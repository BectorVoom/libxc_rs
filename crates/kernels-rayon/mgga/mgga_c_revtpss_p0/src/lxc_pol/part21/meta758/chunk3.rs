//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2669/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2669(t14020: f64, t3957: f64, t2659: f64, t5744: f64, t816: f64, t13792: f64, t48863: f64, t13920: f64, t2661: f64, t3992: f64, t543: f64, t550: f64) -> (f64, f64, f64) {
    let t49134 = t3957 * t14020;
    let t49137 = t816 * t2659 * t5744;
    let t49139 = t49137 * t48863 * t13792;
    let t49144 = t2661 * t3992 * t550 * t13920 * t543;
    (t49134, t49139, t49144)
}
