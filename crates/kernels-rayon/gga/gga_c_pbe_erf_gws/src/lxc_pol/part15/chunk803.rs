//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 803/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk803(t2365: f64, t828: f64, t2137: f64, t2134: f64, t2132: f64, t2271: f64, t822: f64, t362: f64, t922: f64, t2276: f64, t932: f64, t2315: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6183 = t2365 * t828;
    let t6184 = t6183 * t2137;
    let t6185 = t2134 * t6184;
    let t6187 = t2271 * t2132;
    let t6188 = t822 * t6187;
    let t6201 = t362 * t922;
    let t6203 = t2276 * t6201 * t932;
    let t6204 = t6203 * t2315;
    (t6183, t6184, t6185, t6187, t6188, t6201, t6203, t6204)
}
