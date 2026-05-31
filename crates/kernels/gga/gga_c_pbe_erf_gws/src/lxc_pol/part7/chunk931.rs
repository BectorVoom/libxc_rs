//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 931/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk931<F: Float>(t5352: F, t636: F, t4923: F, t5218: F, t5220: F, t5334: F, t561: F, t582: F, t1403: F, t1406: F, t1663: F, t1820: F, t2559: F) -> (F, F, F, F) {
    let t17390 = t5352 * t636;
    let t17391 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t17390;
    let t17394 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t5218 * t5220 * t4923;
    let t17396 = t561 * t582 * t5334;
    let t17397 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t17396;
    let t17402 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1820 * t2559 * t1406 * t1663 * t1403;
    (t17391, t17394, t17397, t17402)
}
