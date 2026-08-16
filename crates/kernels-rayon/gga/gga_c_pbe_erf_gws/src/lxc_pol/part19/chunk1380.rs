//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1380/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1380(t55473: f64, t55480: f64, t55482: f64, t56929: f64, t56931: f64, t56933: f64, t56935: f64, t56938: f64, t56940: f64, t56943: f64, t56945: f64, t56947: f64, t56949: f64) -> f64 {
    let t58630 = t56929 / 48.0_f64 + t56931 / 48.0_f64 + t56933 / 48.0_f64 - 7.0_f64 / 576.0_f64 * t56935 + t56938 / 8.0_f64 + t55473 - 7.0_f64 / 144.0_f64 * t56940 - t56943 / 6.0_f64 + t55480 + t55482 - t56945 / 48.0_f64 - 5.0_f64 / 32.0_f64 * t56947 - t56949 / 24.0_f64;
    t58630
}
