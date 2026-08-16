//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1304/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1304(t20892: f64, t20894: f64, t20897: f64, t20898: f64, t20899: f64, t20901: f64, t20903: f64, t20905: f64, t20910: f64, t20914: f64, t20916: f64, t20917: f64, t9250: f64) -> f64 {
    let t23201 = t20892 - t20894 + t20897 - t9250 + t20898 + t20899 - t20901 - t20903 + t20905 - t20910 + t20914 - t20916 + t20917;
    t23201
}
