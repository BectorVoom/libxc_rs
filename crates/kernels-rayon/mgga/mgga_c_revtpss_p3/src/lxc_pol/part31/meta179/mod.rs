//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk862;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk863;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk864;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta179(t4772: f64, t996: f64, t1678: f64, t994: f64, t1668: f64, t73: f64, t3095: f64, t3092: f64, t3093: f64, t357: f64, t1592: f64, t1058: f64, t1660: f64, t1053: f64, t1659: f64, t225: f64, t4743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4773 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk862(t4772, t996);
        let t4778 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk863(t1678, t994);
        let t4781 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk864(t1668, t73);
        let (t4782, t4783, t4786, t4787, t4788, t4792, t4794, t4797) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk865(t3095, t4781, t3092, t3093, t357, t1592, t1058, t1660, t1053, t1659, t225, t4743);
    (t4773, t4778, t4781, t4782, t4783, t4786, t4787, t4788, t4792, t4794, t4797)
}
