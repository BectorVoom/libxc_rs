//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 964/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk964<F: Float>(t9885: F, t9887: F, t1179: F, t132: F, t441: F, t4829: F, t9890: F, t9892: F, t13056: F, t13060: F, t13063: F, t13067: F, t13071: F, t13074: F, t9895: F, t9898: F) -> (F, F, F, F, F, F, F, F) {
    let t13075 = t9885 / 15.0;
    let t13076 = t9887 / 15.0;
    let t13079 = t132 * t1179 * t441 * t4829;
    let t13080 = 8.0 / 45.0 * t13079;
    let t13081 = 4.0 / 135.0 * t9890;
    let t13082 = 2.0 / 45.0 * t9892;
    let t13083 = -t13056 - t13060 - t13063 + t13067 + t13071 + t13074 - t13075 - t13076 - t13080 - t13081 + t13082;
    let t13084 = 2.0 / 45.0 * t9895;
    let t13085 = 2.0 / 45.0 * t9898;
    (t13075, t13076, t13080, t13081, t13082, t13083, t13084, t13085)
}
