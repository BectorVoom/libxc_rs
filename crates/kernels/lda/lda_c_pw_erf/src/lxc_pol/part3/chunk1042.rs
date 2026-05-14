//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1042/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1042<F: Float>(t13714: F, t10066: F, t10079: F, t10090: F, t10094: F, t10096: F, t13655: F, t13659: F, t13708: F, t13710: F, t13712: F, t13717: F, t13720: F, t13722: F, t13724: F, t13726: F, t13729: F, t13731: F, t13734: F, t13736: F) -> (F,) {
    let t14140 = 0.0016792592592592592 * t13714;
    let t14152 = -0.003778333333333333 * t10066 - 0.0006297222222222223 * t10079 - 0.005877407407407408 * t10090 - 0.02267 * t13655 + 0.006297222222222222 * t13659 + 0.026448333333333334 * t13708 - 0.005037777777777778 * t13710 - 0.011335 * t13712 + t14140 - 0.003778333333333333 * t13717 + 0.02267 * t13720 + 0.003778333333333333 * t13722 + 0.007556666666666666 * t13724 - 0.08312333333333333 * t13726 + 0.02267 * t13729 - 0.0019591358024691357 * t13731 - 0.007556666666666666 * t13734 - 0.061712777777777776 * t13736 + 0.003778333333333333 * t10094 - 0.0012594444444444445 * t10096;
    (t14152,)
}
