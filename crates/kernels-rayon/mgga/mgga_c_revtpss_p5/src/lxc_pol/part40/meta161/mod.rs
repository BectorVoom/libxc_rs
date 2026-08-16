//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk727;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk728;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk729;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta161(t1204: f64, t487: f64, t1207: f64, t458: f64, t456: f64, t1214: f64, t1211: f64, t1209: f64, t1269: f64, t1294: f64, t1277: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3561, t3565, t3566) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk727(t1204, t487, t1207, t458, t456);
        let (t3567, t3568) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk728(t3566, t487, t1214);
        let (t3569, t3572) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk729(t1211, t3568, t1209, t1269);
        let (t3575, t3576, t3579, t3584) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk730(t1214, t1294, t1277, t3356, t3358, t3365, t3370, t3374);
    (t3561, t3565, t3566, t3567, t3568, t3569, t3572, t3575, t3576, t3579, t3584)
}
