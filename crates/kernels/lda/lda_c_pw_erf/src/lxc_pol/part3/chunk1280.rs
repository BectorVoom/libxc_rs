//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1280/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1280<F: Float>(t11093: F, t11097: F, t11098: F, t11101: F, t11104: F, t11105: F, t11107: F, t12719: F, t12720: F, t12721: F, t12722: F, t12724: F, t12726: F) -> F {
    let t15043 = t12719 + t12720 - t12721 - t12722 + t12724 + t12726 - F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t11093 + t11097 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11098 + t11101 - t11104 + F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t11105 - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t11107;
    t15043
}
