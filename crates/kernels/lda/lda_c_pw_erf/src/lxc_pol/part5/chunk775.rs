//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 775/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk775<F: Float>(t171: F, t7337: F, t145: F, t169: F, t242: F, t2880: F, t2887: F, t2897: F, t2932: F, t2934: F, t5745: F, t5760: F, t5770: F, t5777: F, t6037: F, t6046: F, t6052: F, t7387: F) -> (F, F) {
    let t7868 = t171 * t7337;
    let t7878 = t2880 - 0.42447554366239165 * t5760 - t2887 + 0.15917832887339686 * t6037 + 0.3183566577467937 * t5777 + t2897 - 0.031835665774679375 * t169 * t7868 * t242 - 0.09550699732403813 * t6046 - 0.09550699732403813 * t5770 - t2932 - t2934 + 0.9598512193592288 * t5745 - 0.31995040645307626 * t6052 + 0.05332506774217938 * t145 * t7387;
    (t7868, t7878)
}
