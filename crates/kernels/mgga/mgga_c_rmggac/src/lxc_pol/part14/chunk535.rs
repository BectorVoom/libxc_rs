//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 535/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk535<F: Float>(t321: F, t498: F, t236: F, t7248: F, t3351: F, t1965: F, t2189: F, t1969: F) -> (F, F, F, F) {
    let t7249 = t498 * t321;
    let t7250 = t236 * t7249;
    let t7251 = t7248 * t7250;
    let t7252 = t3351 * t7251;
    let t7253 = F::new(0.25538759935978703638e-4) * t7252;
    let t7254 = t2189 * t1965;
    let t7255 = t7254 * t1969;
    (t7251, t7253, t7254, t7255)
}
