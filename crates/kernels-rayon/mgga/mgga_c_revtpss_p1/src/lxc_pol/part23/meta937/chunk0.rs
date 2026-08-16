//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3081/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3081(t1145: f64, t141: f64, t81226: f64, t24294: f64, t698: f64, t24288: f64, t24291: f64, t68262: f64, t68277: f64, t68312: f64, t68332: f64, t68334: f64, t68336: f64, t68368: f64, t68370: f64) -> (f64, f64, f64, f64, f64) {
    let t81423 = t141 * t1145 * t81226;
    let t81425 = t698 * t24294;
    let t81427 = t698 * t24288;
    let t81429 = t698 * t24291;
    let t81437 = -0.33218518518518518518e0_f64 * t68262 - 0.59793333333333333334e0_f64 * t68277 + 0.82156666666666666667e-1_f64 * t81423 - 0.54771111111111111111e-1_f64 * t81425 + 0.10954222222222222222e0_f64 * t81427 - 0.32862666666666666666e0_f64 * t81429 + 0.5477111111111111111e-1_f64 * t68312 + 0.19931111111111111111e0_f64 * t68332 + 0.39862222222222222222e0_f64 * t68334 + 0.11958666666666666667e1_f64 * t68336 - 0.32862666666666666666e0_f64 * t68368 - 0.73028148148148148146e-1_f64 * t68370;
    (t81423, t81425, t81427, t81429, t81437)
}
