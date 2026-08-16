//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta141 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk945;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk946;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk947;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk948;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk949;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta141(t1120: f64, t3368: f64, t128: f64, t1121: f64, t2258: f64, t3357: f64, t3358: f64, t3365: f64, t422: f64, t1126: f64, t1130: f64, t1151: f64, t1129: f64, t418: f64, t408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3369, t3370) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk945(t1120, t3368, t128);
        let t3372 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk946(t1121, t2258);
        let (t3373, t3374) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk947(t1120, t3372, t128);
        let (t3376, t3378, t3379) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk948(t3357, t3358, t3365, t3370, t3374, t422, t1126, t1130);
        let (t3381, t3382, t3383) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk949(t1151, t3379, t1129, t418);
        let t3384 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk950(t3383, t408);
    (t3369, t3370, t3372, t3373, t3374, t3376, t3378, t3379, t3381, t3382, t3383, t3384)
}
