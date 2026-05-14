//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1004/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1004<F: Float>(t1972: F, t6292: F, t1444: F, t7656: F, t2488: F, t493: F, t5312: F, t1420: F, t7646: F, t17577: F, t432: F, t7803: F, t132: F, t435: F, t7718: F, t6851: F, t831: F) -> (F, F, F, F, F, F, F, F) {
    let t20784 = 2.0 / 15.0 * t1972 * t6292;
    let t20786 = 2.0 / 15.0 * t1444 * t7656;
    let t20789 = 2.0 / 15.0 * t493 * t5312 * t2488;
    let t20791 = 2.0 / 9.0 * t1420 * t7646;
    let t20792 = 4.0 / 45.0 * t17577;
    let t20794 = t432 * t7803 / 30.0;
    let t20796 = t132 * t435 * t7718;
    let t20797 = 2.0 / 15.0 * t20796;
    let t20798 = t831 * t6851;
    (t20784, t20786, t20789, t20791, t20792, t20794, t20797, t20798)
}
