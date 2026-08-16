//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 348/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk348(t1044: f64, t650: f64, t186: f64, t211: f64, t225: f64, t991: f64) -> (f64, f64, f64, f64) {
    let t1045 = t650 * t1044;
    let t1046 = t186 * t1045;
    let t1048 = 2.0_f64 / 15.0_f64 * t211 * t1046;
    let t1049 = t991 * t225;
    (t1045, t1046, t1048, t1049)
}
