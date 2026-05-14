//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 998/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk998<F: Float>(t16050: F, t16053: F, t16058: F, t16065: F, t568: F, t7676: F, t2023: F, t6205: F, t11020: F, t11022: F, t11025: F, t11027: F, t11029: F, t19123: F, t21001: F, t16069: F) -> (F, F, F, F, F, F, F, F) {
    let t21002 = 32.0 / 45.0 * t16050;
    let t21003 = 32.0 / 45.0 * t16053;
    let t21004 = 16.0 / 135.0 * t16058;
    let t21005 = 8.0 / 45.0 * t16065;
    let t21007 = t7676 * t568;
    let t21008 = 4.0 / 45.0 * t21007;
    let t21012 = 4.0 / 15.0 * t6205 * t2023;
    let t21013 = t21001 - t21002 - t21003 + t21004 - t21005 + 2.0 / 45.0 * t19123 + t21008 + 0.09973633333333333 * t11020 - 0.06649088888888889 * t11022 - t11025 + t11027 + t11029 - t21012;
    let t21014 = 8.0 / 27.0 * t16069;
    (t21002, t21003, t21004, t21005, t21008, t21012, t21013, t21014)
}
