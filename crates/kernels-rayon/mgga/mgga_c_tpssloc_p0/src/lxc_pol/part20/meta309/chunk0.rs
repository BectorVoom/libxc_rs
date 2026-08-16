//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1559/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1559(t11129: f64, t11292: f64, t3403: f64, t1164: f64, t1143: f64, t3375: f64, t1156: f64, t1124: f64, t3331: f64, t1136: f64, t3333: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11294 = t11292 * t11129 * t3403;
    let t11296 = 0.10389515463408878255e3_f64 * t1164 * t11294;
    let t11297 = t1143 * t3375;
    let t11300 = t11129 * t1156;
    let t11303 = t1124 * t3331;
    let t11306 = t3333 * t1136;
    (t11294, t11296, t11297, t11300, t11303, t11306)
}
