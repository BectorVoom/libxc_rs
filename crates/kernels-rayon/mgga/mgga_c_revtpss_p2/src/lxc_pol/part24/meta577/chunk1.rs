//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1770/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1770(t422: f64, t90614: f64, t90626: f64, t20400: f64, t6556: f64, t1196: f64, t24408: f64, t5197: f64, t24473: f64, t5192: f64, t1188: f64, t12485: f64, t90357: f64) -> (f64, f64, f64, f64, f64) {
    let t90629 = 0.621814e-1_f64 * (t90614 + t90626) * t422;
    let t90631 = 0.10389515463408878255e3_f64 * t20400 * t6556;
    let t90634 = 0.46785788981077169656e1_f64 * t1196 * t5197 * t24408;
    let t90636 = 0.20779030926817756511e3_f64 * t5192 * t24473;
    let t90640 = 0.14035736694323150897e2_f64 * t1196 * t12485 * t90357 * t1188;
    (t90629, t90631, t90634, t90636, t90640)
}
