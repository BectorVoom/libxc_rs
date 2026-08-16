//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1372/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1372(t58397: f64, t58401: f64, t58405: f64, t58409: f64, t58412: f64, t58415: f64, t58418: f64, t58421: f64, t58424: f64, t58428: f64, t58431: f64, t1025: f64, t11: f64, t58295: f64) -> (f64, f64) {
    let t58433 = 0.2585111111111111111e2_f64 * t58397 - 0.46531999999999999999e2_f64 * t58401 - 0.38776666666666666665e1_f64 * t58405 + 0.46532e2_f64 * t58409 + 0.11633e2_f64 * t58412 - 0.12315e-2_f64 * t58415 - 0.14778e-1_f64 * t58418 - 0.12315e-2_f64 * t58421 + 0.29556e-1_f64 * t58424 - 0.12771111111111111111e-2_f64 * t58428 - 0.57446913580246913579e1_f64 * t58431;
    let t58435 = t11 * t1025 * t58295;
    (t58433, t58435)
}
