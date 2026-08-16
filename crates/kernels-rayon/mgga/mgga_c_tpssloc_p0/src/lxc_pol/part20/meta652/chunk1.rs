//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2401/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2401(t47761: f64, t47765: f64, t47769: f64, t48112: f64, t48114: f64, t48116: f64, t48119: f64, t48122: f64, t48125: f64, t48128: f64, t48131: f64, t41887: f64, t41889: f64, t48134: f64, t48137: f64, t48142: f64, t48145: f64, t48148: f64, t49009: f64, t49012: f64, t49015: f64, t49018: f64, t49021: f64) -> (f64, f64) {
    let t49181 = 0.181155e1_f64 * t47761 + 0.181155e1_f64 * t47765 + 0.60385e0_f64 * t47769 + 0.49671e0_f64 * t48112 + 0.16557e0_f64 * t48114 + 0.73586666666666666668e-1_f64 * t48116 + 0.49671e0_f64 * t48119 + 0.44152e0_f64 * t48122 - 0.149013e1_f64 * t48125 - 0.82785e-1_f64 * t48128 - 0.11038e0_f64 * t48131;
    let t49194 = -0.27595e-1_f64 * t48134 - 0.8585111111111111111e-1_f64 * t48137 + 0.49671e0_f64 * t48142 - 0.149013e1_f64 * t48145 - 0.1294625e1_f64 * t49009 - 0.1237865625e0_f64 * t49012 + 0.58258125e1_f64 * t49015 - 0.485484375e1_f64 * t49018 + 0.6189328125e-1_f64 * t49021 - 0.11038e0_f64 * t48148 - 0.33114e0_f64 * t41887 + 0.55190000000000000001e-1_f64 * t41889;
    (t49181, t49194)
}
