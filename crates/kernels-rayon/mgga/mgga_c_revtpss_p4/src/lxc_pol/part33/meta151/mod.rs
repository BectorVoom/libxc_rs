//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta151 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk780;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk781;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk782;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk783;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk784;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta151(t1398: f64, t543: f64, t550: f64, t3992: f64, t2661: f64, t1384: f64, t544: f64, t235: f64, t239: f64, t820: f64, t531: f64, t549: f64, t240: f64, t72: f64, t1386: f64, t2482: f64, t27: f64, t136: f64, t1389: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3994 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk780(t1398, t543, t550);
        let (t3995, t3996, t3999) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk781(t3992, t3994, t2661, t1384, t544);
        let t4000 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk782(t235, t3999);
        let (t4002, t4003) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk783(t239, t4000, t820, t543);
        let (t4010, t4011, t4012, t4018) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk784(t531, t549, t240, t72, t1386, t2482, t27);
        let t4019 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk785(t136, t1389);
    (t3994, t3995, t3996, t3999, t4000, t4002, t4003, t4010, t4011, t4012, t4018, t4019)
}
