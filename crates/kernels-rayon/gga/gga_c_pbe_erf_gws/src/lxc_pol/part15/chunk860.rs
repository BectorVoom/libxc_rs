//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 860/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk860(t1663: f64, t34: f64, t418: f64, t1856: f64, t1407: f64, t2554: f64, t606: f64, t1764: f64, t1403: f64, t2560: f64, t2778: f64, t401: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7345 = t1663 * t34;
    let t7346 = t7345 * t418;
    let t7347 = t1856 * t7346;
    let t7350 = t2554 * t1407;
    let t7351 = t606 * t7350;
    let t7354 = t1764 * t34;
    let t7355 = t7354 * t418;
    let t7356 = t606 * t7355;
    let t7359 = t2560 * t1403;
    let t7360 = t606 * t7359;
    let t7364 = 0.17777777777777777778e-1_f64 * t401 * t2778;
    (t7346, t7347, t7350, t7351, t7355, t7356, t7359, t7360, t7364)
}
