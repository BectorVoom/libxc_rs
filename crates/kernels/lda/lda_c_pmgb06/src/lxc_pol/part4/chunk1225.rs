//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1225/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1225<F: Float>(t16137: F, t12084: F, t12105: F, t12107: F, t16112: F, t16114: F, t16117: F, t16121: F, t16122: F, t16124: F, t16126: F, t16130: F, t16132: F, t16135: F, t16136: F) -> (F, F, F, F, F) {
    let t16138 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16137;
    let t16139 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t12084;
    let t16140 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12105;
    let t16141 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t12107;
    let t16142 = t16112 + t16114 + t16117 + t16121 + t16122 + t16124 + t16126 + t16130 - t16132 + t16135 - t16136 + t16138 + t16139 + t16140 + t16141;
    (t16138, t16139, t16140, t16141, t16142)
}
