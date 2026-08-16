//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1200/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1200(t13714: f64, t10066: f64, t10079: f64, t10090: f64, t10094: f64, t10096: f64, t13655: f64, t13659: f64, t13708: f64, t13710: f64, t13712: f64, t13717: f64, t13720: f64, t13722: f64, t13724: f64, t13726: f64, t13729: f64, t13731: f64, t13734: f64, t13736: f64) -> f64 {
    let t14140 = 0.0016792592592592592_f64 * t13714;
    let t14152 = -0.003778333333333333_f64 * t10066 - 0.0006297222222222223_f64 * t10079 - 0.005877407407407408_f64 * t10090 - 0.02267_f64 * t13655 + 0.006297222222222222_f64 * t13659 + 0.026448333333333334_f64 * t13708 - 0.005037777777777778_f64 * t13710 - 0.011335_f64 * t13712 + t14140 - 0.003778333333333333_f64 * t13717 + 0.02267_f64 * t13720 + 0.003778333333333333_f64 * t13722 + 0.007556666666666666_f64 * t13724 - 0.08312333333333333_f64 * t13726 + 0.02267_f64 * t13729 - 0.0019591358024691357_f64 * t13731 - 0.007556666666666666_f64 * t13734 - 0.061712777777777776_f64 * t13736 + 0.003778333333333333_f64 * t10094 - 0.0012594444444444445_f64 * t10096;
    t14152
}
