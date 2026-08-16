//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1893/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1893(t1339: f64, t19732: f64, t6936: f64, t22779: f64, t28057: f64, t6371: f64, t80827: f64, t28073: f64, t80888: f64, t26301: f64, t7708: f64, t91208: f64) -> (f64, f64, f64, f64, f64) {
    let t97398 = t6936 * t1339 * t19732;
    let t97400 = t22779 * t28057;
    let t97402 = t80827 * t6371;
    let t97404 = t80888 * t28073;
    let t97407 = t91208 * t7708 * t26301;
    (t97398, t97400, t97402, t97404, t97407)
}
