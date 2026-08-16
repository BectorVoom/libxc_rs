//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 552/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk552(t20: f64, t2916: f64, t1125: f64, t161: f64, t1210: f64, t1132: f64, t1135: f64, t1139: f64, t1140: f64, t159: f64, t2908: f64, t2911: f64, t39: f64, t628: f64, t629: f64) -> (f64, f64, f64, f64) {
    let t2917 = t2916 * t20;
    let t2920 = t1125 * t161;
    let t2923 = t1210 * t161;
    let t2929 = t2908 / 2.0_f64 + 0.09405_f64 * t2911 * t629 - 0.1254_f64 * t1132 * t1135 + 0.02358774_f64 * t2917 * t1140 + 0.09753333333333333_f64 * t628 * t2920 - 0.03145032_f64 * t1139 * t2923 + 0.001883059277350998_f64 * t159 * t39 * t161;
    (t2917, t2920, t2923, t2929)
}
