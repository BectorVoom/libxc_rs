//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 826/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk826(t594: f64, t8: f64, t130: f64, t1024: f64, t56: f64, t38: f64, t22: f64, t413: f64, t406: f64, t524: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7321 = 1.0_f64 / t8 / t594;
    let t7322 = t130 * t7321;
    let t7335 = t56 * t1024;
    let t7508 = t38 * t38;
    let t7510 = 1.0_f64 / t22 / t7508;
    let t7599 = t130 * t413;
    let t7777 = 1.0_f64 / t7508;
    let t8401 = t524 * t406;
    (t7321, t7322, t7335, t7510, t7599, t7777, t8401)
}
