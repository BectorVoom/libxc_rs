//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1704;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta405<F: Float>(t11219: F, t18206: F, t136: F, t18211: F, t3297: F, t18215: F, t6014: F, t699: F, t1113: F, t18221: F, t18225: F, t6017: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18496, t18497, t18499, t18500, t18502, t18503, t18505) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1704::<F>(t11219, t18206, t136, t18211, t3297, t18215, t6014, t699);
        let (t18507, t18508, t18509, t18510, t18512) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1705::<F>(t1113, t18221, t136, t18225, t6017, t699);
    (t18496, t18497, t18499, t18500, t18502, t18503, t18505, t18507, t18508, t18509, t18510, t18512)
}
