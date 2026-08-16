//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 671/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk671(t4073: f64, t551: f64, t3992: f64, t3996: f64, t4012: f64, t4028: f64, t4030: f64, t4032: f64, t4034: f64, t4038: f64, t4041: f64, t4046: f64, t4054: f64, t4056: f64, t4058: f64, t4061: f64, t4065: f64, t4069: f64, t4071: f64) -> (f64, f64) {
    let t4075 = 4.0_f64 / 5.0_f64 * t4073 * t551;
    let t4076 = t3992 + t3996 + t4012 + t4028 - t4030 + t4032 + t4034 + t4038 + t4041 + t4046 + t4054 + t4056 + t4058 + t4061 + t4065 + t4069 - t4071 + t4075;
    (t4075, t4076)
}
