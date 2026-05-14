//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1093/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1093<F: Float>(t11: F, t21240: F, t3536: F, t11829: F, t11834: F, t11846: F, t11848: F, t15838: F, t15848: F, t15850: F, t15887: F, t17327: F, t17332: F, t2061: F, t22713: F, t22717: F, t22722: F, t22725: F, t22728: F, t22731: F, t22734: F, t22737: F, t22740: F, t25: F, t3516: F, t9761: F) -> (F, F) {
    let t22743 = t11 * t3536 * t21240;
    let t22745 = -0.2879333333333333 * t15838 - 0.07198333333333333 * t15848 + 0.023994444444444443 * t15850 + 0.05925925925925926 * t11829 + 0.11197407407407407 * t11834 - 0.044444444444444446 * t11846 - 0.007407407407407408 * t17327 + 0.044444444444444446 * t17332 - 0.09597777777777777 * t11848 - 0.03199259259259259 * t15887 - 0.006913580246913581 * t25 * t9761 * t22713 - 0.017777777777777778 * t2061 * t3516 * t22717 - 0.07198333333333333 * t22722 - 0.14396666666666666 * t22725 + 0.8638 * t22728 - 1.2957 * t22731 + 0.21595 * t22734 + 0.4319 * t22737 - 0.8638 * t22740 + 0.47988888888888886 * t22743;
    (t22743, t22745)
}
