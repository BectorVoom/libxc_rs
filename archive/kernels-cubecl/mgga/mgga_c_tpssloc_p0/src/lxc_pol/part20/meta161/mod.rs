//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1020;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1021;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta161<F: Float>(t2225: F, t522: F, t2221: F, t2223: F, t2516: F, t521: F, t17: F, t1284: F, t750: F) -> (F, F, F, F, F, F) {
        let (t3819, t3821, t3823, t3824) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1020::<F>(t2225, t522, t2221, t2223, t2516, t521);
        let (t3825, t3826) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1021::<F>(t17, t3824, t1284, t750);
    (t3819, t3821, t3823, t3824, t3825, t3826)
}
