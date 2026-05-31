//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 671/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk671<F: Float>(t4073: F, t551: F, t3992: F, t3996: F, t4012: F, t4028: F, t4030: F, t4032: F, t4034: F, t4038: F, t4041: F, t4046: F, t4054: F, t4056: F, t4058: F, t4061: F, t4065: F, t4069: F, t4071: F) -> (F, F) {
    let t4075 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t4073 * t551;
    let t4076 = t3992 + t3996 + t4012 + t4028 - t4030 + t4032 + t4034 + t4038 + t4041 + t4046 + t4054 + t4056 + t4058 + t4061 + t4065 + t4069 - t4071 + t4075;
    (t4075, t4076)
}
