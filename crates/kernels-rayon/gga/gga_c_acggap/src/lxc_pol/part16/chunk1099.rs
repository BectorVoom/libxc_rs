//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1099/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1099(t137: f64, t5674: f64, t1089: f64, t1459: f64, t598: f64, t1980: f64, t38892: f64, t7458: f64, t1967: f64, t9543: f64, t1988: f64, t9560: f64) -> (f64, f64, f64, f64, f64) {
    let t39219 = t137 * t5674;
    let t39222 = t598 * t1089 * t1459 * t39219;
    let t39226 = t1980 * t7458 * t1459 * t38892;
    let t39228 = t1967 * t9543;
    let t39230 = t1988 * t9560;
    (t39219, t39222, t39226, t39228, t39230)
}
