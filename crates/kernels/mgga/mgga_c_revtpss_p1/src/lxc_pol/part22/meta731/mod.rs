//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2788;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta731<F: Float>(t10111: F, t823: F, t9720: F, t685: F, t827: F, t837: F, t10837: F, t9775: F, t2237: F, t2482: F, t2487: F, t849: F, t775: F, t855: F, t242: F, t240: F, t72: F, t10710: F, t10733: F, t10716: F, t10741: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40406, t40409, t40411, t40424, t40425, t40452) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2788::<F>(t10111, t823, t9720, t685, t827, t837, t10837, t9775, t2237, t2482, t2487, t849);
        let (t40455, t40462, t40473, t40475, t40477) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2789::<F>(t40452, t685, t775, t855, t242, t240, t72, t10710, t9775, t10733, t10716, t10741);
    (t40406, t40409, t40411, t40424, t40425, t40452, t40455, t40462, t40473, t40475, t40477)
}
