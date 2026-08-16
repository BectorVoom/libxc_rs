//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta444<F: Float>(t11697: F, t4953: F, t3577: F, t12648: F, t4972: F, t4582: F, t1229: F, t3242: F, t14165: F, t3493: F, t3508: F, t4977: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15608, t15610, t15611, t15612, t15615, t15616, t15617, t15620, t15621) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1991::<F>(t11697, t4953, t3577, t12648, t4972, t4582, t1229, t3242, t14165, t3493, t3508, t4977);
    (t15608, t15610, t15611, t15612, t15615, t15616, t15617, t15620, t15621)
}
