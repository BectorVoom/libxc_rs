//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 235/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk235(t286: f64, t912: f64, t420: f64, t94: f64, t377: f64, t396: f64) -> (f64, f64, f64) {
    let t913 = t286 * t912;
    let t914 = 0.11696447245269292414e1_f64 * t913;
    let t921 = t94 * t420;
    let t935 = t377 * t396;
    (t914, t921, t935)
}
