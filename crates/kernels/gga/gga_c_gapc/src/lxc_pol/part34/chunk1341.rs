//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1341/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1341<F: Float>(t35928: F, t35930: F, t35932: F, t35934: F, t35938: F, t35940: F, t35943: F, t35945: F, t35948: F, t35954: F, t35956: F, t35959: F, t35962: F) -> F {
    let t36218 = -F::cast_from(0.32293198289056946716e-4_f64) * t35928 - F::cast_from(0.14226130163765189728e-3_f64) * t35930 + F::cast_from(0.32293198289056946716e-4_f64) * t35932 + F::cast_from(0.38974171724179661463e-4_f64) * t35934 - F::cast_from(0.43637343375932385357e-7_f64) * t35938 - F::cast_from(0.83516082266099274564e-5_f64) * t35940 - F::cast_from(0.83516082266099274564e-5_f64) * t35943 + F::cast_from(0.22798285518854470718e-6_f64) * t35945 + F::cast_from(0.10943177049050145945e-4_f64) * t35948 + F::cast_from(0.12487111080837992338e-6_f64) * t35954 - F::cast_from(0.10943177049050145945e-4_f64) * t35956 + F::cast_from(0.23485962392041415794e-4_f64) * t35959 + F::cast_from(0.46971924784082831588e-4_f64) * t35962;
    t36218
}
