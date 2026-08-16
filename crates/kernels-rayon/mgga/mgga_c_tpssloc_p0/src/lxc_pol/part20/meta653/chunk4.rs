//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2412/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2412(t47761: f64, t47765: f64, t47769: f64, t48112: f64, t48114: f64, t48116: f64, t48119: f64, t48122: f64, t48125: f64, t48128: f64, t48131: f64, t41887: f64, t41889: f64, t48134: f64, t48137: f64, t48142: f64, t48145: f64, t48148: f64, t49009: f64, t49012: f64, t49015: f64, t49018: f64, t49021: f64) -> (f64, f64) {
    let t49359 = 0.309885e1_f64 * t47761 + 0.309885e1_f64 * t47765 + 0.103295e1_f64 * t47769 + 0.62517e0_f64 * t48112 + 0.20839e0_f64 * t48114 + 0.92617777777777777778e-1_f64 * t48116 + 0.62517000000000000001e0_f64 * t48119 + 0.55570666666666666666e0_f64 * t48122 - 0.187551e1_f64 * t48125 - 0.104195e0_f64 * t48128 - 0.13892666666666666667e0_f64 * t48131;
    let t49372 = -0.34731666666666666667e-1_f64 * t48134 - 0.10805407407407407407e0_f64 * t48137 + 0.62517e0_f64 * t48142 - 0.187551e1_f64 * t48145 - 0.17648625e1_f64 * t49009 - 0.473371875e0_f64 * t49012 + 0.794188125e1_f64 * t49015 - 0.6618234375e1_f64 * t49018 + 0.2366859375e0_f64 * t49021 - 0.13892666666666666667e0_f64 * t48148 - 0.41678000000000000001e0_f64 * t41887 + 0.69463333333333333332e-1_f64 * t41889;
    (t49359, t49372)
}
