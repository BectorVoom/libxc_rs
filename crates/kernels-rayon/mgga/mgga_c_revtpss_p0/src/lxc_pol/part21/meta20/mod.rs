//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta20 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk159;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk160;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk161;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk162;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk163;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk164;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk165;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta20(t371: f64, t372: f64, t373: f64, t345: f64, t348: f64, t367: f64, t225: f64, t359: f64, t342: f64, t198: f64, t293: f64, t328: f64, t330: f64, t336: f64, t265: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t375 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk159(t371, t372, t373);
        let t378 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk160(t345, t348, t367, t375);
        let t379 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk161(t225, t378);
        let t380 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk162(t225, t359);
        let t381 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk163(t378, t380);
        let (t384, t385) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk164(t342, t381);
        let t386 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk165(t379, t385);
        let (t389, t395, t393) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk166(t342, t386, t198, t293, t328, t330, t336, t265);
    (t375, t378, t379, t380, t381, t384, t385, t386, t389, t395, t393)
}
