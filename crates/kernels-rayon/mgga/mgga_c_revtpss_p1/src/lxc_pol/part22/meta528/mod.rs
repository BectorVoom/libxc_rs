//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2311;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2312;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta528(t17240: f64, t5052: f64, t1222: f64, t16738: f64, t5308: f64, t16742: f64, t16733: f64, t16771: f64, t247: f64, t3719: f64, t3636: f64, t5391: f64, t5381: f64, t1260: f64, t12966: f64, t16775: f64, t1261: f64, t17232: f64, t17237: f64, t5384: f64, t5386: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17241, t17243, t17244, t17247, t17250, t17254, t17258) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2311(t17240, t5052, t1222, t16738, t5308, t16742, t16733, t16771, t247, t3719, t3636, t5391);
        let (t17260, t17261) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2312(t3636, t5381, t1260, t12966);
        let (t17265, t17268) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2313(t16775, t247, t3719, t1222, t1261, t17232, t17237, t17243, t17244, t17247, t17250, t17254, t17258, t17260, t17261, t5384, t5386);
    (t17241, t17243, t17244, t17247, t17250, t17254, t17258, t17260, t17261, t17265, t17268)
}
