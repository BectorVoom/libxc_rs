//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 415/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk415(t239: f64, t64: f64, t2270: f64, t2276: f64, t2279: f64, t2283: f64, t2286: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64) -> (f64, f64) {
    let t2289 = t64 * t239;
    let t2290 = 88.0_f64 / 9.0_f64 * t2289;
    let t2291 = 88.0_f64 / 9.0_f64 * t2270 * t49 - 40.0_f64 / 9.0_f64 * t614 * t617 + 5.0_f64 / 18.0_f64 * t44 * t2276 + 5.0_f64 / 6.0_f64 * t44 * t2279 + 5.0_f64 / 18.0_f64 * t56 * t2283 - 5.0_f64 / 6.0_f64 * t56 * t2286 - t2290;
    (t2289, t2291)
}
