//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2421;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2422;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta571(t124: f64, t18392: f64, t800: f64, t828: f64, t855: f64, t221: f64, t2675: f64, t5962: f64, t2674: f64, t10756: f64, t10758: f64, t10762: f64, t14836: f64, t14837: f64, t14839: f64, t14846: f64, t14850: f64, t14859: f64, t14864: f64, t799: f64, t851: f64, t243: f64, t6016: f64, t231: f64, t2662: f64, t2661: f64, t5977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18393, t18394, t18398, t18402, t18405) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2421(t124, t18392, t800, t828, t855, t221, t2675, t5962, t2674, t10756, t10758, t10762, t14836, t14837, t14839, t14846, t14850, t14859, t14864, t799, t851);
        let (t18408, t18409, t18410, t18411, t18413) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2422(t243, t6016, t231, t2662, t2661, t5977);
    (t18393, t18394, t18398, t18402, t18405, t18408, t18409, t18410, t18411, t18413)
}
