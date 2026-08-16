//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2113;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta676<F: Float>(t15572: F, t24741: F, t15501: F, t24727: F, t3500: F, t7337: F, t27710: F, t3: F, t24684: F, t15608: F, t24682: F, t460: F, t95484: F) -> (F, F, F, F, F, F, F) {
        let (t95617, t95623, t95627, t95648, t95649, t95662, t95678) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2113::<F>(t15572, t24741, t15501, t24727, t3500, t7337, t27710, t3, t24684, t15608, t24682, t460, t95484);
    (t95617, t95623, t95627, t95648, t95649, t95662, t95678)
}
