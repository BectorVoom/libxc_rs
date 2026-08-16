//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 229/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk229(t650: f64, t661: f64, t186: f64, t211: f64, t225: f64, t535: f64) -> (f64, f64, f64, f64) {
    let t662 = t650 * t661;
    let t663 = t186 * t662;
    let t665 = 2.0_f64 / 15.0_f64 * t211 * t663;
    let t666 = t535 * t225;
    (t662, t663, t665, t666)
}
