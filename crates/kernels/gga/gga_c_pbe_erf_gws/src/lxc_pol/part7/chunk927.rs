//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 927/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk927<F: Float>(t1829: F, t5406: F, t2735: F, t561: F, t563: F, t1730: F, t5116: F, t17316: F, t17318: F, t17326: F, t17328: F, t17330: F, t17335: F, t17338: F, t17341: F) -> (F, F, F, F) {
    let t17343 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t5406 * t1829;
    let t17345 = t561 * t2735 * t563;
    let t17346 = F::cast_from(128.0_f64) / F::cast_from(405.0_f64) * t17345;
    let t17347 = t1730 * t5116;
    let t17348 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t17347;
    let t17349 = t17316 + t17318 + t17326 + t17328 + t17330 - t17335 + t17338 + t17341 - t17343 + t17346 - t17348;
    (t17343, t17346, t17348, t17349)
}
