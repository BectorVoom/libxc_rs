//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1320/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1320(t26280: f64, t26284: f64, t26293: f64, t26296: f64, t26304: f64, t26311: f64, t26324: f64, t26394: f64, t26396: f64, t26398: f64, t26406: f64, t26409: f64, t26412: f64, t26415: f64) -> f64 {
    let t26417 = -0.79724444444444444444e0_f64 * t26324 - 0.13145066666666666666e1_f64 * t26394 + 0.21908444444444444444e0_f64 * t26396 - 0.28483875e1_f64 * t26398 + 0.23917333333333333333e1_f64 * t26280 - 0.71752000000000000002e1_f64 * t26284 - 0.59793333333333333333e0_f64 * t26293 + 0.71752e1_f64 * t26296 + 0.17938e1_f64 * t26304 - 0.15944888888888888889e1_f64 * t26311 + 0.13145066666666666666e1_f64 * t26406 - 0.98587999999999999998e0_f64 * t26409 - 0.82156666666666666668e-1_f64 * t26412 + 0.197176e1_f64 * t26415;
    t26417
}
