//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 921/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk921<F: Float>(t337: F, t6759: F, t1915: F, t493: F, t1464: F, t2389: F, t1919: F, t2541: F, t529: F, t2991: F, t2648: F, t443: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6760 = t6759 * t337;
    let t6761 = t1915 * t6760;
    let t6763 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t493 * t6761;
    let t6764 = t1464 * t2389;
    let t6765 = t6764 * t337;
    let t6766 = t1919 * t6765;
    let t6768 = t493 * t6766 / F::cast_from(27.0_f64);
    let t6769 = t2541 * t529;
    let t6770 = t2991 * t6769;
    let t6772 = t493 * t6770 / F::cast_from(27.0_f64);
    let t6773 = t2648 * t443;
    (t6760, t6761, t6763, t6764, t6765, t6766, t6768, t6769, t6770, t6772, t6773)
}
