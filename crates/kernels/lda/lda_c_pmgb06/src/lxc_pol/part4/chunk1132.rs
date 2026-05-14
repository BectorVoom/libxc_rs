//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1132/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1132<F: Float>(t1416: F, t493: F, t6130: F, t1417: F, t6134: F, t1972: F, t4757: F, t1559: F, t439: F, t6123: F, t1560: F, t6127: F, t2466: F, t3226: F, t1447: F, t6541: F) -> (F, F, F, F, F, F, F) {
    let t16952 = 2.0 / 45.0 * t493 * t6130 * t1416;
    let t16954 = 2.0 / 45.0 * t6134 * t1417;
    let t16956 = 4.0 / 45.0 * t1972 * t4757;
    let t16959 = 2.0 / 45.0 * t439 * t6123 * t1559;
    let t16961 = 2.0 / 45.0 * t6127 * t1560;
    let t16962 = t3226 * t2466;
    let t16963 = 4.0 / 135.0 * t16962;
    let t16964 = t1447 * t6541;
    (t16952, t16954, t16956, t16959, t16961, t16963, t16964)
}
