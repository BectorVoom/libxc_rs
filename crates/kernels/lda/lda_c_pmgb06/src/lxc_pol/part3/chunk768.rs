//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 768/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk768<F: Float>(t1770: F, t8085: F, t31: F, t4001: F, t122: F, t302: F, t1755: F, t1773: F, t1759: F, t1763: F, t4294: F, t707: F, t100: F, t1099: F, t1193: F, t4299: F, t83: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8087 = 0.0012955432484775182 * t8085 * t1770;
    let t8088 = t31 * t4001;
    let t8091 = 0.9106331049773876 * t122 * t8088 * t302;
    let t8092 = t1773 * t1755;
    let t8094 = t1773 * t1759;
    let t8097 = 0.31931290694012293 * t1773 * t1763;
    let t8099 = 0.07982822673503073 * t707 * t4294;
    let t8101 = 1.0 / t100 / t1099;
    let t8105 = 6.701521338562081e-05 * t8101 * t83 * t1193 * t4299;
    (t8087, t8088, t8091, t8092, t8094, t8097, t8099, t8101, t8105)
}
