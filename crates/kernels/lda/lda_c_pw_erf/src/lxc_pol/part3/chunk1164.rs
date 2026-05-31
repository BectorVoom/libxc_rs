//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1164/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1164<F: Float>(t331: F, t5010: F, t4988: F, t5001: F, t5004: F, t10053: F, t10066: F, t10079: F, t10090: F, t10216: F, t12153: F, t12264: F, t13290: F, t13368: F, t13598: F, t13655: F, t13659: F, t13661: F, t13663: F, t13665: F, t13667: F, t1371: F, t2061: F, t25: F, t589: F) -> F {
    let t13675 = t331 * t5010;
    let t13677 = t331 * t4988;
    let t13679 = t331 * t5001;
    let t13681 = t331 * t5004;
    let t13692 = -F::cast_from(0.047988888888888886_f64) * t10053 + F::cast_from(0.07198333333333333_f64) * t10066 + F::cast_from(0.011997222222222222_f64) * t10079 + F::cast_from(0.11197407407407407_f64) * t10090 + F::cast_from(0.4319_f64) * t13655 - F::cast_from(0.11997222222222222_f64) * t13659 + F::cast_from(0.044444444444444446_f64) * t13661 - F::cast_from(0.007407407407407408_f64) * t13663 + F::cast_from(0.3466666666666667_f64) * t13665 - F::cast_from(0.057777777777777775_f64) * t13667 - F::cast_from(0.08_f64) * t25 * t1371 * t13290 + F::cast_from(0.16_f64) * t25 * t589 * t13368 - F::cast_from(0.02666666666666667_f64) * t13675 - F::cast_from(0.02666666666666667_f64) * t13677 + F::cast_from(0.0044444444444444444_f64) * t13679 + F::cast_from(0.005925925925925926_f64) * t13681 - F::cast_from(0.0022222222222222222_f64) * t25 * t1371 * t12264 + F::cast_from(0.013333333333333334_f64) * t2061 * t1371 * t12153 - F::cast_from(0.006913580246913581_f64) * t25 * t10216 * t13598;
    t13692
}
