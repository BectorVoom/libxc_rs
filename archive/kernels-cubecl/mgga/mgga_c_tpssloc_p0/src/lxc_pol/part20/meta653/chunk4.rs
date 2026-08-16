//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2412/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2412<F: Float>(t47761: F, t47765: F, t47769: F, t48112: F, t48114: F, t48116: F, t48119: F, t48122: F, t48125: F, t48128: F, t48131: F, t41887: F, t41889: F, t48134: F, t48137: F, t48142: F, t48145: F, t48148: F, t49009: F, t49012: F, t49015: F, t49018: F, t49021: F) -> (F, F) {
    let t49359 = F::cast_from(0.309885e1_f64) * t47761 + F::cast_from(0.309885e1_f64) * t47765 + F::cast_from(0.103295e1_f64) * t47769 + F::cast_from(0.62517e0_f64) * t48112 + F::cast_from(0.20839e0_f64) * t48114 + F::cast_from(0.92617777777777777778e-1_f64) * t48116 + F::cast_from(0.62517000000000000001e0_f64) * t48119 + F::cast_from(0.55570666666666666666e0_f64) * t48122 - F::cast_from(0.187551e1_f64) * t48125 - F::cast_from(0.104195e0_f64) * t48128 - F::cast_from(0.13892666666666666667e0_f64) * t48131;
    let t49372 = -F::cast_from(0.34731666666666666667e-1_f64) * t48134 - F::cast_from(0.10805407407407407407e0_f64) * t48137 + F::cast_from(0.62517e0_f64) * t48142 - F::cast_from(0.187551e1_f64) * t48145 - F::cast_from(0.17648625e1_f64) * t49009 - F::cast_from(0.473371875e0_f64) * t49012 + F::cast_from(0.794188125e1_f64) * t49015 - F::cast_from(0.6618234375e1_f64) * t49018 + F::cast_from(0.2366859375e0_f64) * t49021 - F::cast_from(0.13892666666666666667e0_f64) * t48148 - F::cast_from(0.41678000000000000001e0_f64) * t41887 + F::cast_from(0.69463333333333333332e-1_f64) * t41889;
    (t49359, t49372)
}
