//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 894/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk894(t1105: f64, t2416: f64, t3068: f64, t9283: f64, t2362: f64, t2397: f64, t2408: f64, t2498: f64, t2503: f64, t3052: f64, t3733: f64, t3921: f64, t6778: f64, t827: f64, t8629: f64, t8654: f64, t8671: f64, t8677: f64, t8790: f64, t8793: f64, t9726: f64, t9729: f64, t9899: f64, t9902: f64, t9907: f64, t9912: f64, t9917: f64, t9923: f64, t9928: f64, t9932: f64) -> (f64, f64) {
    let t9941 = t2416 * t1105;
    let t9942 = t9941 * t3068;
    let t9943 = t9283 * t9942;
    let t9946 = -t8671 - t9726 * t3733 / 96.0_f64 - t827 * t9899 / 96.0_f64 - t9902 * t2362 / 48.0_f64 - t9729 * t3733 / 96.0_f64 + t9907 * t6778 / 48.0_f64 + t8677 + t2498 * t2503 / 48.0_f64 + 7.0_f64 / 72.0_f64 * t9912 + t8629 * t9917 / 48.0_f64 + t8629 * t9923 / 96.0_f64 + t2408 * t9928 / 24.0_f64 + t2408 * t9932 / 24.0_f64 + t8793 * t8790 / 24.0_f64 + t3921 * t2397 / 96.0_f64 - t8654 * t3052 / 24.0_f64 - t2408 * t9943 / 12.0_f64;
    (t9942, t9946)
}
