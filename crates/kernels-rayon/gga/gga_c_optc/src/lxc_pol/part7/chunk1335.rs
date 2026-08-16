//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1335/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1335(t26280: f64, t26284: f64, t26293: f64, t26296: f64, t26304: f64, t26311: f64, t26324: f64, t26394: f64, t26396: f64, t26398: f64, t26406: f64, t26409: f64, t26412: f64, t26415: f64) -> f64 {
    let t26642 = -0.13772666666666666667e1_f64 * t26324 - 0.166712e1_f64 * t26394 + 0.27785333333333333333e0_f64 * t26396 - 0.52945875e1_f64 * t26398 + 0.41318e1_f64 * t26280 - 0.123954e2_f64 * t26284 - 0.103295e1_f64 * t26293 + 0.123954e2_f64 * t26296 + 0.309885e1_f64 * t26304 - 0.27545333333333333332e1_f64 * t26311 + 0.166712e1_f64 * t26406 - 0.125034e1_f64 * t26409 - 0.104195e0_f64 * t26412 + 0.250068e1_f64 * t26415;
    t26642
}
