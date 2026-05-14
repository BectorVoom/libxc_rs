//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 355/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk355<F: Float>(t3177: F, t912: F, t587: F, t1201: F, t124: F, t60: F, t1390: F, t40: F, t203: F, t574: F, t2488: F, t2487: F, t286: F, t3092: F, t708: F, t1687: F, t3098: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3178 = t912 * t3177;
    let t3179 = t587 * t3178;
    let t3180 = 0.38342925953920749676e0 * t3179;
    let t3190 = t60 * t1201 * t124;
    let t3191 = t1390 * t40;
    let t3192 = t3191 * t203;
    let t3193 = t3190 * t3192;
    let t3194 = t574 * t3193;
    let t3196 = t2488 * t3177;
    let t3197 = t2487 * t3196;
    let t3198 = 0.38342925953920749676e0 * t3197;
    let t3216 = t3092 * t286 * t708;
    let t3218 = t3098 * t1687;
    (t3178, t3180, t3190, t3191, t3192, t3193, t3194, t3196, t3198, t3216, t3218)
}
