//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 611/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk611<F: Float>(t12385: F, t3102: F, t137: F, t4061: F, t135: F, t4074: F, t4077: F, t4082: F, t4085: F, t1247: F, t3103: F, t12380: F, t464: F, t866: F, t3109: F, t871: F) -> (F, F, F, F, F, F, F, F) {
    let t12386 = t3102 * t12385;
    let t12389 = 1.0 / t137 / t4061;
    let t12390 = t135 * t12389;
    let t12392 = t12390 * t4074 * t4077;
    let t12395 = t4082 * t12390 * t4085;
    let t12397 = t1247 * t3103;
    let t12399 = t464 * t12380;
    let t12400 = t12399 * t866;
    let t12404 = t3109 * t871;
    (t12386, t12390, t12392, t12395, t12397, t12399, t12400, t12404)
}
