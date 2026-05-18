//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1234/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1234<F: Float>(t1995: F, t5194: F, t1629: F, t1966: F, t439: F, t6554: F, t493: F, t5175: F, t6119: F, t12239: F, t12241: F, t432: F, t6736: F) -> (F, F, F, F, F, F) {
    let t16254 = t5194 * t1995;
    let t16255 = F::new(8.0) / F::new(45.0) * t16254;
    let t16259 = t439 * t1966 * t6554 * t1629 / F::new(15.0);
    let t16262 = F::new(4.0) / F::new(15.0) * t493 * t6119 * t5175;
    let t16263 = F::new(4.0) / F::new(135.0) * t12239;
    let t16264 = F::new(4.0) / F::new(45.0) * t12241;
    let t16266 = t432 * t6736 / F::new(15.0);
    (t16255, t16259, t16262, t16263, t16264, t16266)
}
