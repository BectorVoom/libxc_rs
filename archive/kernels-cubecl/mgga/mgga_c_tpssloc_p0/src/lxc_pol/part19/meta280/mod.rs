//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta280<F: Float>(t12155: F, t12156: F, t1307: F, t1365: F, t3719: F, t12012: F, t1347: F, t12147: F, t1345: F, t1348: F, t3839: F, t3844: F, t3847: F, t5278: F, t546: F, t548: F) -> (F, F, F, F, F) {
        let (t12157, t12160, t12161, t12164, t12167) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1045::<F>(t12155, t12156, t1307, t1365, t3719, t12012, t1347, t12147, t1345, t1348, t3839, t3844, t3847, t5278, t546, t548);
    (t12157, t12160, t12161, t12164, t12167)
}
