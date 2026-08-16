//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1292/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1292(t20469: f64, t422: f64, t12485: f64, t6518: f64, t5206: f64, t1196: f64, t5192: f64, t5198: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64) -> (f64, f64, f64, f64) {
    let t20471 = 0.621814e-1_f64 * t20469 * t422;
    let t20472 = t12485 * t6518;
    let t20473 = t20472 * t5206;
    let t20475 = 0.10389515463408878255e3_f64 * t1196 * t20473;
    let t20477 = 0.23392894490538584828e1_f64 * t5192 * t5198;
    let t20498 = 0.11477222222222222222e0_f64 * t20283 - 0.34431666666666666667e0_f64 * t20285 - 0.17215833333333333333e0_f64 * t20287 + 0.516475e0_f64 * t20290 + 0.57386111111111111112e0_f64 * t20295 - 0.20659e1_f64 * t20300 - 0.68863333333333333334e0_f64 * t20304 + 0.309885e1_f64 * t20308 + 0.20659e1_f64 * t20312 - 0.34431666666666666667e0_f64 * t20315 + 0.103295e1_f64 * t20320;
    (t20471, t20475, t20477, t20498)
}
