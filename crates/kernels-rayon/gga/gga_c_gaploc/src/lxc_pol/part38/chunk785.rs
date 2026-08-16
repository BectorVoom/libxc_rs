//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 785/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk785(t12993: f64, t7014: f64, t10215: f64, t123: f64, t883: f64, t2487: f64, t2488: f64, t10151: f64, t2464: f64, t2465: f64, t10417: f64, t1415: f64, t7030: f64) -> (f64, f64, f64, f64, f64) {
    let t41631 = t7014 * t12993;
    let t41634 = t10215 * t123 * t883;
    let t41636 = t2487 * t2488 * t41634;
    let t41640 = t2487 * t2464 * t2465 * t10151;
    let t41643 = t1415 * t10417 * t7030;
    (t41631, t41634, t41636, t41640, t41643)
}
