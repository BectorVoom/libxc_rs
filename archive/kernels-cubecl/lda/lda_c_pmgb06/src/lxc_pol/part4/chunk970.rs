//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 970/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk970<F: Float>(t100: F, t1099: F, t1193: F, t4299: F, t83: F, t1166: F, t1767: F, t1770: F, t419: F, t1530: F, t9: F, t14: F, t1413: F) -> (F, F, F, F, F) {
    let t8101 = F::cast_from(1.0_f64) / t100 / t1099;
    let t8105 = F::cast_from(6.701521338562081e-05_f64) * t8101 * t83 * t1193 * t4299;
    let t8108 = t1767 * t1166 * t419 * t1770;
    let t8119 = F::cast_from(1.0_f64) / t9 / t1530;
    let t8139 = F::cast_from(1.0_f64) / t14 / t1413;
    (t8101, t8105, t8108, t8119, t8139)
}
