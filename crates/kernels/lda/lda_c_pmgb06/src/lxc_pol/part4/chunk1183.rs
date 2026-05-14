//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1183/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1183<F: Float>(t14019: F, t1916: F, t5194: F, t1972: F, t5333: F, t1920: F, t1594: F, t2570: F, t439: F, t9084: F, t15349: F, t1897: F, t2578: F, t2864: F, t1420: F, t6788: F) -> (F, F, F, F, F, F, F, F) {
    let t17885 = 4.0 / 45.0 * t14019;
    let t17886 = t5194 * t1916;
    let t17887 = 16.0 / 135.0 * t17886;
    let t17889 = 4.0 / 15.0 * t1972 * t5333;
    let t17890 = t5194 * t1920;
    let t17891 = 8.0 / 81.0 * t17890;
    let t17895 = 2.0 / 27.0 * t439 * t9084 * t2570 * t1594;
    let t17898 = 2.0 / 15.0 * t439 * t1897 * t15349;
    let t17902 = 2.0 / 45.0 * t439 * t2864 * t2578 * t1594;
    let t17904 = 4.0 / 45.0 * t1420 * t6788;
    (t17885, t17887, t17889, t17891, t17895, t17898, t17902, t17904)
}
