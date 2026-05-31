//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1324/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1324<F: Float>(t2389: F, t337: F, t529: F, t5068: F, t5069: F, t5138: F, t5139: F, t1414: F, t2093: F, t5071: F, t1420: F, t6551: F) -> (F, F, F, F) {
    let t17404 = t2389 * t529 * t337;
    let t17407 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5068 * t5069 * t17404;
    let t17410 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5138 * t5139 * t17404;
    let t17414 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5068 * t2093 * t1414 * t5071;
    let t17416 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1420 * t6551;
    (t17407, t17410, t17414, t17416)
}
