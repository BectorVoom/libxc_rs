//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1130/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1130(t2225: f64, t3824: f64, t1287: f64, t9214: f64, t39033: f64, t522: f64, t39035: f64, t39031: f64, t16: f64, t185: f64, t520: f64, t9212: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39595 = 120.0_f64 * t2225 * t3824;
    let t39596 = t9214 * t1287;
    let t39597 = 576.0_f64 * t39596;
    let t39603 = t39033 * t522;
    let t39604 = 1440.0_f64 * t39603;
    let t39605 = t39035 * t522;
    let t39606 = 1920.0_f64 * t39605;
    let t39607 = t39031 * t522;
    let t39608 = 384.0_f64 * t39607;
    let t39615 = 24.0_f64 * t16 * t520 * t185;
    let t39634 = t9212 * t1287;
    (t39595, t39597, t39604, t39606, t39608, t39615, t39634)
}
