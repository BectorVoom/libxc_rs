//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta125 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk816;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk817;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk818;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk819;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk820;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta125(t3356: f64, t1156: f64, t1160: f64, t1159: f64, t431: f64, t426: f64, t3413: f64, t434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3439, t3447, t3450, t3451) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk816(t3356, t1156, t1160, t1159, t431);
        let t3452 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk817(t3451, t426);
        let (t3459, t3466, t3475) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk818(t3356, t3413, t1159);
        let t3476 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk819(t3475);
        let t3477 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk820(t3476, t426);
        let (t3478, t3479) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk821(t434);
    (t3439, t3447, t3450, t3451, t3452, t3459, t3466, t3475, t3476, t3477, t3478, t3479)
}
