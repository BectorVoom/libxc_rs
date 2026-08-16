//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1041 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3634;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1041(t12227: f64, t12230: f64, t3385: f64, t6470: f64, t12243: f64, t20648: f64, t16942: f64, t3433: f64, t5108: f64, t16812: f64, t5192: f64, t1196: f64, t3516: f64, t6555: f64, t45046: f64, t6474: f64, t3383: f64, t6433: f64, t3386: f64, t5180: f64, t1188: f64, t3495: f64, t16811: f64, t43752: f64, t6518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68779, t68781, t68784, t68786, t68789) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3634(t12227, t12230, t3385, t6470, t12243, t20648, t16942, t3433, t5108, t16812, t5192, t1196, t3516, t6555);
        let (t68791, t68794, t68795, t68799, t68803) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3635(t45046, t6474, t3383, t6433, t3386, t5180, t1188, t1196, t3495, t16811, t43752, t6518);
    (t68779, t68781, t68784, t68786, t68789, t68791, t68794, t68795, t68799, t68803)
}
