//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 113/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk113(t123: f64, t130: f64, t138: f64, t133: f64, t134: f64) -> (f64, f64, f64, f64) {
    let t342 = t130 * t123;
    let t343 = t342 * t138;
    let t344 = t343 / 3.0_f64;
    let t345 = t133 * t134;
    (t342, t343, t344, t345)
}
