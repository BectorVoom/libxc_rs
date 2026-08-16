//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1311/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1311(t22765: f64, t6417: f64, t6390: f64, t80997: f64, t22797: f64, t6375: f64, t22779: f64, t28057: f64, t6371: f64, t80827: f64, t28073: f64, t80888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97378 = t22765 * t6417;
    let t97380 = t80997 * t6390;
    let t97394 = t22797 * t6375;
    let t97400 = t22779 * t28057;
    let t97402 = t80827 * t6371;
    let t97404 = t80888 * t28073;
    (t97378, t97380, t97394, t97400, t97402, t97404)
}
