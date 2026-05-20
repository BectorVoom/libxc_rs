//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1271;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1272;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1273;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta364<F: Float>(t15191: F, t13312: F, t905: F, t904: F, t128: F, t4628: F, t698: F, t930: F, t141: F, t15127: F, t15125: F, t11134: F, t11136: F, t11138: F, t11140: F, t11304: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t923: F, t916: F, t11339: F, t11366: F, t11368: F, t11479: F, t11480: F, t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15128: F, t15175: F, t15178: F, t15181: F, t15184: F, t15187: F) -> (F, F, F, F, F, F, F) {
        let (t15192, t15193, t15195) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1271::<F>(t15191, t13312, t905, t904, t128);
        let (t15197, t15198, t15200, t15220) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1272::<F>(t4628, t698, t15193, t930, t141, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11304, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15221, t15230, t15232) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1273::<F>(t15220, t923, t916, t11134, t11136, t11138, t11140, t11339, t11366, t11368, t11479, t11480);
        let t15234 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1274::<F>(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15128, t15132, t15175, t15178, t15181, t15184, t15187, t15189, t15192, t15195, t15198, t15200, t15232);
    (t15193, t15195, t15197, t15200, t15221, t15230, t15234)
}
