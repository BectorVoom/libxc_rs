//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1164/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1164(t331: f64, t5010: f64, t4988: f64, t5001: f64, t5004: f64, t10053: f64, t10066: f64, t10079: f64, t10090: f64, t10216: f64, t12153: f64, t12264: f64, t13290: f64, t13368: f64, t13598: f64, t13655: f64, t13659: f64, t13661: f64, t13663: f64, t13665: f64, t13667: f64, t1371: f64, t2061: f64, t25: f64, t589: f64) -> f64 {
    let t13675 = t331 * t5010;
    let t13677 = t331 * t4988;
    let t13679 = t331 * t5001;
    let t13681 = t331 * t5004;
    let t13692 = -0.047988888888888886_f64 * t10053 + 0.07198333333333333_f64 * t10066 + 0.011997222222222222_f64 * t10079 + 0.11197407407407407_f64 * t10090 + 0.4319_f64 * t13655 - 0.11997222222222222_f64 * t13659 + 0.044444444444444446_f64 * t13661 - 0.007407407407407408_f64 * t13663 + 0.3466666666666667_f64 * t13665 - 0.057777777777777775_f64 * t13667 - 0.08_f64 * t25 * t1371 * t13290 + 0.16_f64 * t25 * t589 * t13368 - 0.02666666666666667_f64 * t13675 - 0.02666666666666667_f64 * t13677 + 0.0044444444444444444_f64 * t13679 + 0.005925925925925926_f64 * t13681 - 0.0022222222222222222_f64 * t25 * t1371 * t12264 + 0.013333333333333334_f64 * t2061 * t1371 * t12153 - 0.006913580246913581_f64 * t25 * t10216 * t13598;
    t13692
}
