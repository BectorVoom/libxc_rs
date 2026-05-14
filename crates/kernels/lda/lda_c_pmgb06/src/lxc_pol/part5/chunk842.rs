//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 842/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk842<F: Float>(t12462: F, t11862: F, t160: F, t1983: F, t27: F, t34: F, t1435: F, t5075: F, t1438: F, t1593: F, t3247: F, t5065: F, t5066: F, t2911: F, t518: F, t409: F) -> (F, F, F, F, F, F, F, F) {
    let t12463 = 2.0 / 135.0 * t12462;
    let t12465 = t160 * t11862 * t1983;
    let t12514 = t27 * t34;
    let t12516 = t5075 * t12514 * t1435;
    let t12519 = t1593 * t1438;
    let t12529 = t5065 * t5066 * t3247;
    let t12530 = t518 * t2911;
    let t12535 = t27 * t409;
    (t12463, t12465, t12514, t12516, t12519, t12529, t12530, t12535)
}
