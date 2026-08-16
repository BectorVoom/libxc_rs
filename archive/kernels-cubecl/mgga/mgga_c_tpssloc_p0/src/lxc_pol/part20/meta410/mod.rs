//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1814;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta410<F: Float>(t13969: F, t4599: F, t3039: F, t376: F, t4649: F) -> (F, F, F) {
        let (t13970, t13972, t13975) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1814::<F>(t13969, t4599, t3039, t376, t4649);
    (t13970, t13972, t13975)
}
