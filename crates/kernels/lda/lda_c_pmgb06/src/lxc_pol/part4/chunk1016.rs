//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1016/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1016<F: Float>(t3055: F, t432: F, t132: F, t1396: F, t1547: F, t1540: F, t1592: F, t1595: F, t175: F, t3456: F, t152: F, t3030: F) -> (F, F, F, F, F, F) {
    let t9598 = t432 * t3055;
    let t9601 = t132 * t1547 * t1396;
    let t9610 = t1540 * t1592;
    let t9619 = t132 * t1547 * t1595;
    let t9636 = F::cast_from(1.0_f64) / t3456 / t175;
    let t9647 = F::cast_from(1.0_f64) / t3030 / t152;
    (t9598, t9601, t9610, t9619, t9636, t9647)
}
