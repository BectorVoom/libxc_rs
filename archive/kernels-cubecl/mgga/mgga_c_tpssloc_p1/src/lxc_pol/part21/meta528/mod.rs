//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta528<F: Float>(t14164: F, t17686: F, t4582: F, t17691: F, t4583: F, t1023: F, t17670: F, t4594: F, t17167: F, t977: F, t17171: F, t17157: F, t2979: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17971, t17972, t17975, t17976, t17979, t17980, t17983, t17984, t17988, t17991, t17994) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2184::<F>(t14164, t17686, t4582, t17691, t4583, t1023, t17670, t4594, t17167, t977, t17171, t17157, t2979);
    (t17971, t17972, t17975, t17976, t17979, t17980, t17983, t17984, t17988, t17991, t17994)
}
