//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta589<F: Float>(t43819: F, t11223: F, t699: F, t11205: F, t11208: F, t2403: F, t3298: F, t11220: F, t1114: F, t9709: F, t3304: F, t3301: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t43820, t43835, t43837, t43839, t43855, t43857, t43859, t43861, t43863) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2168::<F>(t43819, t11223, t699, t11205, t11208, t2403, t3298, t11220, t1114, t9709, t3304, t3301);
    (t43820, t43835, t43837, t43839, t43855, t43857, t43859, t43861, t43863)
}
