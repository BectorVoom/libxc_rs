//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 938/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk938(t12161: f64, t123: f64, t883: f64, t2684: f64, t2685: f64, t12213: f64, t2464: f64, t2465: f64, t13851: f64, t2013: f64, t12240: f64, t2679: f64, t9800: f64) -> (f64, f64, f64, f64, f64) {
    let t47143 = t12161 * t123 * t883;
    let t47145 = t2684 * t2685 * t47143;
    let t47149 = t2684 * t2464 * t2465 * t12213;
    let t47151 = t2013 * t13851;
    let t47166 = t9800 * t12240 * t2679;
    (t47143, t47145, t47149, t47151, t47166)
}
