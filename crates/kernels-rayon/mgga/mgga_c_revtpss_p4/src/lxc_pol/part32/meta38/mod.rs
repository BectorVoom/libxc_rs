//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta38 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk242;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk243;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk244;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk245;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk246;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk247;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta38(t729: f64, t730: f64, t182: f64, t177: f64, t687: f64, t689: f64, t693: f64, t698: f64, t185: f64, t123: f64, t173: f64, t186: f64, t676: f64, t679: f64, t704: f64, t724: f64, t162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t731, t737, t738) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk242(t729, t730, t182);
        let (t739, t744) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk243(t177, t738, t687, t689, t693, t698);
        let t745 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk244(t185);
        let t746 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk245(t744, t745);
        let t749 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk246(t123, t173, t186, t676, t679, t704, t724, t731, t739, t746);
        let t750 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk247(t162, t749);
    (t731, t737, t738, t739, t744, t745, t746, t749, t750)
}
