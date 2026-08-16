//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta21 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk167;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk168;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk169;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk170;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk171;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta21(t406: f64, t404: f64, t281: f64, t282: f64, t408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t409 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk167(t406);
        let (t412, t414) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk168(t406, t404);
        let (t416, t418, t421, t422) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk169(t281, t282, t414, t406, t409, t412);
        let (t424, t426) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk170(t408, t422, t406);
        let (t431, t434, t435) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk171(t406, t409, t412, t416);
    (t409, t412, t414, t416, t418, t421, t422, t424, t426, t431, t434, t435)
}
