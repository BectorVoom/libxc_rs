//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 630/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk630<F: Float>(t1896: F, t242: F, t5446: F, t1901: F, t632: F, t1125: F, t153: F, t865: F, t1210: F, t168: F, t861: F, t1891: F, t474: F, t1729: F, t452: F, t454: F) -> (F, F, F, F, F, F, F) {
    let t5894 = t1896 * t242;
    let t5897 = 0.1675256410710088 * t5446 * t242;
    let t5898 = t1901 * t632;
    let t5904 = t153 * t1125 * t865;
    let t5907 = t168 * t1210 * t861;
    let t5911 = 1.1389037339096726 * t153 * t474 * t1891;
    let t5924 = t1729 * t452 * t454;
    (t5894, t5897, t5898, t5904, t5907, t5911, t5924)
}
