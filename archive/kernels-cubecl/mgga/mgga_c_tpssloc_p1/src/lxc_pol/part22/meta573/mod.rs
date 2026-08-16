//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta573<F: Float>(t22715: F, t268: F, t405: F, t1114: F, t9709: F, t39267: F, t404: F, t410: F, t407: F, t1094: F, t11274: F, t3262: F, t3311: F, t409: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t43819, t43820, t43859, t43880, t43889, t43895, t43942, t43964, t43969) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2082::<F>(t22715, t268, t405, t1114, t9709, t39267, t404, t410, t407, t1094, t11274, t3262, t3311, t409);
    (t43819, t43820, t43859, t43880, t43889, t43895, t43942, t43964, t43969)
}
