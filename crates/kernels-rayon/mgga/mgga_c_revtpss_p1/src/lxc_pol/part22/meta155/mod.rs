//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta155 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1034;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1035;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1036;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1037;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1038;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta155(t3584: f64, t482: f64, t371: f64, t372: f64, t225: f64, t3555: f64, t480: f64, t3566: f64, t3568: f64, t1236: f64, t127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3661, t3663, t3666) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1034(t3584, t482, t371, t372, t225, t3555);
        let t3667 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1035(t3666, t480);
        let t3670 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1036(t225, t3566);
        let t3671 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1037(t3670, t480);
        let (t3672, t3674) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1038(t3568, t482, t371, t372);
        let t3678 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1039(t1236, t127, t371);
    (t3661, t3663, t3666, t3667, t3670, t3671, t3672, t3674, t3678)
}
