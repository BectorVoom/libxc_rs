//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1518;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1519;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta266<F: Float>(t2573: F, t9573: F, t2690: F, t59: F, t154: F, t2588: F, t21: F, t207: F, t795: F, t225: F, t2711: F, t2594: F, t841: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9574, t9576, t9577) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1518::<F>(t2573, t9573, t2690, t59, t154);
        let (t9579, t9580, t9583, t9590, t9593, t9600, t9601) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1519::<F>(t2588, t9577, t21, t59, t207, t795, t225, t2711, t2594, t2690, t841, t812);
    (t9574, t9576, t9577, t9579, t9580, t9583, t9590, t9593, t9600, t9601)
}
