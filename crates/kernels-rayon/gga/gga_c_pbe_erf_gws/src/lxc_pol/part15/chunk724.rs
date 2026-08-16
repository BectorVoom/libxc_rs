//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 724/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk724(t2200: f64, t329: f64, t340: f64, t847: f64, t2366: f64, t2387: f64, t833: f64, t2395: f64, t814: f64, t829: f64, t830: f64, t2100: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4442 = t329 * t2200 * t340;
    let t4443 = t4442 * t847;
    let t4453 = t2387 * t2366;
    let t4454 = t4453 * t833;
    let t4459 = t829 * t830 * t2395 * t814;
    let t4464 = t829 * t830 * t831 * t2100;
    (t4442, t4443, t4453, t4454, t4459, t4464)
}
