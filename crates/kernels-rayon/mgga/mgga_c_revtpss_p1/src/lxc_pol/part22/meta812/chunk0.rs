//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2916/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2916(t4066: f64, t4086: f64, t786: f64, t10022: f64, t2453: f64, t281: f64, t4003: f64, t46507: f64, t268: f64, t39644: f64, t546: f64, t555: f64, t8779: f64) -> (f64, f64, f64, f64) {
    let t47423 = t786 * t4086 * t4066;
    let t47429 = t2453 * t10022;
    let t47432 = t47429 * t281 * t46507 * t4003;
    let t47442 = 0.11638313500518478545e-4_f64 * t39644 * t546 * t555 * t8779 * t268;
    (t47423, t47429, t47432, t47442)
}
