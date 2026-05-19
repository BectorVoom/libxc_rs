//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 384/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk384<F: Float>(t203: F, t3191: F, t3190: F, t574: F, t2488: F, t3177: F, t2487: F, t1565: F, t3085: F, t568: F, t3116: F, t600: F) -> (F, F, F, F, F, F, F, F) {
    let t3192 = t3191 * t203;
    let t3193 = t3190 * t3192;
    let t3194 = t574 * t3193;
    let t3196 = t2488 * t3177;
    let t3197 = t2487 * t3196;
    let t3198 = F::cast_from(0.38342925953920749676e0_f64) * t3197;
    let t3199 = t1565 * t3085;
    let t3200 = t568 * t3199;
    let t3203 = t600 * t3116;
    (t3192, t3193, t3194, t3196, t3198, t3199, t3200, t3203)
}
