//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 868/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk868(t7341: f64, t7435: f64, t587: f64, t1407: f64, t2565: f64, t1827: f64, t1017: f64, t1663: f64, t1403: f64, t5543: f64, t1416: f64, t2570: f64) -> (f64, f64, f64, f64) {
    let t7436 = t7435 * t7341;
    let t7438 = 32.0_f64 / 81.0_f64 * t587 * t7436;
    let t7439 = t2565 * t1407;
    let t7440 = t1827 * t7439;
    let t7442 = 4.0_f64 / 45.0_f64 * t587 * t7440;
    let t7443 = t1017 * t1663;
    let t7444 = t7443 * t1403;
    let t7445 = t5543 * t7444;
    let t7447 = 4.0_f64 / 27.0_f64 * t587 * t7445;
    let t7448 = t2570 * t1416;
    (t7438, t7442, t7447, t7448)
}
