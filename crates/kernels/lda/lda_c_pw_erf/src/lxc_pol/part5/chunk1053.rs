//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1053/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1053<F: Float>(t242: F, t6138: F, t2594: F, t2765: F, t440: F, t7199: F, t7191: F, t7158: F, t925: F, t7161: F, t1686: F, t2624: F, t933: F) -> (F, F, F, F, F, F, F) {
    let t19397 = t6138 * t242;
    let t19421 = t2765 * t2594 * t440;
    let t19425 = t2765 * t7199;
    let t19449 = t2765 * t7191;
    let t19516 = t7158 * t925;
    let t19518 = t7161 * t925;
    let t19523 = t1686 * t2624 * t933;
    (t19397, t19421, t19425, t19449, t19516, t19518, t19523)
}
