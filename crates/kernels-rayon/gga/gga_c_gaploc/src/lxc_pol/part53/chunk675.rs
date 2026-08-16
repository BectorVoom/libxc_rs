//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 675/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk675(t1247: f64, t3103: f64, t12380: f64, t464: f64, t866: f64, t3109: f64, t871: f64, t3113: f64, t869: f64, t1233: f64, t157: f64, t883: f64, t9193: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12397 = t1247 * t3103;
    let t12399 = t464 * t12380;
    let t12400 = t12399 * t866;
    let t12404 = t3109 * t871;
    let t12405 = t869 * t3113;
    let t12411 = 1.0_f64 / t1233;
    let t12412 = t157 * t12411;
    let t12423 = t883 * t9193;
    (t12397, t12399, t12400, t12404, t12405, t12411, t12412, t12423)
}
