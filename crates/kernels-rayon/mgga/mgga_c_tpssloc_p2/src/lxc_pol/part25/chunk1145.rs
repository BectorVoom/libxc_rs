//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1145/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1145(t2635: f64, t81803: f64, t2690: f64, t6612: f64, t812: f64, t831: f64, t23041: f64, t2686: f64, t6614: f64, t9663: f64, t23048: f64, t9983: f64) -> (f64, f64, f64, f64, f64) {
    let t81804 = t81803 * t2635;
    let t81807 = t812 * t6612 * t2690;
    let t81808 = t81807 * t831;
    let t81810 = t23041 * t2686;
    let t81812 = t6614 * t9663;
    let t81814 = t23048 * t9983;
    (t81804, t81808, t81810, t81812, t81814)
}
