//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 232/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk232(t316: f64, t872: f64, t113: f64, t4: f64, t668: f64, t678: f64) -> (f64, f64) {
    let t873 = t316 * t872;
    let t879 = 0.55555555555555555556e-1_f64 * t4 * t668 * t113 + 0.24694166666666666668e-1_f64 * t678;
    (t873, t879)
}
