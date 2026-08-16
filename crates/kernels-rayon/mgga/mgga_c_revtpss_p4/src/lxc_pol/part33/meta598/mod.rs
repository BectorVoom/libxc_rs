//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2019;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta598(t25953: f64, t26072: f64, t2435: f64, t25913: f64, t7289: f64, t94600: f64, t2028: f64, t3999: f64, t25875: f64, t25894: f64, t25877: f64, t94382: f64, t94590: f64, t25304: f64, t25949: f64, t25946: f64, t25878: f64, t94661: f64, t7246: f64, t9692: f64, t1419: f64, t7063: f64, t25898: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94756, t94758, t94761, t94763, t94768, t94771) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2019(t25953, t26072, t2435, t25913, t7289, t94600, t2028, t3999, t25875, t25894, t25877, t94382);
        let (t94772, t94777, t94779, t94784, t94801, t94802) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2020(t94590, t94771, t25304, t25949, t25946, t25878, t94661, t7246, t9692, t1419, t7063, t25898);
    (t94756, t94758, t94761, t94763, t94768, t94771, t94772, t94777, t94779, t94784, t94801, t94802)
}
