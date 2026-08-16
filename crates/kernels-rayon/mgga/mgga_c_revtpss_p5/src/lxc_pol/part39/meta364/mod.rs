//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1271;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1272;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1273;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta364(t15191: f64, t13312: f64, t905: f64, t904: f64, t128: f64, t4628: f64, t698: f64, t930: f64, t141: f64, t15127: f64, t15125: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11304: f64, t15132: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15189: f64, t923: f64, t916: f64, t11339: f64, t11366: f64, t11368: f64, t11479: f64, t11480: f64, t11326: f64, t15108: f64, t15111: f64, t15114: f64, t15116: f64, t15119: f64, t15121: f64, t15123: f64, t15128: f64, t15175: f64, t15178: f64, t15181: f64, t15184: f64, t15187: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15192, t15193, t15195) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1271(t15191, t13312, t905, t904, t128);
        let (t15197, t15198, t15200, t15220) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1272(t4628, t698, t15193, t930, t141, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11304, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15221, t15230, t15232) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1273(t15220, t923, t916, t11134, t11136, t11138, t11140, t11339, t11366, t11368, t11479, t11480);
        let t15234 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1274(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15128, t15132, t15175, t15178, t15181, t15184, t15187, t15189, t15192, t15195, t15198, t15200, t15232);
    (t15193, t15195, t15197, t15200, t15221, t15230, t15234)
}
