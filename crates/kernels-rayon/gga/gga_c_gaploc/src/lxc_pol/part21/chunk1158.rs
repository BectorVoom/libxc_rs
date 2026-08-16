//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1158/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1158(t3176: f64, t493: f64, t6576: f64, t6578: f64, t20013: f64, t4782: f64, t883: f64, t9272: f64, t20900: f64, t7030: f64, t20374: f64, t7035: f64, t888: f64) -> (f64, f64, f64, f64) {
    let t31393 = t6576 * t493 * t3176 * t6578;
    let t31412 = 0.11502877786176224903e1_f64 * t9272 * t4782 * t883 * t20013;
    let t31414 = 0.59584149919750711116e-1_f64 * t20900 * t7030;
    let t31416 = t20374 * t888 * t7035;
    (t31393, t31412, t31414, t31416)
}
