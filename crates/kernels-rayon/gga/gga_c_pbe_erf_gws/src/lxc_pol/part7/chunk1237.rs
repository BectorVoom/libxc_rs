//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1237/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1237(t2074: f64, t2395: f64, t2370: f64, t830: f64, t3074: f64, t4473: f64, t6772: f64, t6130: f64, t840: f64, t19505: f64, t19553: f64, t19602: f64, t21819: f64, t21823: f64, t21826: f64, t21830: f64, t2306: f64, t2373: f64, t2376: f64, t2401: f64, t2409: f64, t2410: f64, t3066: f64, t3067: f64, t3079: f64, t3207: f64, t328: f64, t338: f64, t353: f64, t376: f64, t4427: f64, t6104: f64, t6385: f64, t6723: f64, t827: f64, t844: f64, t9241: f64, t9283: f64, t938: f64) -> f64 {
    let t21832 = t2395 * t2074;
    let t21834 = t2370 * t830 * t21832;
    let t21845 = t3074 * t4473 * t6772;
    let t21847 = t840 * t6130;
    let t21867 = -t844 * t338 * t353 * t376 * t19553 / 48.0_f64 - 7.0_f64 / 24.0_f64 * t21819 + t21823 + 7.0_f64 / 3.0_f64 * t21826 - 7.0_f64 / 72.0_f64 * t21830 - t827 * t21834 / 4.0_f64 - t4427 * t2373 / 6.0_f64 + t3074 * t2306 * t6104 * t328 * t3079 / 24.0_f64 - 7.0_f64 / 24.0_f64 * t21845 + 7.0_f64 / 12.0_f64 * t21847 + 3.0_f64 / 16.0_f64 * t2401 * t338 * t353 * t376 * t19505 - 3.0_f64 / 4.0_f64 * t3207 * t9283 * t19602 * t2410 + t3066 * t2409 * t3067 * t6723 * t938 / 12.0_f64 + t9241 * t2409 * t2376 * t6385 * t938;
    t21867
}
