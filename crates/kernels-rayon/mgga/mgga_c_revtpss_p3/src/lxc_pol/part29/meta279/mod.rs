//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1149;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1150;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1151;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta279(t128: f64, t121: f64, t22: f64, t2508: f64, t9285: f64, t692: f64, t9288: f64, t124: f64, t624: f64, t138: f64, t9283: f64, t9286: f64, t9289: f64, t9292: f64, t701: f64, t682: f64, t2580: f64, t680: f64, t130: f64, t146: f64, t2583: f64, t9275: f64, t2514: f64, t2596: f64, t746: f64, t1340: f64, t2491: f64, t2495: f64, t744: f64, t215: f64, t681: f64, t268: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9296, t9298, t9300, t9302, t9303) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1149(t128, t121, t22, t2508, t9285, t692, t9288, t124, t624, t138);
        let t9308 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1150(t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303, t701, t682);
        let t9316 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1151(t2580, t680, t130, t146, t2583, t9275);
        let (t9318, t9320, t9323, t9325, t9329) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1152(t2514, t2596, t746, t1340, t2491, t2495, t744, t215, t681, t268, t702);
    (t9296, t9298, t9300, t9302, t9303, t9308, t9316, t9318, t9320, t9323, t9325, t9329)
}
