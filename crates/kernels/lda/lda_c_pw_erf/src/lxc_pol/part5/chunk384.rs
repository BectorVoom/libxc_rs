//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 384/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk384<F: Float>(t1: F, t1750: F, t305: F, t152: F, t6: F, t1124: F, t279: F, t19: F, t726: F, t729: F, t748: F, t75: F) -> (F, F, F, F, F) {
    let t1752 = t305 * t1750 * t1;
    let t1753 = t152 * t6;
    let t1755 = t1753 * t1124 * t279;
    let t1759 = t726 * t729 * t19;
    let t1765 = t748 * t75;
    (t1752, t1753, t1755, t1759, t1765)
}
