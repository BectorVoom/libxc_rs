//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1038 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3628;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1038<F: Float>(t20473: F, t3531: F, t16685: F, t5192: F, t16652: F, t57854: F, t1196: F, t12500: F, t20472: F, t20892: F, t20384: F, t3497: F, t45187: F, t45190: F, t6518: F, t16784: F, t5198: F, t12571: F, t6548: F, t1149: F, t56265: F, t57795: F, t17151: F, t5197: F, t16639: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t68707, t68709, t68711, t68714, t68716, t68718, t68723) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3628::<F>(t20473, t3531, t16685, t5192, t16652, t57854, t1196, t12500, t20472, t20892, t20384, t3497, t45187, t45190, t6518);
        let (t68725, t68727, t68730, t68733, t68735) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3629::<F>(t16784, t5198, t12571, t6548, t1149, t56265, t57795, t1196, t17151, t5197, t16639, t5192);
    (t68707, t68709, t68711, t68714, t68716, t68718, t68723, t68725, t68727, t68730, t68733, t68735)
}
