//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 555/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk555<F: Float>(t20: F, t2916: F, t1125: F, t161: F, t1210: F, t1132: F, t1135: F, t1139: F, t1140: F, t159: F, t2908: F, t2911: F, t39: F, t628: F, t629: F) -> (F, F, F, F) {
    let t2917 = t2916 * t20;
    let t2920 = t1125 * t161;
    let t2923 = t1210 * t161;
    let t2929 = t2908 / F::cast_from(2.0_f64) + F::cast_from(0.09405_f64) * t2911 * t629 - F::cast_from(0.1254_f64) * t1132 * t1135 + F::cast_from(0.02358774_f64) * t2917 * t1140 + F::cast_from(0.09753333333333333_f64) * t628 * t2920 - F::cast_from(0.03145032_f64) * t1139 * t2923 + F::cast_from(0.001883059277350998_f64) * t159 * t39 * t161;
    (t2917, t2920, t2923, t2929)
}
