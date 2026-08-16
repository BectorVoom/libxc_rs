//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1100/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1100(t481: f64, t792: f64, t983: f64, t1065: f64, t39190: f64, t10609: f64, t1561: f64, t97: f64, t2625: f64, t13908: f64, t986: f64, t3270: f64) -> (f64, f64, f64) {
    let t39192 = t983 * t481 * t792;
    let t39195 = 135.0_f64 / 32.0_f64 * t39190 * t1065 * t39192;
    let t39197 = t97 * t10609 * t1561;
    let t39198 = t2625 * t792;
    let t39201 = 15.0_f64 / 4.0_f64 * t39197 * t1065 * t39198;
    let t39202 = t13908 * t986;
    let t39203 = t3270 * t39202;
    (t39195, t39201, t39203)
}
