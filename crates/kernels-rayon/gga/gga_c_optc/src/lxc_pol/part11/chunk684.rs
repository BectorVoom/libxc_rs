//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 684/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk684(t2219: f64, t727: f64, t2224: f64, t3649: f64, t3696: f64, t183: f64, t2213: f64, t2218: f64, t6576: f64, t6578: f64, t6581: f64, t6588: f64, t724: f64) -> (f64, f64, f64, f64) {
    let t6589 = t2219 * t727;
    let t6592 = t727 * t2224;
    let t6597 = -0.22615185185185185185e4_f64 * t3649 - 0.34962962962962962963e3_f64 * t3696;
    let t6599 = t6576 * t183 - 3.0_f64 * t2213 * t2224 + 6.0_f64 * t2218 * t6592 + 6.0_f64 * t6581 * t2219 - 3.0_f64 * t6578 * t727 - 6.0_f64 * t6588 * t6589 - t724 * t6597;
    (t6589, t6592, t6597, t6599)
}
