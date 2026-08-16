//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 818/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk818(t12651: f64, t2684: f64, t7354: f64, t12652: f64, t7416: f64, t161: f64, t165: f64, t9688: f64, t2685: f64, t2464: f64, t2465: f64, t9729: f64) -> (f64, f64, f64, f64, f64) {
    let t41411 = t2684 * t7354 * t12651;
    let t41413 = t7416 * t12652;
    let t41416 = t161 * t165 * t9688;
    let t41418 = t2684 * t2685 * t41416;
    let t41422 = t2684 * t2464 * t2465 * t9729;
    (t41411, t41413, t41416, t41418, t41422)
}
