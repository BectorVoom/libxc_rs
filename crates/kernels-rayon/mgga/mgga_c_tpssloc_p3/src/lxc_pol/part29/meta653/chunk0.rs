//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2178/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2178(t2241: f64, t72: f64, t7431: f64, t12648: f64, t605: f64, t12652: f64, t12661: f64, t4017: f64, t645: f64, t1433: f64, t12568: f64, t608: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t90141 = t72 * t7431 * t2241;
    let t90150 = t605 * t12648;
    let t90153 = t605 * t12652;
    let t90160 = t605 * t12661;
    let t90177 = t72 * t4017 * t645;
    let t90196 = t72 * t1433 * t2241;
    let t90202 = t12568 * t608;
    (t90141, t90150, t90153, t90160, t90177, t90196, t90202)
}
