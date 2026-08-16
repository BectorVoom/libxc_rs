//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 895/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk895(t106: f64, t797: f64, t9563: f64, t97: f64, t2266: f64, t6955: f64, t910: f64, t3245: f64, t6897: f64) -> (f64, f64, f64) {
    let t9566 = t97 * t106 * t9563 * t797;
    let t9568 = t2266 * t6955 * t910;
    let t9569 = 6.0_f64 * t9568;
    let t9573 = t3245 * t6897;
    (t9566, t9569, t9573)
}
