//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 610/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk610<F: Float>(t12290: F, t12315: F, t12317: F, t12321: F, t883: F, t9198: F, t2325: F, t882: F, t2321: F, t3152: F, t3148: F, t135: F, t139: F, t145: F, t459: F, t463: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12323 = t12290 + t12315 + t12317 + t12321;
    let t12351 = t883 * t9198;
    let t12352 = t2325 * t12351;
    let t12353 = t882 * t12352;
    let t12360 = t3152 * t2321;
    let t12361 = t882 * t12360;
    let t12366 = t3148 * t2321;
    let t12367 = t882 * t12366;
    let t12380 = 1.0 / t135;
    let t12381 = t12380 * t139;
    let t12383 = t12381 * t145 * t459;
    let t12385 = M_PI * t463;
    (t12323, t12352, t12353, t12360, t12361, t12366, t12367, t12380, t12381, t12383, t12385)
}
