//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2100/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2100(t17617: f64, t26870: f64, t3682: f64, t8172: f64, t29020: f64, t3704: f64, t29086: f64, t3678: f64, t3655: f64, t8185: f64, t17628: f64, t7607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104953 = 0.57165357490759649296e-3_f64 * t26870 * t17617;
    let t104963 = t8172 * t3682;
    let t104968 = 0.30488190661738479624e-2_f64 * t29020 * t3704;
    let t104972 = 0.57165357490759649296e-3_f64 * t29086 * t3678;
    let t104988 = t8185 * t3655;
    let t104990 = t7607 * t17628;
    (t104953, t104963, t104968, t104972, t104988, t104990)
}
