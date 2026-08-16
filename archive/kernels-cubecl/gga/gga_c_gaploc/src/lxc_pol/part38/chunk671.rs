//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 671/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk671<F: Float>(t463: F, t3102: F, t137: F, t4061: F, t135: F, t4074: F, t4077: F, t4082: F, t4085: F, t1247: F, t3103: F, t12380: F, t464: F) -> (F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t12385 = pi * t463;
    let t12386 = t3102 * t12385;
    let t12389 = F::cast_from(1.0_f64) / t137 / t4061;
    let t12390 = t135 * t12389;
    let t12392 = t12390 * t4074 * t4077;
    let t12395 = t4082 * t12390 * t4085;
    let t12397 = t1247 * t3103;
    let t12399 = t464 * t12380;
    (t12385, t12386, t12390, t12392, t12395, t12397, t12399)
}
