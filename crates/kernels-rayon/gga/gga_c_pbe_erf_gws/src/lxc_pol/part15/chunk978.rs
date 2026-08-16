//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 978/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk978(t1105: f64, t898: f64, t938: f64, t353: f64, t4386: f64, t1115: f64, t2384: f64, t3047: f64, t3052: f64, t3079: f64, t335: f64, t4385: f64, t4475: f64, t4477: f64, t6135: f64, t6151: f64, t6789: f64, t6793: f64, t827: f64, t8671: f64, t8677: f64, t8685: f64, t8690: f64, t8695: f64, t8700: f64, t8705: f64, t8710: f64) -> (f64, f64, f64) {
    let t8713 = t898 * t1105;
    let t8714 = t8713 * t938;
    let t8715 = t353 * t8714;
    let t8716 = t4386 * t8715;
    let t8721 = -t8671 - t1115 * t6135 / 24.0_f64 - t1115 * t6789 / 48.0_f64 + t8677 + t1115 * t6151 / 16.0_f64 - t2384 * t3047 / 96.0_f64 - t2384 * t3052 / 48.0_f64 - t335 * t8685 / 48.0_f64 + t4385 * t8690 / 96.0_f64 + t6793 * t8695 / 24.0_f64 + t4385 * t8700 / 48.0_f64 + t8705 * t3079 / 48.0_f64 - t827 * t8710 / 24.0_f64 + t6793 * t8716 / 24.0_f64 - 7.0_f64 / 288.0_f64 * t4475 - 7.0_f64 / 288.0_f64 * t4477;
    (t8713, t8716, t8721)
}
