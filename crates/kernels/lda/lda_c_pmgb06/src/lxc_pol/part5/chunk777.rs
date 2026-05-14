//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 777/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk777<F: Float>(t1759: F, t1773: F, t1763: F, t4294: F, t707: F, t100: F, t1099: F, t1193: F, t4299: F, t83: F, t1530: F, t9: F, t1: F, t642: F, t14: F, t1413: F) -> (F, F, F, F, F, F, F, F) {
    let t8094 = t1773 * t1759;
    let t8097 = 0.31931290694012293 * t1773 * t1763;
    let t8099 = 0.07982822673503073 * t707 * t4294;
    let t8101 = 1.0 / t100 / t1099;
    let t8105 = 6.701521338562081e-05 * t8101 * t83 * t1193 * t4299;
    let t8119 = 1.0 / t9 / t1530;
    let t8131 = t1 * t642;
    let t8139 = 1.0 / t14 / t1413;
    (t8094, t8097, t8099, t8101, t8105, t8119, t8131, t8139)
}
