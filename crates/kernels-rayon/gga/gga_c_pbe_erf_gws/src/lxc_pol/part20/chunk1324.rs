//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1324/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1324(t12084: f64, t4028: f64, t11915: f64, t4049: f64, t11981: f64, t54103: f64, t54114: f64, t54118: f64, t56929: f64, t56931: f64, t56933: f64, t56935: f64, t56938: f64, t56940: f64, t56943: f64) -> f64 {
    let t56945 = t4028 * t12084;
    let t56947 = t4049 * t11915;
    let t56949 = t4028 * t11981;
    let t56951 = t56929 / 96.0_f64 + t56931 / 96.0_f64 + t56933 / 96.0_f64 - 7.0_f64 / 1152.0_f64 * t56935 + t56938 / 16.0_f64 + t54103 - 7.0_f64 / 288.0_f64 * t56940 - t56943 / 12.0_f64 + t54114 + t54118 - t56945 / 96.0_f64 - 5.0_f64 / 64.0_f64 * t56947 - t56949 / 48.0_f64;
    t56951
}
