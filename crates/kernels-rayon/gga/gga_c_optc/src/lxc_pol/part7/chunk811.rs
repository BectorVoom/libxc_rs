//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 811/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk811(t265: f64, t7620: f64, t241: f64, t2449: f64, t778: f64, t800: f64, t2378: f64, t2410: f64, t2415: f64, t774: f64, t2419: f64, t7339: f64, t7346: f64, t7348: f64, t7499: f64, t7507: f64, t7509: f64, t7608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7621 = t7620 * t265;
    let t7623 = 0.19751789702565206229e-1_f64 * t241 * t7621;
    let t7624 = t2449 * t778;
    let t7626 = 3.0_f64 * t7624 * t800;
    let t7628 = 3.0_f64 * t2378 * t2410;
    let t7629 = t774 * t2415;
    let t7631 = 0.48245472966453314466e2_f64 * t7629 * t2419;
    let t7632 = -t7499 - t7507 - t7339 + t7346 - t7509 - t7608 + t7348 + t7623 + t7626 + t7628 + t7631;
    (t7621, t7623, t7624, t7626, t7628, t7629, t7631, t7632)
}
