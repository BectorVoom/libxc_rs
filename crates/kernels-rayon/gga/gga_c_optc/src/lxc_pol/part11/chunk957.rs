//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 957/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk957(t17561: f64, t17573: f64, t1565: f64, t5458: f64, t5469: f64, t11671: f64, t14885: f64, t14887: f64, t14889: f64, t17389: f64, t17392: f64, t17406: f64, t17409: f64, t17419: f64, t9311: f64, t9312: f64) -> (f64, f64, f64, f64) {
    let t17574 = t17561 + t17573;
    let t17582 = t5458 * t1565;
    let t17585 = t1565 * t5469;
    let t17597 = -0.96922222222222222223e3_f64 * t11671 - 0.78666666666666666667e2_f64 * t17419 - t9311 - t9312 - 0.14538333333333333333e4_f64 * t14887 + 0.72691666666666666668e3_f64 * t14889 + 0.48461111111111111112e3_f64 * t14885 + 0.15733333333333333333e3_f64 * t17406 - 0.78666666666666666666e2_f64 * t17389 - 0.47199999999999999999e3_f64 * t17409 + 0.47199999999999999999e3_f64 * t17392;
    (t17574, t17582, t17585, t17597)
}
