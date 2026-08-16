//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1859/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1859(t1014: f64, t65: f64, t4579: f64, t3252: f64, t4574: f64, t3204: f64, t7131: f64) -> (f64, f64, f64, f64, f64) {
    let t27527 = t65 * t1014;
    let t27528 = t27527 * t4579;
    let t27531 = t65 * t3252;
    let t27532 = t27531 * t4574;
    let t27536 = t3204 * t7131;
    (t27527, t27528, t27531, t27532, t27536)
}
