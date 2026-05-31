//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1049/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1049<F: Float>(t2924: F, t5138: F, t852: F, t2992: F, t5090: F, t1586: F, t764: F, t5068: F, t529: F, t6559: F, t337: F, t5069: F) -> (F, F, F, F, F) {
    let t12476 = t5138 * t852 * t2924 / F::cast_from(9.0_f64);
    let t12479 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5138 * t5090 * t2992;
    let t12480 = t764 * t1586;
    let t12484 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5068 * t6559 * t12480 * t529;
    let t12485 = t12480 * t337;
    let t12488 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5068 * t5069 * t12485;
    (t12476, t12479, t12484, t12485, t12488)
}
