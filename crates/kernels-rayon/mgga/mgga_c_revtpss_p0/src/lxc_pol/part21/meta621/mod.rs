//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2378;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2379;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta621(t2237: f64, t2482: f64, t823: f64, t2487: f64, t2646: f64, t2661: f64, t2662: f64, t2663: f64, t10777: f64, t10780: f64, t14686: f64, t10803: f64, t10811: f64, t10111: f64, t849: f64, t9720: f64, t685: f64, t775: f64, t855: f64, t242: f64, t240: f64, t72: f64, t10700: f64, t2652: f64, t10710: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40424, t40425, t40429, t40438, t40440) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2378(t2237, t2482, t823, t2487, t2646, t2661, t2662, t2663, t10777, t10780, t14686, t10803, t10811);
        let (t40452, t40455, t40462, t40471, t40473) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2379(t10111, t849, t9720, t685, t775, t855, t242, t240, t72, t10700, t2652, t10710, t9775);
    (t40424, t40425, t40429, t40438, t40440, t40452, t40455, t40462, t40471, t40473)
}
