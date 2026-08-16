//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1053/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1053<F: Float>(t3247: F, t5065: F, t5066: F, t2911: F, t518: F, t1080: F, t5070: F, t27: F, t409: F, t1461: F, t1: F, t337: F, t529: F) -> (F, F, F, F, F, F) {
    let t12529 = t5065 * t5066 * t3247;
    let t12530 = t518 * t2911;
    let t12531 = t5070 * t1080;
    let t12534 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12529 * t12530 * t12531;
    let t12535 = t27 * t409;
    let t12537 = t5065 * t12535 * t1461;
    let t12539 = t1 * t529 * t337;
    (t12529, t12531, t12534, t12535, t12537, t12539)
}
