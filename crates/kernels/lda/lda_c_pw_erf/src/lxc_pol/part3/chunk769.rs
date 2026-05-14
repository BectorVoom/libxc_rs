//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 769/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk769<F: Float>(t1901: F, t646: F, t3985: F, t3988: F, t3992: F, t4955: F, t4961: F, t4963: F, t4966: F, t4968: F, t4970: F, t4972: F, t5033: F, t5035: F, t5037: F, t5039: F, t5043: F, t5047: F) -> (F,) {
    let t5859 = t1901 * t646;
    let t5861 = t4955 - t4961 - t4963 - t4966 - t4968 - t4970 - 4.0 / 27.0 * t3985 - t3988 + t3992 - t4972 - t5033 - t5035 - t5037 + 0.033245444444444446 * t5859 + t5039 + t5043 + t5047;
    (t5861,)
}
