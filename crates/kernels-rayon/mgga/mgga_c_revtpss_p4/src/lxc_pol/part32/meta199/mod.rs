//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta199 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk878;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk879;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk880;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk881;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk882;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta199(t5047: f64, t5312: f64, t482: f64, t5245: f64, t371: f64, t372: f64, t1234: f64, t1803: f64, t225: f64, t5219: f64, t480: f64, t3623: f64, t4890: f64, t3782: f64, t1794: f64, t3153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5313, t5318, t5320, t5323) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk878(t5047, t5312, t482, t5245, t371, t372, t1234, t1803);
        let t5326 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk879(t225, t5219);
        let t5327 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk880(t480, t5326);
        let t5330 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk881(t3623, t4890);
        let t5331 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk882(t3782, t5330);
        let t5332 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk883(t1794, t3153);
    (t5313, t5318, t5320, t5323, t5326, t5327, t5330, t5331, t5332)
}
