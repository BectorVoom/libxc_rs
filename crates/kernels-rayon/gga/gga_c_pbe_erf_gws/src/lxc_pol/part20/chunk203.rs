//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 203/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk203(t418: f64, t572: f64, t571: f64, t11: f64, t570: f64, t173: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t573 = t572 * t418;
    let t574 = t571 * t573;
    let t575 = t11 * t574;
    let t577 = t570 + 0.18891666666666666667e-2_f64 * t575;
    let t578 = t173 * t577;
    let t579 = t578 * t184;
    (t573, t574, t575, t577, t578, t579)
}
