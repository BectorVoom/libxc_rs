//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 193/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk193(t31: f64, t4: f64, t509: f64, t27: f64, t13: f64, t1: f64, t14: f64, t3: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t512 = 0.11073577833333333333e-2_f64 * t4 * t509 * t31;
    let t513 = t27 * t27;
    let t514 = 1.0_f64 / t513;
    let t515 = t13 * t514;
    let t517 = 1.0_f64 / t14 * t1;
    let t518 = t3 * t6;
    (t512, t513, t514, t515, t517, t518)
}
