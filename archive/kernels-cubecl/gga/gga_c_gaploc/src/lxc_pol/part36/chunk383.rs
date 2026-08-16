//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 383/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk383<F: Float>(t3177: F, t912: F, t587: F, t3085: F, t600: F, t568: F, t3116: F, t569: F, t1201: F, t124: F, t60: F, t1390: F, t40: F) -> (F, F, F, F, F, F, F, F) {
    let t3178 = t912 * t3177;
    let t3179 = t587 * t3178;
    let t3180 = F::cast_from(0.38342925953920749676e0_f64) * t3179;
    let t3181 = t600 * t3085;
    let t3182 = t568 * t3181;
    let t3185 = t569 * t3116;
    let t3186 = t568 * t3185;
    let t3190 = t60 * t1201 * t124;
    let t3191 = t1390 * t40;
    (t3178, t3180, t3181, t3182, t3185, t3186, t3190, t3191)
}
