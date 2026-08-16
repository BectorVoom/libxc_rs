//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 841/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk841(t6188: f64, t8676: f64, t9798: f64, t2664: f64, t9504: f64, t3127: f64, t3363: f64, t3132: f64, t7294: f64, t7259: f64, t8624: f64, t7325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9799 = t8676 * t6188;
    let t9800 = t9798 * t9799;
    let t9802 = t9504 * t2664;
    let t9804 = t3363 * t3127;
    let t9805 = t9804 * t2664;
    let t9807 = t7294 * t3132;
    let t9808 = t9807 * t2664;
    let t9810 = t7259 * t8624;
    let t9811 = t9810 * t7325;
    (t9799, t9800, t9802, t9805, t9808, t9810, t9811)
}
