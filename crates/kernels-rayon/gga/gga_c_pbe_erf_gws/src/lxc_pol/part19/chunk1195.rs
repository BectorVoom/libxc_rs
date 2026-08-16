//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1195/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1195(t2416: f64, t360: f64, t4383: f64, t4408: f64, t2365: f64, t56: f64, t2118: f64, t2306: f64, t4395: f64, t824: f64, t822: f64, t2169: f64, t2200: f64, t329: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15641 = t2416 * t360;
    let t19658 = t4408 * t4383;
    let t19775 = t2365 * t56;
    let t19776 = t2118 * t19775;
    let t19894 = t2306 * t4383;
    let t19898 = t4395 * t4383;
    let t19905 = t824 * t19775;
    let t19906 = t822 * t19905;
    let t20091 = t329 * t2200 * t2169;
    (t15641, t19658, t19776, t19894, t19898, t19905, t19906, t20091)
}
