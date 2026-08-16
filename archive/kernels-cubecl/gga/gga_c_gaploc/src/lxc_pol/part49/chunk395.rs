//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 395/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk395<F: Float>(t3255: F, t738: F, t270: F, t3212: F, t3237: F, t3242: F, t3244: F, t3250: F, t3252: F, t977: F) -> (F, F, F) {
    let t3256 = t738 * t3255;
    let t3259 = F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3212 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3237 + t3242 - F::cast_from(0.23071578690426672851e-1_f64) * t270 * t3244 - t3250 + F::cast_from(0.15381052460284448567e-1_f64) * t270 * t3252 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t3256;
    let t3263 = t977 * t977;
    (t3256, t3259, t3263)
}
