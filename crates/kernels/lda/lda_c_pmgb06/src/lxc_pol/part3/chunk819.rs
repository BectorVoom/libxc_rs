//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 819/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk819<F: Float>(t3341: F, t405: F, t132: F, t3046: F, t435: F, t1547: F, t1630: F, t1980: F, t604: F, t223: F, t5210: F, t4143: F, t607: F, t1710: F, t1727: F, t1512: F, t1548: F) -> (F, F, F, F, F, F, F, F) {
    let t10006 = t405 * t3341;
    let t10040 = t132 * t435 * t3046;
    let t10046 = t132 * t1547 * t1630;
    let t10079 = t604 * t1980;
    let t10082 = 56.0 / 1215.0 * t223 * t5210;
    let t10083 = t4143 * t607;
    let t10085 = t1727 * t1710;
    let t10087 = t1512 * t1548;
    (t10006, t10040, t10046, t10079, t10082, t10083, t10085, t10087)
}
