//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1162/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1162<F: Float>(t2057: F, t6012: F, t17: F, t2022: F, t697: F, t2053: F, t140: F, t19746: F, t35: F, t2062: F, t150: F, t168: F) -> (F, F, F, F, F, F) {
    let t20282 = t6012 * t2057;
    let t20290 = t17 / t697 / t2022;
    let t20291 = t2053 * t2053;
    let t20292 = F::cast_from(1.0_f64) / t20291;
    let t20346 = F::cast_from(140.0_f64) / F::cast_from(729.0_f64) * t35 * t19746 * t140;
    let t20355 = t6012 * t2062;
    let t20396 = F::cast_from(1.0_f64) / t168 / t150;
    (t20282, t20290, t20292, t20346, t20355, t20396)
}
