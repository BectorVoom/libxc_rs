//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1169/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1169(t2365: f64, t56: f64, t2118: f64, t822: f64, t2306: f64, t4383: f64, t2382: f64, t824: f64, t376: f64, t6161: f64, t2169: f64, t2200: f64, t329: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19775 = t2365 * t56;
    let t19776 = t2118 * t19775;
    let t19777 = t822 * t19776;
    let t19894 = t2306 * t4383;
    let t19895 = t2382 * t19894;
    let t19905 = t824 * t19775;
    let t19906 = t822 * t19905;
    let t19911 = t376 * t6161;
    let t20091 = t329 * t2200 * t2169;
    (t19776, t19777, t19895, t19905, t19906, t19911, t20091)
}
