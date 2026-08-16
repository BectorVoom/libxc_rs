//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta22 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk160;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk161;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk162;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta22(t281: f64, t282: f64, t414: f64, t406: f64, t409: f64, t412: f64, t408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t416, t418, t421, t422) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk160(t281, t282, t414, t406, t409, t412);
        let (t424, t426) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk161(t408, t422, t406);
        let (t431, t434, t435) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk162(t406, t409, t412, t416);
        let t439 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk163(t406);
    (t416, t418, t421, t422, t424, t426, t431, t434, t435, t439)
}
