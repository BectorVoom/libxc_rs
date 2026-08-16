//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta637<F: Float>(t111: F, t19449: F, t19681: F, t2528: F, t172: F, t19572: F, t763: F, t2535: F, t2371: F, t19575: F, t592: F, t1390: F, t20063: F) -> (F, F, F, F, F, F, F) {
        let (t55943, t56099, t56102, t56104, t56168, t56185, t56358) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2175::<F>(t111, t19449, t19681, t2528, t172, t19572, t763, t2535, t2371, t19575, t592, t1390, t20063);
    (t55943, t56099, t56102, t56104, t56168, t56185, t56358)
}
