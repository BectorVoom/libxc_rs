//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 924/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk924<F: Float>(t174: F, t2745: F, t3123: F, t1746: F, t4307: F, t902: F, t906: F, t13: F, t8185: F, t3128: F, t907: F, t3153: F, t357: F, t40: F, t2749: F, t936: F) -> (F, F, F, F, F, F) {
    let t8400 = 0.4274 * t174 * t2745 * t3123;
    let t8405 = t4307 * t1746;
    let t8407 = t902 * t902;
    let t8410 = t906 * t906;
    let t8414 = 24954.97798673547 * t13 / t8407 * t8185 / t8410;
    let t8417 = 578.9456755974397 * t3128 * t8185 * t907;
    let t8419 = t40 * t357 * t3153;
    let t8423 = 0.14246666666666666 * t174 * t2749 * t936;
    (t8400, t8405, t8414, t8417, t8419, t8423)
}
