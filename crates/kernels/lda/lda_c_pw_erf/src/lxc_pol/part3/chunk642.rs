//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 642/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk642<F: Float>(t3890: F, t3898: F, t3902: F, t3907: F, t3955: F, t3957: F, t3972: F, t3981: F, t3984: F, t3996: F, t4012: F, t4028: F, t4030: F, t4032: F, t4034: F, t4038: F, t4041: F) -> (F,) {
    let t4213 = t3890 + t3898 - t3902 - t3907 + t3955 + t3957 - t3972 - t3981 + t3984 + t3996 + t4012 + t4028 - t4030 + t4032 + t4034 + t4038 + t4041;
    (t4213,)
}
