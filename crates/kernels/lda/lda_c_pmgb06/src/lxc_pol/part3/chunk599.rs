//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 599/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk599<F: Float>(t1039: F, t620: F, t232: F, t1025: F, t632: F, t1042: F, t241: F, t238: F, t1043: F, t28: F, t3500: F, t247: F, t740: F, t934: F, t940: F, t2781: F, t623: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3665 = 1.0 / t1039 / t620;
    let t3666 = t232 * t3665;
    let t3667 = t1025 * t632;
    let t3669 = 1.0 / t1042 / t241;
    let t3670 = t3667 * t3669;
    let t3672 = 517.260129192734 * t3666 * t3670;
    let t3674 = 1.0 / t1039 / t238;
    let t3675 = t232 * t3674;
    let t3676 = t3667 * t1043;
    let t3678 = 96.49187699215521 * t3675 * t3676;
    let t3679 = t3500 * t28;
    let t3680 = t3679 * t247;
    let t3682 = t934 * t740;
    let t3683 = t940 * t3682;
    let t3685 = t623 * t2781;
    (t3665, t3666, t3667, t3669, t3670, t3672, t3674, t3675, t3676, t3678, t3679, t3680, t3682, t3683, t3685)
}
