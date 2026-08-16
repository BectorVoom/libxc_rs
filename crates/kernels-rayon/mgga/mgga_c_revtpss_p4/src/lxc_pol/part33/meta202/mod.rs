//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk939;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk940;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk941;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta202(t1214: f64, t471: f64, t5351: f64, t3720: f64, t140: f64, t1781: f64, t1222: f64, t127: f64, t1789: f64, t371: f64, t1235: f64, t1219: f64, t1778: f64, t1225: f64, t4186: f64, t1012: f64, t3657: f64, t3658: f64, t3679: f64, t3684: f64, t3718: f64, t5340: f64, t5343: f64, t5348: f64, t1010: f64, t1480: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5352, t5353, t5354) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk939(t1214, t471, t5351, t3720);
        let (t5357, t5358, t5362) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk940(t140, t1781, t1222, t127, t1789, t371);
        let (t5363, t5366, t5368, t5369, t5372) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk941(t1235, t5362, t1219, t1778, t1225, t4186, t1012, t1222, t3657, t3658, t3679, t3684, t3718, t5340, t5343, t5348, t5354, t5358);
        let t5373 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk942(t1010, t1480);
    (t5352, t5353, t5354, t5357, t5358, t5362, t5363, t5366, t5368, t5369, t5372, t5373)
}
