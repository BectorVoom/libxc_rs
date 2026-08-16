//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 403/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk403(t1396: f64, t470: f64, t427: f64, t75: f64, t472: f64, t92: f64, t418: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1397 = t470 * t1396;
    let t1398 = 0.58482233974552040708e0_f64 * t1397;
    let t1399 = t427 * t75;
    let t1400 = t1399 * t472;
    let t1401 = 0.11696446794910408142e1_f64 * t1400;
    let t1402 = 1.0_f64 / t92;
    let t1403 = t418 * t418;
    (t1398, t1399, t1400, t1401, t1402, t1403)
}
