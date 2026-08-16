//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 743/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk743(t2558: f64, t28438: f64, t10928: f64, t6574: f64, t822: f64, t2012: f64, t7809: f64, t7802: f64, t5638: f64, t9419: f64, t5538: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28439 = t28438 * t2558;
    let t28640 = t822 * t10928 * t6574;
    let t28673 = t2012 * t7809;
    let t28737 = t2012 * t7802;
    let t28856 = t822 * t5638 * t9419;
    let t29277 = t5538 * t883;
    (t28439, t28640, t28673, t28737, t28856, t29277)
}
