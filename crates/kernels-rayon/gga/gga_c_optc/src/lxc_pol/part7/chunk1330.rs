//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1330/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1330(t26280: f64, t26284: f64, t26293: f64, t26296: f64, t26304: f64, t26311: f64, t26324: f64, t26394: f64, t26396: f64, t26398: f64, t26406: f64, t26409: f64, t26412: f64, t26415: f64) -> f64 {
    let t26539 = -0.80513333333333333332e0_f64 * t26324 - 0.132456e1_f64 * t26394 + 0.22076e0_f64 * t26396 - 0.3883875e1_f64 * t26398 + 0.24154e1_f64 * t26280 - 0.72462e1_f64 * t26284 - 0.60384999999999999999e0_f64 * t26293 + 0.72462e1_f64 * t26296 + 0.181155e1_f64 * t26304 - 0.16102666666666666667e1_f64 * t26311 + 0.132456e1_f64 * t26406 - 0.99342e0_f64 * t26409 - 0.82785e-1_f64 * t26412 + 0.198684e1_f64 * t26415;
    t26539
}
