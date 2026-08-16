//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1348/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1348(t35928: f64, t35930: f64, t35932: f64, t35934: f64, t35938: f64, t35940: f64, t35943: f64, t35945: f64, t35948: f64, t35954: f64, t35956: f64, t35959: f64, t35962: f64) -> f64 {
    let t36218 = -0.32293198289056946716e-4_f64 * t35928 - 0.14226130163765189728e-3_f64 * t35930 + 0.32293198289056946716e-4_f64 * t35932 + 0.38974171724179661463e-4_f64 * t35934 - 0.43637343375932385357e-7_f64 * t35938 - 0.83516082266099274564e-5_f64 * t35940 - 0.83516082266099274564e-5_f64 * t35943 + 0.22798285518854470718e-6_f64 * t35945 + 0.10943177049050145945e-4_f64 * t35948 + 0.12487111080837992338e-6_f64 * t35954 - 0.10943177049050145945e-4_f64 * t35956 + 0.23485962392041415794e-4_f64 * t35959 + 0.46971924784082831588e-4_f64 * t35962;
    t36218
}
