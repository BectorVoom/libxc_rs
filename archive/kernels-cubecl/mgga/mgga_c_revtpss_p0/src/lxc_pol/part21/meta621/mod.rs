//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2378;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2379;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta621<F: Float>(t2237: F, t2482: F, t823: F, t2487: F, t2646: F, t2661: F, t2662: F, t2663: F, t10777: F, t10780: F, t14686: F, t10803: F, t10811: F, t10111: F, t849: F, t9720: F, t685: F, t775: F, t855: F, t242: F, t240: F, t72: F, t10700: F, t2652: F, t10710: F, t9775: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40424, t40425, t40429, t40438, t40440) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2378::<F>(t2237, t2482, t823, t2487, t2646, t2661, t2662, t2663, t10777, t10780, t14686, t10803, t10811);
        let (t40452, t40455, t40462, t40471, t40473) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2379::<F>(t10111, t849, t9720, t685, t775, t855, t242, t240, t72, t10700, t2652, t10710, t9775);
    (t40424, t40425, t40429, t40438, t40440, t40452, t40455, t40462, t40471, t40473)
}
