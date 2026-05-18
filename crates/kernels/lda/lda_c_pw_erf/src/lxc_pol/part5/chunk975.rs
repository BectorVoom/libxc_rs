//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 975/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk975<F: Float>(t197: F, t3892: F, t473: F, t10527: F, t219: F, t10605: F, t1944: F, t571: F, t9408: F, t10162: F, t1325: F, t2167: F) -> (F, F, F, F, F) {
    let t14205 = t473 * t3892 * t197;
    let t14240 = t10527 * t219;
    let t14255 = t571 * t10605 * t219 * t1944;
    let t14256 = F::new(8.0) / F::new(81.0) * t14255;
    let t14257 = t9408 * t219;
    let t14313 = t1325 * t10162 * t2167;
    (t14205, t14240, t14256, t14257, t14313)
}
