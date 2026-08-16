//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1132/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1132(t1113: f64, t1501: f64, t12387: f64, t3068: f64, t12378: f64, t1289: f64, t12377: f64, t1562: f64, t4052: f64, t4047: f64, t9702: f64, t1114: f64, t5068: f64) -> (f64, f64, f64, f64, f64) {
    let t15567 = t1501 * t1113;
    let t15568 = t12387 * t15567;
    let t15569 = t3068 * t15568;
    let t15572 = t12378 * t1289;
    let t15573 = t12377 * t15572;
    let t15574 = t3068 * t15573;
    let t15577 = t1562 * t4052;
    let t15578 = t3068 * t15577;
    let t15581 = t1562 * t4047;
    let t15582 = t9702 * t15581;
    let t15585 = t5068 * t1114;
    (t15569, t15574, t15578, t15582, t15585)
}
