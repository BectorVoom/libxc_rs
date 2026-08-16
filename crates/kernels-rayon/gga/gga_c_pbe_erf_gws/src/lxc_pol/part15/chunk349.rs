//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 349/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk349(t1099: f64, t386: f64, t407: f64, t411: f64, t416: f64, t462: f64, t469: f64, t474: f64, t803: f64, t961: f64, t963: f64) -> f64 {
    let t1109 = t386 + t407 - t411 - t416 + t961 + t462 + t963 - t469 - t474 - t1099 - t803;
    t1109
}
