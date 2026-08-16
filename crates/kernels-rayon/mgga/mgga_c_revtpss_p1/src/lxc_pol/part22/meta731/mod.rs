//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2788;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta731(t10111: f64, t823: f64, t9720: f64, t685: f64, t827: f64, t837: f64, t10837: f64, t9775: f64, t2237: f64, t2482: f64, t2487: f64, t849: f64, t775: f64, t855: f64, t242: f64, t240: f64, t72: f64, t10710: f64, t10733: f64, t10716: f64, t10741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40406, t40409, t40411, t40424, t40425, t40452) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2788(t10111, t823, t9720, t685, t827, t837, t10837, t9775, t2237, t2482, t2487, t849);
        let (t40455, t40462, t40473, t40475, t40477) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2789(t40452, t685, t775, t855, t242, t240, t72, t10710, t9775, t10733, t10716, t10741);
    (t40406, t40409, t40411, t40424, t40425, t40452, t40455, t40462, t40473, t40475, t40477)
}
