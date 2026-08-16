//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1647;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta383<F: Float>(t14164: F, t17686: F, t4582: F, t17691: F, t4583: F, t1023: F, t17670: F, t4594: F, t17167: F, t977: F, t17171: F, t17157: F, t2979: F, t5677: F, t10408: F, t1036: F, t5905: F, t1041: F, t10876: F, t10883: F, t10952: F, t13995: F, t14158: F, t14160: F, t3070: F, t3109: F, t4579: F, t5869: F, t5880: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17971, t17972, t17975, t17976, t17979, t17980, t17983, t17984, t17988, t17991, t17994) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1647::<F>(t14164, t17686, t4582, t17691, t4583, t1023, t17670, t4594, t17167, t977, t17171, t17157, t2979);
        let (t17997, t17998, t18005, t18007) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1648::<F>(t1023, t5677, t10408, t1036, t5905, t1041, t10876, t10883, t10952, t13995, t14158, t14160, t17972, t17976, t17980, t17984, t17988, t17991, t17994, t3070, t3109, t4579, t5869, t5880, t973);
    (t17971, t17972, t17975, t17976, t17979, t17980, t17983, t17984, t17997, t17998, t18005, t18007)
}
