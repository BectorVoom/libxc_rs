//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk931;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk932;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk933;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta145(t3363: f64, t3417: f64, t141: f64, t1145: f64, t3368: f64, t3372: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t3392: f64, t3400: f64, t3402: f64, t3408: f64, t3410: f64, t3414: f64, t3415: f64, t1150: f64, t1131: f64, t1129: f64, t408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3418, t3419, t3421, t3422, t3424, t3425, t3427) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk931(t3363, t3417, t141, t1145, t3368, t3372, t3358, t3365, t3370, t3374, t3392, t3400, t3402, t3408, t3410, t3414, t3415);
        let (t3428, t3430, t3431) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk932(t1150, t3427, t1131, t1129);
        let t3432 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk933(t3431);
        let t3433 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk934(t3432, t408);
    (t3418, t3419, t3421, t3422, t3424, t3425, t3427, t3428, t3430, t3431, t3432, t3433)
}
