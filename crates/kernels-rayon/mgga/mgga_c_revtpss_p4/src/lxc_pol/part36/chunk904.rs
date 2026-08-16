//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 904/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk904(t18738: f64, t2782: f64, t18677: f64, t231: f64, t2783: f64, t18681: f64, t2723: f64, t4503: f64, t6041: f64, t72: f64, t686: f64, t874: f64) -> (f64, f64, f64, f64, f64) {
    let t18739 = t2782 * t18738;
    let t18742 = t2783 * t18677 * t231;
    let t18743 = t2782 * t18742;
    let t18746 = t2783 * t18681 * t231;
    let t18747 = t2782 * t18746;
    let t18750 = t4503 * t18677 * t2723;
    let t18751 = t2782 * t18750;
    let t18761 = t6041 * t72;
    let t18763 = t874 * t18761 * t686;
    (t18739, t18743, t18747, t18751, t18763)
}
