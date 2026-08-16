//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2421;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2422;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta571<F: Float>(t124: F, t18392: F, t800: F, t828: F, t855: F, t221: F, t2675: F, t5962: F, t2674: F, t10756: F, t10758: F, t10762: F, t14836: F, t14837: F, t14839: F, t14846: F, t14850: F, t14859: F, t14864: F, t799: F, t851: F, t243: F, t6016: F, t231: F, t2662: F, t2661: F, t5977: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18393, t18394, t18398, t18402, t18405) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2421::<F>(t124, t18392, t800, t828, t855, t221, t2675, t5962, t2674, t10756, t10758, t10762, t14836, t14837, t14839, t14846, t14850, t14859, t14864, t799, t851);
        let (t18408, t18409, t18410, t18411, t18413) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2422::<F>(t243, t6016, t231, t2662, t2661, t5977);
    (t18393, t18394, t18398, t18402, t18405, t18408, t18409, t18410, t18411, t18413)
}
