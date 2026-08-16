//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 983/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk983<F: Float>(t1090: F, t1105: F, t1092: F, t1101: F, t3767: F, t643: F, t248: F, t3890: F, t653: F, t1024: F, t3697: F, t634: F) -> (F, F, F, F, F) {
    let t8541 = t1105 * t1090;
    let t8543 = t1101 * t1092;
    let t8545 = t643 * t3767;
    let t8548 = t248 * t653 * t3890;
    let t8552 = F::cast_from(8.0_f64) * t1024 * t634 * t3697;
    (t8541, t8543, t8545, t8548, t8552)
}
