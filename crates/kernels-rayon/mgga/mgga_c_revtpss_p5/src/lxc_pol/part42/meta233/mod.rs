//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk898;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk899;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta233(t1045: f64, t6271: f64, t3117: f64, t373: f64, t6258: f64, t371: f64, t372: f64, t3236: f64, t5819: f64, t1012: f64, t1015: f64, t5825: f64, t3253: f64, t1011: f64, t1025: f64, t1665: f64, t3082: f64, t3091: f64, t3115: f64, t3127: f64, t4792: f64, t4818: f64, t4821: f64, t4858: f64, t6263: f64, t6268: f64, t6106: f64, t6108: f64, t6112: f64, t6144: f64, t6147: f64, t6213: f64, t6215: f64, t6217: f64, t6221: f64, t6225: f64, t6229: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6272, t6273, t6276, t6278, t6284, t6285, t6288) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk898(t1045, t6271, t3117, t373, t6258, t371, t372, t3236, t5819, t1012, t1015, t5825);
        let (t6289, t6292, t6293, t6298) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk899(t1012, t6288, t3253, t5819, t1011, t1025, t1665, t3082, t3091, t3115, t3127, t4792, t4818, t4821, t4858, t6263, t6268, t6273, t6278, t6285);
        let t6299 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk900(t6106, t6108, t6112, t6144, t6147, t6213, t6215, t6217, t6221, t6225, t6229);
    (t6272, t6273, t6276, t6278, t6284, t6285, t6288, t6289, t6292, t6293, t6298, t6299)
}
