//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta636<F: Float>(t2281: F, t5465: F, t19474: F, t626: F, t19483: F, t19477: F, t1409: F, t628: F, t67: F, t19297: F, t604: F, t2239: F, t5385: F) -> (F, F, F, F, F, F, F) {
        let (t55537, t55546, t55559, t55561, t55653, t55880, t55921) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2174::<F>(t2281, t5465, t19474, t626, t19483, t19477, t1409, t628, t67, t19297, t604, t2239, t5385);
    (t55537, t55546, t55559, t55561, t55653, t55880, t55921)
}
