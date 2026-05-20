//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1426;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta457<F: Float>(t378: F, t53014: F, t11200: F, t1678: F, t11970: F, t1660: F, t127: F, t4823: F, t11239: F, t1647: F, t11245: F, t11255: F, t1063: F, t1592: F, t247: F, t42778: F, t3298: F, t4746: F, t4891: F, t225: F, t366: F, t1011: F, t1655: F, t2438: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53015, t53160, t53326, t53391, t53703, t53704, t53707) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1426::<F>(t378, t53014, t11200, t1678, t11970, t1660, t127, t4823, t11239, t1647, t11245, t11255);
        let (t53762, t53800, t53877, t53878, t54118) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1427::<F>(t1063, t1592, t247, t42778, t3298, t4746, t4891, t225, t53014, t366, t1011, t1655, t2438);
    (t53015, t53160, t53326, t53391, t53703, t53704, t53707, t53762, t53800, t53877, t53878, t54118)
}
