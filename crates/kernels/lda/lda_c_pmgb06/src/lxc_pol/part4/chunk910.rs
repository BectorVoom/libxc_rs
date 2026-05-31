//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 910/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk910<F: Float>(t6624: F, t2605: F, t435: F, t132: F, t337: F, t6560: F, t5069: F, t5068: F, t5139: F, t5138: F, t1593: F, t443: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6625 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6624;
    let t6626 = t435 * t2605;
    let t6627 = t132 * t6626;
    let t6628 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6627;
    let t6629 = t6560 * t337;
    let t6630 = t5069 * t6629;
    let t6632 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5068 * t6630;
    let t6633 = t5139 * t6629;
    let t6635 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5138 * t6633;
    let t6636 = t1593 * t443;
    (t6625, t6626, t6628, t6629, t6630, t6632, t6633, t6635, t6636)
}
