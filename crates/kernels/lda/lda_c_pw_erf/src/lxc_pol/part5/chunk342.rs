//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 342/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk342<F: Float>(t1260: F, t190: F, t212: F, t331: F, t590: F, t204: F, t205: F, t191: F) -> (F, F, F, F) {
    let t1366 = 0.011111111111111112 * t190 * t1260 * t212;
    let t1367 = t331 * t590;
    let t1370 = 1.0 / t205 / t204;
    let t1371 = t191 * t1370;
    (t1366, t1367, t1370, t1371)
}
