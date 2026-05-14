//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 609/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk609<F: Float>(t12290: F, t12315: F, t12317: F, t12321: F, t135: F, t139: F, t145: F, t459: F, t463: F, t3102: F, t137: F, t4061: F, t4074: F, t4077: F, t4082: F, t4085: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12323 = t12290 + t12315 + t12317 + t12321;
    let t12380 = 1.0 / t135;
    let t12381 = t12380 * t139;
    let t12383 = t12381 * t145 * t459;
    let t12385 = M_PI * t463;
    let t12386 = t3102 * t12385;
    let t12389 = 1.0 / t137 / t4061;
    let t12390 = t135 * t12389;
    let t12392 = t12390 * t4074 * t4077;
    let t12395 = t4082 * t12390 * t4085;
    (t12323, t12380, t12381, t12383, t12385, t12386, t12390, t12392, t12395)
}
