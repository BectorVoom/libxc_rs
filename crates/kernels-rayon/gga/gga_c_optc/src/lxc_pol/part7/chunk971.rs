//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 971/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk971(t9322: f64, t9334: f64, t1200: f64, t1205: f64, t2881: f64, t2886: f64, t2887: f64, t2900: f64, t485: f64, t9292: f64, t9294: f64, t9297: f64, t9304: f64, t9305: f64, t9308: f64) -> (f64, f64) {
    let t9335 = t9322 + t9334;
    let t9337 = -t1200 * t9335 - 3.0_f64 * t9294 * t1205 - 3.0_f64 * t2881 * t2900 + 6.0_f64 * t2886 * t9308 + 6.0_f64 * t9297 * t2887 + t9292 * t485 - 6.0_f64 * t9304 * t9305;
    (t9335, t9337)
}
