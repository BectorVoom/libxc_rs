//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 632/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk632<F: Float>(t232: F, t3674: F, t1043: F, t3667: F, t28: F, t3500: F, t247: F, t740: F, t934: F, t940: F, t2781: F, t623: F) -> (F, F, F, F, F, F, F, F) {
    let t3675 = t232 * t3674;
    let t3676 = t3667 * t1043;
    let t3678 = F::new(96.49187699215521) * t3675 * t3676;
    let t3679 = t3500 * t28;
    let t3680 = t3679 * t247;
    let t3682 = t934 * t740;
    let t3683 = t940 * t3682;
    let t3685 = t623 * t2781;
    (t3675, t3676, t3678, t3679, t3680, t3682, t3683, t3685)
}
