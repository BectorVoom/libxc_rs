//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 792/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk792<F: Float>(t16502: F, t1406: F, t181: F, t184: F, t199: F, t1885: F, t5273: F, t5393: F, t587: F, t16481: F, t16485: F, t16487: F, t16490: F, t16492: F, t16494: F, t16498: F, t16501: F) -> (F, F, F, F) {
    let t16503 = 8.0 / 45.0 * t16502;
    let t16504 = t1406 * t1406;
    let t16508 = 4.0 / 5.0 * t16504 * t181 * t184 * t199;
    let t16512 = 16.0 / 15.0 * t587 * t1885 * t5393 * t5273;
    let t16513 = -0.38474813732852776452e0 * t16481 + t16485 - t16487 - t16490 + 0.67090456446662028936e-1 * t16492 - 0.44726970964441352624e-1 * t16494 + t16498 - t16501 - t16503 + t16508 + t16512;
    (t16503, t16508, t16512, t16513)
}
