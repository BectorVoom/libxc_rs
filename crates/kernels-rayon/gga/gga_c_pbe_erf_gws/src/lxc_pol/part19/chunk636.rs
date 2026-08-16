//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 636/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk636(t1758: f64, t3421: f64, t11: f64, t1764: f64, t3342: f64, t571: f64, t3346: f64, t572: f64, t1856: f64, t606: f64, t1844: f64, t1851: f64, t25: f64, t2760: f64, t2773: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3422 = t1758 * t3421;
    let t3423 = t11 * t3422;
    let t3425 = t1764 * t3342;
    let t3426 = t571 * t3425;
    let t3427 = t11 * t3426;
    let t3429 = t572 * t3346;
    let t3430 = t571 * t3429;
    let t3431 = t11 * t3430;
    let t3434 = t1856 * t3421;
    let t3437 = t606 * t3425;
    let t3440 = t606 * t3429;
    let t3443 = t1844 + 0.23994444444444444444e-1_f64 * t2760 - 0.23994444444444444445e-1_f64 * t3423 + 0.71983333333333333334e-1_f64 * t3427 - 0.35991666666666666667e-1_f64 * t3431 + t1851 + 0.8888888888888888889e-2_f64 * t2773 - 0.22222222222222222222e-2_f64 * t25 * t3434 + 0.13333333333333333333e-1_f64 * t25 * t3437 - 0.66666666666666666667e-2_f64 * t25 * t3440;
    (t3422, t3423, t3425, t3426, t3427, t3429, t3430, t3431, t3434, t3437, t3440, t3443)
}
