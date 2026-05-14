//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 971/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk971<F: Float>(t1981: F, t835: F, t1454: F, t493: F, t5312: F, t1461: F, t1835: F, t1466: F, t1989: F, t3198: F, t1444: F, t4585: F, t3384: F, t831: F, t1636: F, t1848: F) -> (F, F, F, F, F, F, F) {
    let t13177 = t1981 * t835;
    let t13178 = 8.0 / 1215.0 * t13177;
    let t13181 = t493 * t5312 * t1454 / 15.0;
    let t13182 = t1461 * t1835;
    let t13185 = t493 * t13182 * t1466 / 9.0;
    let t13187 = t3198 * t1989 / 15.0;
    let t13189 = t1444 * t4585 / 15.0;
    let t13191 = t831 * t3384 / 30.0;
    let t13192 = t1848 * t1636;
    (t13178, t13181, t13185, t13187, t13189, t13191, t13192)
}
