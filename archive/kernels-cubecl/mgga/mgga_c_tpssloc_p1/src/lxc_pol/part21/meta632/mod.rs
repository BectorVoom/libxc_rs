//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta632<F: Float>(t41083: F, t789: F, t154: F, t1891: F, t205: F, t207: F, t40394: F, t40399: F, t2582: F, t9541: F, t786: F, t9580: F) -> (F, F, F, F, F, F) {
        let (t41156, t41160, t41161, t41185, t41187, t41189) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2416::<F>(t41083, t789, t154, t1891, t205, t207, t40394, t40399, t2582, t9541, t786, t9580);
    (t41156, t41160, t41161, t41185, t41187, t41189)
}
