//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 813/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk813(t10033: f64, t2628: f64, t2617: f64, t3251: f64, t7810: f64, t2679: f64, t3243: f64, t9796: f64, t3255: f64, t7803: f64, t22980: f64, t2615: f64, t9438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41093 = t10033 * t2628;
    let t41133 = t7810 * t3251 * t2617;
    let t41136 = t9796 * t3243 * t2679;
    let t41139 = t9796 * t3255 * t2679;
    let t41143 = t7803 * t3243 * t2617;
    let t41231 = t2615 * t9438 * t22980;
    (t41093, t41133, t41136, t41139, t41143, t41231)
}
