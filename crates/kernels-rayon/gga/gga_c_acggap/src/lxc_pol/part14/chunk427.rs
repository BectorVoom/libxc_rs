//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 427/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk427(t301: f64, t599: f64, t142: f64, t2030: f64, t130: f64, t228: f64) -> (f64, f64, f64, f64) {
    let t2031 = t599 * t301;
    let t2032 = t142 * t2031;
    let t2033 = t2030 * t2032;
    let t2035 = t130 * t228;
    (t2031, t2032, t2033, t2035)
}
