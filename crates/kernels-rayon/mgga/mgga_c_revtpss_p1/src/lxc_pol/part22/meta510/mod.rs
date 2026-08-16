//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2256;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2257;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2258;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2259;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2260;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta510(t16710: f64, t5057: f64, t689: f64, t12256: f64, t1469: f64, t2251: f64, t12305: f64, t128: f64, t12268: f64, t3360: f64, t3362: f64, t4186: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16711, t16712) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2256(t16710, t5057, t689);
        let (t16713, t16714, t16715) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2257(t16712, t12256, t1469, t2251);
        let (t16716, t16717) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2258(t12305, t16715, t128);
        let (t16719, t16720) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2259(t12268, t1469, t2251);
        let (t16721, t16722) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2260(t16720, t3360, t128);
        let t16725 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2261(t3362, t4186, t606);
    (t16711, t16712, t16713, t16714, t16715, t16716, t16717, t16719, t16720, t16721, t16722, t16725)
}
