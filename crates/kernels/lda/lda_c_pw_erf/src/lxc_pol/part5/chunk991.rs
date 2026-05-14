//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 991/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk991<F: Float>(t15764: F, t14979: F, t14980: F, t20897: F, t20898: F, t20899: F, t20901: F, t20903: F, t20905: F, t20910: F, t20914: F, t20916: F, t9250: F, t4763: F, t6244: F, t2011: F, t6205: F) -> (F, F, F, F) {
    let t20917 = 8.0 / 45.0 * t15764;
    let t20919 = t20897 - t9250 + t20898 + t20899 - t20901 - t20903 + t20905 - t20910 + t20914 - t20916 + t20917 + t14979 + 0.299209 * t14980;
    let t20921 = 16.0 / 5.0 * t4763 * t6244;
    let t20923 = 4.0 / 15.0 * t6205 * t2011;
    (t20917, t20919, t20921, t20923)
}
