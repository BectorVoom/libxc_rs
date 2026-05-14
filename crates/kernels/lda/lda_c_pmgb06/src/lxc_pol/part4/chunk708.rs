//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 708/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk708<F: Float>(t2043: F, t432: F, t1395: F, t2064: F, t137: F, t132: F, t3058: F, t822: F, t1512: F, t824: F, t443: F, t472: F, t4637: F, t819: F, t955: F, t146: F, t3082: F, t3084: F, t3086: F, t3088: F, t3365: F, t3428: F, t4639: F, t4647: F, t4652: F, t4657: F, t4661: F, t4665: F, t4670: F, t4674: F, t4678: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4973 = t432 * t2043 / 15.0;
    let t4974 = t1395 * t2064;
    let t4975 = t137 * t4974;
    let t4977 = t132 * t4975 / 15.0;
    let t4978 = t3058 * t822;
    let t4979 = t137 * t4978;
    let t4981 = t132 * t4979 / 30.0;
    let t4983 = t1512 * t824 / 30.0;
    let t4989 = t472 * t443;
    let t5002 = 0.015996296296296297 * t4637;
    let t5003 = t955 * t819;
    let t5005 = -0.008888888888888889 * t3428 - 0.023994444444444443 * t3086 - 0.03199259259259259 * t3082 + 0.011997222222222222 * t3088 + 0.007998148148148148 * t3084 - 0.013333333333333334 * t146 * t3365 * t4989 - 0.07198333333333333 * t4678 - 0.21595 * t4665 + 0.14396666666666666 * t4652 - 0.023994444444444443 * t4661 - 0.03999074074074074 * t4647 - 0.09597777777777777 * t4657 + 0.07198333333333333 * t4674 + 0.2879333333333333 * t4670 - 0.047988888888888886 * t4639 + t5002 - 0.007407407407407408 * t5003;
    (t4973, t4974, t4975, t4977, t4978, t4979, t4981, t4983, t4989, t5002, t5003, t5005)
}
