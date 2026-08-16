//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta358<F: Float>(t17191: F, t324: F, t300: F, t5689: F, t892: F, t914: F, t11094: F, t5950: F, t3216: F, t5946: F, t4483: F, t4498: F) -> (F, F, F, F, F, F, F) {
        let (t17192, t17194, t17195, t17197, t17198, t17202, t17209) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1595::<F>(t17191, t324, t300, t5689, t892, t914, t11094, t5950, t3216, t5946, t4483, t4498);
    (t17192, t17194, t17195, t17197, t17198, t17202, t17209)
}
