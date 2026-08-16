//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1428/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1428<F: Float>(t78118: F, t78120: F, t78122: F, t78125: F, t78128: F, t78132: F, t78196: F, t78199: F, t78227: F, t78229: F, t78232: F, t78236: F, t78239: F) -> F {
    let t78240 = -t78118 + t78120 - t78122 - t78125 - t78128 - t78132 + t78196 + t78199 + t78227 + t78229 - t78232 - t78236 + t78239;
    t78240
}
