//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 234/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk234(t168: f64, t270: f64, t703: f64, t247: f64, t535: f64, t251: f64, t147: f64, t19: f64, t336: f64) -> (f64, f64, f64, f64, f64) {
    let t706 = 0.19897291109174608293e-1_f64 * t168 * t703 * t270;
    let t707 = t535 * t247;
    let t708 = t707 * t251;
    let t711 = t147 * t19;
    let t712 = t711 * t336;
    (t706, t707, t708, t711, t712)
}
