//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1197/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1197(t108587: f64, t108590: f64, t108592: f64, t108601: f64, t114564: f64, t114566: f64, t96323: f64, t96326: f64, t96341: f64, t96342: f64, t98218: f64, t98220: f64, t98224: f64, t98260: f64) -> f64 {
    let t115052 = -t96323 - 0.3658582879408617555e-2_f64 * t98218 + 0.34299214494455789577e-3_f64 * t108587 - 0.54214778996945588151e-4_f64 * t98220 - 0.24009450146119052704e-1_f64 * t108590 + 0.12004725073059526352e-1_f64 * t108592 - 0.68026775414003982662e-1_f64 * t98224 + t96326 - 0.85748036236139473944e-3_f64 * t114564 + 0.51448821741683684367e-2_f64 * t114566 - 35.0_f64 / 36.0_f64 * t98260 - t96341 + t96342 + 0.85748036236139473944e-4_f64 * t108601;
    t115052
}
