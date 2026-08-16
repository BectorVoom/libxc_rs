//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 213/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk213(t598: f64, t610: f64, t186: f64, t185: f64, t202: f64, t209: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t611 = t598 * t610;
    let t612 = t186 * t611;
    let t614 = 2.0_f64 / 15.0_f64 * t185 * t612;
    let t615 = t202 * t209;
    let t616 = t615 * t184;
    (t611, t612, t614, t615, t616)
}
