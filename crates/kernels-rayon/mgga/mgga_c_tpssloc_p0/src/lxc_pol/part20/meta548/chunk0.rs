//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2091/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2091(t118: f64, t2576: f64, t794: f64, t9516: f64, t207: f64, t40394: f64, t40399: f64, t2582: f64, t9541: f64, t786: f64, t9580: f64, t2578: f64) -> (f64, f64, f64, f64, f64) {
    let t41181 = t2576 * t118 * t794 * t9516;
    let t41185 = 0.69444444444444444445e-4_f64 * t40394 * t207 * t40399;
    let t41187 = t9541 * t2582;
    let t41189 = t9580 * t786;
    let t41190 = t41189 * t2578;
    (t41181, t41185, t41187, t41189, t41190)
}
