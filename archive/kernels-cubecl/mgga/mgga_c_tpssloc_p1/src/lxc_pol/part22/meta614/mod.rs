//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta614<F: Float>(t11292: F, t1687: F, t50826: F, t50948: F, t11365: F, t1694: F, t3331: F, t4794: F, t50919: F, t300: F, t3401: F, t11310: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t51680, t51683, t51707, t51727, t51730, t51745, t51760, t51769, t51810, t51819) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2142::<F>(t11292, t1687, t50826, t50948, t11365, t1694, t3331, t4794, t50919, t300, t3401, t11310);
    (t51680, t51683, t51707, t51727, t51730, t51745, t51760, t51769, t51810, t51819)
}
