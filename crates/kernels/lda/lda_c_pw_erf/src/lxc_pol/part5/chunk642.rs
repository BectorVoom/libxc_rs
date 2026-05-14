//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 642/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk642<F: Float>(t145: F, t2906: F, t2932: F, t2934: F, t2935: F, t5745: F, t5750: F, t5768: F, t5770: F, t6046: F, t6052: F, t6080: F, t1553: F, t774: F, t1555: F, t2644: F, t405: F) -> (F, F, F, F) {
    let t6083 = -0.031835665774679375 * t6046 - t5768 - 0.06367133154935875 * t5770 - 0.031835665774679375 * t2906 - t2932 - t2934 + 0.31995040645307626 * t2935 + 0.6399008129061525 * t5745 - t5750 - 0.10665013548435875 * t6052 + 0.05332506774217938 * t145 * t6080;
    let t6086 = t1553 * t774;
    let t6087 = t6086 * t1555;
    let t6089 = t405 * t2644;
    (t6083, t6086, t6087, t6089)
}
