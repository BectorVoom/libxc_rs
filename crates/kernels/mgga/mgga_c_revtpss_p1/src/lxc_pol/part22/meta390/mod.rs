//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1959;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta390<F: Float>(t1903: F, t4131: F, t4076: F, t4077: F, t9657: F, t1444: F, t5774: F, t10171: F, t13727: F, t13733: F, t13737: F, t1424: F, t1904: F, t9632: F, t9636: F, t9639: F, t9642: F, t9650: F, t13716: F, t1414: F, t828: F, t221: F, t3979: F, t5591: F, t3978: F, t3989: F, t5614: F, t5622: F, t9765: F, t1408: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13738, t13739, t13743, t13746, t13747, t13750) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1959::<F>(t1903, t4131, t4076, t4077, t9657, t1444, t5774, t10171, t13727, t13733, t13737, t1424, t1904, t9632, t9636, t9639, t9642, t9650);
        let (t13756, t13760, t13762, t13763, t13765, t13767) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1960::<F>(t13716, t1414, t828, t221, t3979, t5591, t3978, t3989, t5614, t5622, t9765, t1408, t240);
    (t13738, t13739, t13743, t13746, t13747, t13750, t13756, t13760, t13762, t13763, t13765, t13767)
}
