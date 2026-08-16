//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 543/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk543(t159: f64, t2699: f64, t222: f64, t794: f64, t798: f64, t802: f64, t124: f64, t2430: f64, t800: f64, t234: f64, t2453: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2700 = t2699 * t159;
    let t2702 = 35.0_f64 / 432.0_f64 * t2700 * t222;
    let t2703 = t794 * t798;
    let t2704 = t2703 * t802;
    let t2706 = t124 * t2430;
    let t2707 = t800 * t2706;
    let t2710 = t2453 * t234;
    (t2700, t2702, t2703, t2704, t2706, t2707, t2710)
}
