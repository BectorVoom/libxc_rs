//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1254;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1255;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1256;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta281(t684: f64, t9288: f64, t125: f64, t793: f64, t123: f64, t128: f64, t121: f64, t22: f64, t2508: f64, t9285: f64, t692: f64, t124: f64, t624: f64, t138: f64, t9283: f64, t9286: f64, t701: f64, t682: f64, t2580: f64, t680: f64, t130: f64, t146: f64, t2583: f64, t9275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9289, t9291, t9292) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1254(t684, t9288, t125, t793, t123);
        let (t9296, t9298, t9300, t9302, t9303) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1255(t128, t121, t22, t2508, t9285, t692, t9288, t124, t624, t138);
        let t9308 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1256(t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303, t701, t682);
        let t9316 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1257(t2580, t680, t130, t146, t2583, t9275);
    (t9289, t9291, t9292, t9296, t9298, t9300, t9302, t9303, t9308, t9316)
}
