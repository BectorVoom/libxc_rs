//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1012/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1012<F: Float>(t1444: F, t4728: F, t4732: F, t4602: F, t5442: F, t1911: F, t493: F, t9925: F, t2979: F, t4731: F, t1981: F, t5441: F, t1380: F, t3382: F, t838: F, t1912: F, t3226: F) -> (F, F, F, F, F, F, F, F) {
    let t13865 = 2.0 / 15.0 * t1444 * t4728;
    let t13867 = t1444 * t4732 / 15.0;
    let t13869 = 4.0 / 15.0 * t4602 * t5442;
    let t13872 = t493 * t9925 * t1911 / 15.0;
    let t13875 = t493 * t2979 * t4731 / 15.0;
    let t13878 = 4.0 / 15.0 * t1981 * t2979 * t5441;
    let t13882 = t493 * t1380 * t838 * t3382 / 45.0;
    let t13883 = t3226 * t1912;
    (t13865, t13867, t13869, t13872, t13875, t13878, t13882, t13883)
}
