//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 277/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk277(t330: f64, t363: f64, t130: f64, t328: f64, t138: f64, t134: f64, t342: f64) -> (f64, f64, f64, f64, f64) {
    let t1044 = t330 * t363;
    let t1046 = t130 * t328;
    let t1047 = t1046 * t138;
    let t1048 = 7.0_f64 / 9.0_f64 * t1047;
    let t1049 = t342 * t134;
    (t1044, t1046, t1047, t1048, t1049)
}
