//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 325/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk325(t1032: f64, t389: f64, t385: f64, t375: f64, t376: f64, t1023: f64, t1030: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1034 = 0.62182e-1_f64 * t1032 * t389;
    let t1035 = t385 * t385;
    let t1036 = 1.0_f64 / t1035;
    let t1037 = t375 * t1036;
    let t1038 = 1.0_f64 / t376;
    let t1040 = -t1023 / 3.0_f64 - t1030 / 3.0_f64;
    (t1034, t1035, t1036, t1037, t1038, t1040)
}
