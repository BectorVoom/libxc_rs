//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk898;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk899;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta233<F: Float>(t1045: F, t6271: F, t3117: F, t373: F, t6258: F, t371: F, t372: F, t3236: F, t5819: F, t1012: F, t1015: F, t5825: F, t3253: F, t1011: F, t1025: F, t1665: F, t3082: F, t3091: F, t3115: F, t3127: F, t4792: F, t4818: F, t4821: F, t4858: F, t6263: F, t6268: F, t6106: F, t6108: F, t6112: F, t6144: F, t6147: F, t6213: F, t6215: F, t6217: F, t6221: F, t6225: F, t6229: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6272, t6273, t6276, t6278, t6284, t6285, t6288) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk898::<F>(t1045, t6271, t3117, t373, t6258, t371, t372, t3236, t5819, t1012, t1015, t5825);
        let (t6289, t6292, t6293, t6298) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk899::<F>(t1012, t6288, t3253, t5819, t1011, t1025, t1665, t3082, t3091, t3115, t3127, t4792, t4818, t4821, t4858, t6263, t6268, t6273, t6278, t6285);
        let t6299 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk900::<F>(t6106, t6108, t6112, t6144, t6147, t6213, t6215, t6217, t6221, t6225, t6229);
    (t6272, t6273, t6276, t6278, t6284, t6285, t6288, t6289, t6292, t6293, t6298, t6299)
}
