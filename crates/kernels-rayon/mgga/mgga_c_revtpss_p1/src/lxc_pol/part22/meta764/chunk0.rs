//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2846/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2846(t16409: f64, t994: f64, t3057: f64, t4980: f64, t11223: f64, t3286: f64, t11200: f64, t11213: f64, t3046: f64, t4995: f64, t3143: f64, t42859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43432 = t994 * t16409;
    let t43438 = t3057 * t4980;
    let t43443 = t11223 * t3286;
    let t43446 = t11200 * t3286;
    let t43450 = t11213 * t3286;
    let t43453 = t3046 * t4995;
    let t43456 = t3057 * t4995;
    let t43471 = t42859 * t3143;
    (t43432, t43438, t43443, t43446, t43450, t43453, t43456, t43471)
}
