//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1265/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1265<F: Float>(t1499: F, t2601: F, t486: F, t6449: F, t1586: F, t1992: F, t493: F, t6112: F, t2002: F, t4615: F, t4620: F, t1420: F, t6556: F) -> (F, F, F, F, F, F) {
    let t16623 = t1499 * t2601 / F::new(15.0);
    let t16625 = F::new(2.0) / F::new(15.0) * t486 * t6449;
    let t16629 = t493 * t1992 * t6112 * t1586 / F::new(15.0);
    let t16631 = F::new(2.0) / F::new(45.0) * t2002 * t4615;
    let t16633 = F::new(2.0) / F::new(27.0) * t2002 * t4620;
    let t16635 = F::new(2.0) / F::new(15.0) * t1420 * t6556;
    (t16623, t16625, t16629, t16631, t16633, t16635)
}
