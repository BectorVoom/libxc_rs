//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2671/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2671(t39611: f64, t39620: f64, t39628: f64, t39630: f64, t39632: f64, t39634: f64, t39636: f64, t39642: f64, t39644: f64, t5154: f64, t9722: f64, t39659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54442 = 360.0_f64 * t39611;
    let t54443 = 3.0_f64 * t39620;
    let t54444 = 60.0_f64 * t39628;
    let t54445 = 4.0_f64 * t39630;
    let t54446 = 4.0_f64 * t39632;
    let t54447 = 48.0_f64 * t39634;
    let t54448 = 72.0_f64 * t39636;
    let t54449 = 3.0_f64 * t39642;
    let t54450 = 24.0_f64 * t39644;
    let t54451 = t5154 * t9722;
    let t54452 = 0.10389515463408878255e3_f64 * t54451;
    let t54453 = 96.0_f64 * t39659;
    (t54442, t54443, t54444, t54445, t54446, t54447, t54448, t54449, t54450, t54452, t54453)
}
