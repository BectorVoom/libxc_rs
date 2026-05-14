//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1019/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1019<F: Float>(t13971: F, t1499: F, t2018: F, t132: F, t443: F, t459: F, t4828: F, t464: F, t4680: F, t137: F, t477: F, t1423: F, t5350: F, t12389: F, t1897: F, t439: F) -> (F, F, F, F, F, F) {
    let t13972 = 2.0 / 15.0 * t13971;
    let t13973 = t1499 * t2018;
    let t13974 = t13973 / 15.0;
    let t13978 = 2.0 / 15.0 * t132 * t4828 * t459 * t443;
    let t13979 = t4680 * t464;
    let t13983 = t132 * t137 * t13979 * t477 / 10.0;
    let t13984 = t1423 * t5350;
    let t13985 = 4.0 / 15.0 * t13984;
    let t13988 = 8.0 / 15.0 * t439 * t1897 * t12389;
    (t13972, t13974, t13978, t13983, t13985, t13988)
}
