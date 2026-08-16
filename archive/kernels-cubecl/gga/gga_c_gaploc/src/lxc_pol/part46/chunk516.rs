//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 516/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk516<F: Float>(t188: F, t9189: F, t3085: F, t4130: F, t590: F, t1339: F, t3116: F, t3196: F, t7014: F, t2488: F, t9278: F, t2487: F) -> (F, F, F, F, F, F) {
    let t9351 = t188 * t9189;
    let t9354 = t4130 * t3085;
    let t9355 = t9354 * t590;
    let t9358 = t1339 * t3116;
    let t9359 = t9358 * t590;
    let t9362 = t7014 * t3196;
    let t9363 = F::cast_from(0.38342925953920749676e0_f64) * t9362;
    let t9364 = t2488 * t9278;
    let t9365 = t2487 * t9364;
    (t9351, t9354, t9355, t9359, t9363, t9365)
}
