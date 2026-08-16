//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta852 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta852<F: Float>(t1113: F, t136: F, t63294: F, t63298: F, t63302: F, t2403: F, t6017: F, t11219: F, t63415: F, t43748: F, t63332: F, t63334: F, t63336: F, t63886: F, t63888: F, t63891: F, t63893: F, t63896: F, t63899: F) -> (F, F, F, F, F, F) {
        let (t63903, t63906, t63909, t63911, t63914, t63916) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3081::<F>(t1113, t136, t63294, t63298, t63302, t2403, t6017, t11219, t63415, t43748, t63332, t63334, t63336, t63886, t63888, t63891, t63893, t63896, t63899);
    (t63903, t63906, t63909, t63911, t63914, t63916)
}
