//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1280/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1280(t11093: f64, t11097: f64, t11098: f64, t11101: f64, t11104: f64, t11105: f64, t11107: f64, t12719: f64, t12720: f64, t12721: f64, t12722: f64, t12724: f64, t12726: f64) -> f64 {
    let t15043 = t12719 + t12720 - t12721 - t12722 + t12724 + t12726 - 8.0_f64 / 135.0_f64 * t11093 + t11097 + 8.0_f64 / 27.0_f64 * t11098 + t11101 - t11104 + 2.0_f64 / 45.0_f64 * t11105 - 2.0_f64 / 45.0_f64 * t11107;
    t15043
}
