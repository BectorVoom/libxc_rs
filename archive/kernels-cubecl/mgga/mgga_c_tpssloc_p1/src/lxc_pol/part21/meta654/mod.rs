//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta654<F: Float>(t3185: F, t42741: F, t1014: F, t42340: F, t42341: F, t3127: F, t23508: F, t3131: F, t3199: F, t10474: F, t10482: F, t11060: F, t3120: F) -> (F, F, F, F, F, F, F, F) {
        let (t43480, t43503, t43515, t43516, t43536, t43553, t43554, t43558) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2452::<F>(t3185, t42741, t1014, t42340, t42341, t3127, t23508, t3131, t3199, t10474, t10482, t11060, t3120);
    (t43480, t43503, t43515, t43516, t43536, t43553, t43554, t43558)
}
