//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 527/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk527(t2348: f64, t2349: f64, t73: f64, t799: f64, t26: f64, t66: f64) -> (f64, f64, f64) {
    let t2351 = 0.10843581300301739842e-1_f64 * t2348 * t2349;
    let t2357 = t73 * t799;
    let t2376 = 1.0_f64 / t66 / t26;
    (t2351, t2357, t2376)
}
