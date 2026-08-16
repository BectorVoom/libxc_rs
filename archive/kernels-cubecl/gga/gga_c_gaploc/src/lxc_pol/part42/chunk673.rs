//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 673/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk673<F: Float>(t12389: F, t135: F, t4074: F, t4077: F, t4082: F, t4085: F, t1247: F, t3103: F, t12380: F, t464: F, t866: F, t3113: F, t869: F) -> (F, F, F, F, F, F, F) {
    let t12390 = t135 * t12389;
    let t12392 = t12390 * t4074 * t4077;
    let t12395 = t4082 * t12390 * t4085;
    let t12397 = t1247 * t3103;
    let t12399 = t464 * t12380;
    let t12400 = t12399 * t866;
    let t12405 = t869 * t3113;
    (t12390, t12392, t12395, t12397, t12399, t12400, t12405)
}
