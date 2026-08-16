//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 395/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk395(t628: f64, t641: f64, t1554: f64, t181: f64, t200: f64, t505: f64, t172: f64) -> (f64, f64, f64, f64) {
    let t1921 = t628 * t641;
    let t1924 = t181 * t1554;
    let t1927 = t505 * t200;
    let t1928 = t1927 * t172;
    (t1921, t1924, t1927, t1928)
}
