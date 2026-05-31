//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 672/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk672<F: Float>(t1645: F, t2792: F, t2963: F, t883: F, t9198: F, t2325: F, t882: F, t2321: F, t3152: F, t3148: F, t135: F, t139: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11392 = t1645 * t2792;
    let t11807 = t1645 * t2963;
    let t12351 = t883 * t9198;
    let t12352 = t2325 * t12351;
    let t12353 = t882 * t12352;
    let t12360 = t3152 * t2321;
    let t12361 = t882 * t12360;
    let t12366 = t3148 * t2321;
    let t12367 = t882 * t12366;
    let t12380 = F::cast_from(1.0_f64) / t135;
    let t12381 = t12380 * t139;
    (t11392, t11807, t12352, t12353, t12360, t12361, t12366, t12367, t12380, t12381)
}
