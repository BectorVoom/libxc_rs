//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 602/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk602<F: Float>(t10409: F, t9263: F, t9422: F, t10381: F, t10384: F, t10387: F, t10388: F, t10394: F, t10395: F, t10398: F, t10401: F, t10404: F, t10406: F, t1537: F, t567: F, t9363: F, t9366: F, t9370: F) -> F {
    let t10410 = t9263 * t10409;
    let t10411 = F::cast_from(0.38342925953920749676e0_f64) * t10410;
    let t10412 = F::cast_from(0.63904876589867916128e-1_f64) * t9422;
    let t10413 = t10381 + t10384 + t10387 + F::cast_from(0.23005755572352449806e1_f64) * t567 * t10388 + t9363 + t9366 - t9370 + t10394 - t10395 - t10398 + t10401 - t10404 - F::cast_from(0.51123901271894332902e0_f64) * t1537 * t10406 - t10411 + t10412;
    t10413
}
