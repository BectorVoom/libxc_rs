//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 942/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk942(t2522: f64, t299: f64, t169: f64, t242: f64, t2994: f64, t700: f64, t784: f64, t991: f64, t171: f64, t7908: f64, t2998: f64, t1086: f64, t1383: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8352 = t299 * t2522;
    let t8355 = 0.1061188859155979109e0_f64 * t169 * t8352 * t242;
    let t8357 = t169 * t2994 * t700;
    let t8361 = t784 * t991;
    let t8363 = t169 * t8361 * t242;
    let t8365 = t171 * t7908;
    let t8371 = 0.63671331549358746542e-1_f64 * t169 * t2998 * t700;
    let t8373 = t169 * t1086 * t1383;
    (t8355, t8357, t8361, t8363, t8365, t8371, t8373)
}
