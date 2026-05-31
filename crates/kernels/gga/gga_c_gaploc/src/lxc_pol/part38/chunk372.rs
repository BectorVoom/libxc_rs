//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 372/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk372<F: Float>(t130: F, t139: F, t145: F, t459: F, t136: F, t453: F, t129: F, t1242: F, t1247: F, t1240: F, t464: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t3091 = F::cast_from(1.0_f64) / t130;
    let t3092 = t3091 * t139;
    let t3094 = t3092 * t145 * t459;
    let t3096 = t453 * t136;
    let t3097 = F::cast_from(1.0_f64) / t3096;
    let t3098 = t129 * t3097;
    let t3099 = t3098 * t1242;
    let t3101 = t1247 * t129;
    let t3102 = t3097 * t1240;
    let t3103 = t3102 * pi;
    let t3104 = t3101 * t3103;
    let t3106 = t464 * t3091;
    (t3091, t3092, t3094, t3096, t3097, t3098, t3099, t3101, t3102, t3103, t3104, t3106)
}
