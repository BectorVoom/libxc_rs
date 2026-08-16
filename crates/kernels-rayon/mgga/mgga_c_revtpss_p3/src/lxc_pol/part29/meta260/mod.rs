//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1064;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1065;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1066;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta260(t665: f64, t6998: f64, t30: f64, t775: f64, t159: f64, t793: f64, t218: f64, t816: f64, t1941: f64, t228: f64, t802: f64, t240: f64, t64: f64, t234: f64, t243: f64, t807: f64, t1945: f64, t786: f64, t817: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6999, t7010, t7021) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1064(t665, t6998, t30, t775, t159, t793);
        let (t7023, t7025) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1065(t218, t7021, t816, t1941, t228);
        let (t7026, t7028) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1066(t7025, t802, t240, t64);
        let (t7030, t7031, t7033, t7034, t7036) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1067(t234, t243, t7028, t807, t1945, t786, t817, t64, t822);
    (t6999, t7010, t7021, t7023, t7025, t7026, t7028, t7030, t7031, t7033, t7034, t7036)
}
