//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 561/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk561<F: Float>(t176: F, t2943: F, t166: F, t161: F, t1426: F, t464: F) -> (F, F, F, F) {
    let t2944 = t2943 * t176;
    let t2945 = t166 * t2944;
    let t2947 = t161 * t2945 / F::new(30.0);
    let t2948 = t1426 * t464;
    (t2944, t2945, t2947, t2948)
}
