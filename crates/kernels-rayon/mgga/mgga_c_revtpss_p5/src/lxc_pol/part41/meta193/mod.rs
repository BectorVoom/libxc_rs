//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk781;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk782;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk783;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk784;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta193(t1187: f64, t3523: f64, t5205: f64, t1196: f64, t3358: f64, t3546: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t459: f64, t1208: f64, t1769: f64, t487: f64, t1770: f64, t1214: f64, t1774: f64, t1211: f64, t1294: f64, t1277: f64, t3579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5206, t5207, t5209, t5215, t5216) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk781(t1187, t3523, t5205, t1196, t3358, t3546, t5044, t5049, t5054, t5058, t459);
        let t5219 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk782(t1208, t1769);
        let t5220 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk783(t487, t5219);
        let (t5225, t5230) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk784(t1770, t487, t1214, t1774);
        let (t5231, t5237, t5245) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk785(t1211, t5230, t1294, t1774, t1277, t3358, t3579, t5044, t5049, t5054, t5058);
    (t5206, t5207, t5209, t5215, t5216, t5219, t5220, t5225, t5230, t5231, t5237, t5245)
}
