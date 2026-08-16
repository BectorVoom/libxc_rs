//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk890;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta227<F: Float>(t1132: F, t5079: F, t1723: F, t3407: F, t1134: F, t1139: F, t1729: F, t698: F, t3417: F, t5047: F, t141: F, t1145: F, t5052: F, t5056: F, t3358: F, t3402: F, t3414: F, t3415: F, t5044: F, t5049: F, t5054: F, t5058: F, t5072: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5080, t5087, t5088, t5090, t5093, t5095, t5096, t5098) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk890::<F>(t1132, t5079, t1723, t3407, t1134, t1139, t1729, t698, t3417, t5047, t141, t1145, t5052);
        let (t5099, t5101, t5102, t5104) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk891::<F>(t141, t5098, t1145, t5056, t3358, t3402, t3414, t3415, t5044, t5049, t5054, t5058, t5072, t5080, t5088, t5090, t5093, t5096);
    (t5080, t5087, t5088, t5090, t5093, t5095, t5096, t5098, t5099, t5101, t5102, t5104)
}
