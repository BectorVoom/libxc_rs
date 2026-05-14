//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 874/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk874<F: Float>(t132: F, t435: F, t6583: F, t6571: F, t1894: F, t5220: F, t1898: F, t1902: F, t5211: F, t6478: F, t432: F, t6836: F, t6674: F, t1447: F, t6114: F, t1995: F, t5194: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16181 = t132 * t435 * t6583;
    let t16184 = t132 * t435 * t6571;
    let t16213 = t5220 * t1894;
    let t16215 = t5220 * t1898;
    let t16217 = t5220 * t1902;
    let t16219 = t5211 * t6478;
    let t16238 = t432 * t6836;
    let t16241 = t132 * t435 * t6674;
    let t16249 = t1447 * t6114;
    let t16254 = t5194 * t1995;
    (t16181, t16184, t16213, t16215, t16217, t16219, t16238, t16241, t16249, t16254)
}
