//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 242/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk242(t921: f64, t922: f64, t402: f64, t839: f64, t153: f64, t155: f64, t400: f64, t403: f64, t917: f64) -> (f64, f64, f64) {
    let t923 = t921 * t922;
    let t926 = t402 * t839;
    let t929 = -12.0_f64 * t153 * t923 + 3.0_f64 * t153 * t926 - t155 * t917 + 6.0_f64 * t400 * t403;
    (t923, t926, t929)
}
