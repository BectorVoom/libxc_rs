//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 241/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk241(t108: f64, t418: f64, t422: f64, t726: f64, t728: f64, t266: f64, t9: f64) -> (f64, f64) {
    let t732 = (4.0_f64 / 3.0_f64 * t726 * t418 + 4.0_f64 / 3.0_f64 * t728 * t422) * t108;
    let t735 = t266 * t9;
    (t732, t735)
}
