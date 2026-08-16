//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 676/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk676(t1268: f64, t1458: f64, t4028: f64, t5450: f64, t5456: f64, t5493: f64, t88: f64, t5155: f64, t5158: f64, t1799: f64, t5122: f64, t5169: f64) -> (f64, f64, f64, f64, f64) {
    let t6295 = 2.0_f64 * t1268 * t5493 + 4.0_f64 * t1458 * t4028 + 2.0_f64 * t5456 * t88 + t5450;
    let t6299 = 0.11696447245269292414e1_f64 * t5155;
    let t6300 = 0.36622894612013090108e-3_f64 * t5158;
    let t6301 = t5122 * t1799;
    let t6304 = 2.0_f64 * t5169;
    (t6295, t6299, t6300, t6301, t6304)
}
