//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2314;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2315;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2316;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2317;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta529(t16750: f64, t482: f64, t371: f64, t372: f64, t1803: f64, t3666: f64, t1208: f64, t5215: f64, t225: f64, t480: f64, t3678: f64, t5327: f64, t5323: f64, t1235: f64, t1238: f64, t12800: f64, t12976: f64, t1791: f64, t1808: f64, t3644: f64, t3663: f64, t3667: f64, t5320: f64, t5391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17278, t17280, t17283) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2314(t16750, t482, t371, t372, t1803, t3666);
        let t17288 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2315(t1208, t5215);
        let t17289 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2316(t17288, t225);
        let t17290 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2317(t17289, t480);
        let (t17296, t17298, t17299) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2318(t3678, t5327, t5323, t1235, t1238, t12800, t12976, t17280, t17283, t17290, t1791, t1808, t3644, t3663, t3667, t5320, t5391);
    (t17278, t17280, t17283, t17288, t17289, t17290, t17296, t17298, t17299)
}
