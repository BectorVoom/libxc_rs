//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 851/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk851(t219: f64, t7209: f64, t1642: f64, t34: f64, t422: f64, t639: f64, t1621: f64, t1791: f64, t1044: f64, t617: f64, t661: f64, t1620: f64) -> (f64, f64, f64) {
    let t7210 = t7209 * t219;
    let t7211 = t1642 * t34;
    let t7212 = t7211 * t422;
    let t7213 = t7210 * t7212;
    let t7215 = 16.0_f64 / 27.0_f64 * t639 * t7213;
    let t7216 = t1621 * t1791;
    let t7217 = t1044 * t617;
    let t7218 = t7217 * t661;
    let t7219 = t7216 * t7218;
    let t7221 = 16.0_f64 / 15.0_f64 * t1620 * t7219;
    (t7212, t7215, t7221)
}
