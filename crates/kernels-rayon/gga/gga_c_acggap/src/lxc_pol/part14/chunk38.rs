//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 38/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk38(t11: f64, t10: f64, t4: f64, t111: f64) -> (f64, f64, f64, f64) {
    let t113 = 1.0_f64 + 0.740825e-1_f64 * t11;
    let t117 = 1.0_f64 + 0.125e0_f64 * t4 * t10 * t113;
    let t118 = 1.0_f64 / t117;
    let t119 = t111 * t118;
    (t113, t117, t118, t119)
}
