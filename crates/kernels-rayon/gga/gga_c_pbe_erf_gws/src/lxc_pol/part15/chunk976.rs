//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 976/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk976(t2366: f64, t3039: f64, t833: f64, t2367: f64, t3047: f64, t3200: f64, t338: f64, t939: f64, t1162: f64, t814: f64, t353: f64, t859: f64) -> (f64, f64, f64, f64, f64) {
    let t8669 = t3039 * t2366;
    let t8671 = 7.0_f64 / 144.0_f64 * t8669 * t833;
    let t8677 = 7.0_f64 / 144.0_f64 * t2367 * t3047;
    let t8685 = t338 * t3200 * t939;
    let t8688 = t1162 * t814;
    let t8689 = t353 * t8688;
    let t8690 = t859 * t8689;
    (t8669, t8671, t8677, t8685, t8690)
}
