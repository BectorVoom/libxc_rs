//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1427/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1427<F: Float>(t1063: F, t1592: F, t247: F, t42778: F, t3298: F, t4746: F, t4891: F, t225: F, t53014: F, t366: F, t1011: F, t1655: F, t2438: F) -> (F, F, F, F, F) {
    let t53762 = t1063 * t247 * t42778 * t1592;
    let t53800 = t4746 * t3298 * t4891;
    let t53877 = t53014 * t225;
    let t53878 = t53877 * t366;
    let t54118 = t1011 * t2438 * t1655;
    (t53762, t53800, t53877, t53878, t54118)
}
