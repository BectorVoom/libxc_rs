//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1813;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1814;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1815;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1816;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta478(t3215: f64, t7117: f64, t1972: f64, t3204: f64, t3143: f64, t3148: f64, t3141: f64, t7120: f64, t3123: f64, t7121: f64, sigma0: f64, t365: f64, t3089: f64, t1087: f64, t1024: f64, t7131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25498, t25500, t25504, t25505, t25508, t25509, t25512) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1813(t3215, t7117, t1972, t3204, t3143, t3148, t3141, t7120, t3123, t7121, sigma0);
        let t25515 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1814(t365, sigma0);
        let t25516 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1815(t25515, t3089);
        let t25517 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1816(t1087, t25516);
        let t25522 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1817(t1024, t7131);
    (t25498, t25500, t25504, t25505, t25508, t25509, t25512, t25515, t25516, t25517, t25522)
}
