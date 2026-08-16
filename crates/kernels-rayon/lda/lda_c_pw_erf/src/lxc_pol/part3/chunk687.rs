//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 687/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk687(t3890: f64, t3898: f64, t3902: f64, t3907: f64, t3955: f64, t3957: f64, t3972: f64, t3981: f64, t3984: f64, t3996: f64, t4012: f64, t4028: f64, t4030: f64, t4032: f64, t4034: f64, t4038: f64, t4041: f64) -> f64 {
    let t4213 = t3890 + t3898 - t3902 - t3907 + t3955 + t3957 - t3972 - t3981 + t3984 + t3996 + t4012 + t4028 - t4030 + t4032 + t4034 + t4038 + t4041;
    t4213
}
