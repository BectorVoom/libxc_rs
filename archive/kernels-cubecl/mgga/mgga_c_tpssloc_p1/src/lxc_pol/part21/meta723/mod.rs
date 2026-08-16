//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta723 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta723<F: Float>(t14783: F, t699: F, t14786: F, t14789: F, t14778: F, t11153: F, t1229: F, t3242: F, t486: F, t11147: F, t3584: F, t2403: F, t4775: F) -> (F, F, F, F, F, F, F, F) {
        let (t50968, t50970, t50972, t50978, t50992, t50998, t51002, t51039) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2578::<F>(t14783, t699, t14786, t14789, t14778, t11153, t1229, t3242, t486, t11147, t3584, t2403, t4775);
    (t50968, t50970, t50972, t50978, t50992, t50998, t51002, t51039)
}
