//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta638<F: Float>(t2403: F, t2830: F, t909: F, t9709: F, t2833: F, t2827: F, t10213: F, t241: F, t41654: F, t270: F, t276: F, t39267: F) -> (F, F, F, F, F, F, F) {
        let (t41831, t41863, t41870, t41872, t41880, t41904, t41935) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2428::<F>(t2403, t2830, t909, t9709, t2833, t2827, t10213, t241, t41654, t270, t276, t39267);
    (t41831, t41863, t41870, t41872, t41880, t41904, t41935)
}
