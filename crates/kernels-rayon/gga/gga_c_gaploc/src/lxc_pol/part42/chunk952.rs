//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 952/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk952(t12092: f64, t2478: f64, t6583: f64, t188: f64, t46965: f64, t11977: f64, t524: f64, t13778: f64, t2487: f64, t6985: f64, t12078: f64, t1415: f64, t7030: f64) -> (f64, f64, f64, f64, f64) {
    let t48178 = t6583 * t12092 * t2478;
    let t48187 = t188 * t46965;
    let t48190 = t524 * t11977;
    let t48194 = t2487 * t6985 * t13778;
    let t48208 = t1415 * t12078 * t7030;
    (t48178, t48187, t48190, t48194, t48208)
}
