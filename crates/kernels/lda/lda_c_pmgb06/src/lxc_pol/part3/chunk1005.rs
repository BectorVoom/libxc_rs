//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1005/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1005<F: Float>(t1423: F, t5242: F, t5245: F, t12339: F, t1901: F, t439: F, t5273: F, t2960: F, t3010: F, t3098: F, t822: F, t1447: F, t5277: F, t10216: F, t176: F, t1821: F, t493: F) -> (F, F, F, F, F, F, F) {
    let t13768 = t1423 * t5242;
    let t13769 = 4.0 / 45.0 * t13768;
    let t13770 = t1423 * t5245;
    let t13771 = 4.0 / 9.0 * t13770;
    let t13774 = 4.0 / 3.0 * t439 * t1901 * t12339;
    let t13775 = t1423 * t5273;
    let t13776 = 2.0 / 27.0 * t13775;
    let t13781 = 2.0 / 9.0 * t439 * t2960 * t822 * t3098 * t3010;
    let t13782 = t1447 * t5277;
    let t13783 = 2.0 / 45.0 * t13782;
    let t13787 = t493 * t10216 * t176 * t1821 / 9.0;
    (t13769, t13771, t13774, t13776, t13781, t13783, t13787)
}
