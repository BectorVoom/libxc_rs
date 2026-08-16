//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3644/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3644(t1131: f64, t1150: f64, t68821: f64, t68837: f64, t68854: f64, t68870: f64, t68887: f64, t68903: f64, t68920: f64, t68936: f64, t68779: f64, t68781: f64, t68784: f64, t68786: f64, t68789: f64, t68791: f64, t68794: f64, t68799: f64, t68803: f64, t68805: f64, t68808: f64) -> (f64, f64) {
    let t68942 = 1.0_f64 * t1131 * (t68821 + t68837 + t68854 + t68870 + t68887 + t68903 + t68920 + t68936) * t1150;
    let t68943 = t68779 + t68781 + t68784 - t68786 - t68789 + t68791 - t68794 + t68799 + t68803 + t68805 + t68808 + t68942;
    (t68942, t68943)
}
