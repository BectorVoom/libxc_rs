//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 201/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk201(t1: f64, t231: f64, t369: f64, t46: f64, t382: f64, t132: f64, t283: f64) -> (f64, f64, f64, f64, f64) {
    let t725 = t231 * t1;
    let t727 = 0.18311555036753159941e-3_f64 * t725 * t369;
    let t728 = t231 * t46;
    let t730 = 0.58482233974552040708e0_f64 * t728 * t382;
    let t731 = t132 * t283;
    (t725, t727, t728, t730, t731)
}
