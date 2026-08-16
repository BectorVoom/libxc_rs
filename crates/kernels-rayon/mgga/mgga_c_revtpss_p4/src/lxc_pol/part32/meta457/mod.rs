//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1662;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1663;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1664;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1665;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1666;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1667;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta457(t25374: f64, t25386: f64, t2769: f64, t7056: f64, t1955: f64, t233: f64, t867: f64, t1957: f64, t822: f64, t676: f64, t837: f64, t2718: f64, t25372: f64, t2411: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t25387 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1662(t25374, t25386);
        let (t25390, t25391) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1663(t2769, t7056, t1955);
        let (t25402, t25410) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1664(t233, t867, t1957, t822);
        let t25411 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1665(t25386, t25410);
        let (t25412, t25416, t25431) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1666(t676, t837, t2718, t867, t25372, t25410);
        let t25759 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1667(t2411, t33);
    (t25387, t25390, t25391, t25402, t25410, t25411, t25412, t25416, t25431, t25759)
}
