//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 420/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk420<F: Float>(t3451: F, t738: F, t270: F, t3242: F, t3250: F, t3422: F, t3434: F, t3439: F, t3442: F, t3446: F, t3450: F, t1052: F, t977: F) -> (F, F, F) {
    let t3452 = t738 * t3451;
    let t3455 = t3422 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3434 + t3439 - t3442 + t3242 - t3250 - t3446 + t3450 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3452;
    let t3459 = t1052 * t977;
    (t3452, t3455, t3459)
}
