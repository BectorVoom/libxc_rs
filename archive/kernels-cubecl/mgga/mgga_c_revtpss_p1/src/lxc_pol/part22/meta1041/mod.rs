//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1041 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3634;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1041<F: Float>(t12227: F, t12230: F, t3385: F, t6470: F, t12243: F, t20648: F, t16942: F, t3433: F, t5108: F, t16812: F, t5192: F, t1196: F, t3516: F, t6555: F, t45046: F, t6474: F, t3383: F, t6433: F, t3386: F, t5180: F, t1188: F, t3495: F, t16811: F, t43752: F, t6518: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t68779, t68781, t68784, t68786, t68789) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3634::<F>(t12227, t12230, t3385, t6470, t12243, t20648, t16942, t3433, t5108, t16812, t5192, t1196, t3516, t6555);
        let (t68791, t68794, t68795, t68799, t68803) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3635::<F>(t45046, t6474, t3383, t6433, t3386, t5180, t1188, t1196, t3495, t16811, t43752, t6518);
    (t68779, t68781, t68784, t68786, t68789, t68791, t68794, t68795, t68799, t68803)
}
