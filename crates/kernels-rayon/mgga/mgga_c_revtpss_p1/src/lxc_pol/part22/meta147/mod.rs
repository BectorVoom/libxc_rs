//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta147 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk972;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk973;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk974;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk975;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk976;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta147(t3453: f64, t3479: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t448: f64, t1175: f64, t1179: f64, t1178: f64, t444: f64, t439: f64, t1187: f64, t1188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3480, t3483, t3488) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk972(t3453, t3479, t3356, t3358, t3365, t3370, t3374);
        let (t3489, t3491) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk973(t3488, t448, t1175, t1179);
        let (t3494, t3495) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk974(t1178, t444);
        let t3496 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk975(t3495, t439);
        let t3497 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk976(t1187);
        let t3498 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk977(t1188, t3497);
    (t3480, t3483, t3488, t3489, t3491, t3494, t3495, t3496, t3497, t3498)
}
