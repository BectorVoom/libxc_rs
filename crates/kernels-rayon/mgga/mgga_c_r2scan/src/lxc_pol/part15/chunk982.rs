//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 982/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk982(t11523: f64, t3271: f64, t10619: f64, t3579: f64, t10615: f64, t3275: f64, t3582: f64, t2847: f64, t797: f64, t3276: f64, t3696: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11524 = t11523 * t3271;
    let t11525 = t11524 / 4.0_f64;
    let t11526 = t3579 * t10619;
    let t11527 = t11526 / 4.0_f64;
    let t11529 = t3275 * t10615 * t3582;
    let t11530 = 5.0_f64 / 16.0_f64 * t11529;
    let t11531 = t797 * t2847;
    let t11533 = t3275 * t3276 * t11531;
    let t11534 = 5.0_f64 / 16.0_f64 * t11533;
    let t11535 = t860 * t3696;
    (t11524, t11525, t11526, t11527, t11529, t11530, t11531, t11533, t11534, t11535)
}
