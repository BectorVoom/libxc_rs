//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1304/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1304<F: Float>(t20892: F, t20894: F, t20897: F, t20898: F, t20899: F, t20901: F, t20903: F, t20905: F, t20910: F, t20914: F, t20916: F, t20917: F, t9250: F) -> F {
    let t23201 = t20892 - t20894 + t20897 - t9250 + t20898 + t20899 - t20901 - t20903 + t20905 - t20910 + t20914 - t20916 + t20917;
    t23201
}
