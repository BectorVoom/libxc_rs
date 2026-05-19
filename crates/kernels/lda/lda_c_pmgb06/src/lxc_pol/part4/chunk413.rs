//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 413/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk413<F: Float>(t132: F, t1517: F, t134: F, t138: F, t1470: F, t350: F, t455: F, t139: F, t441: F) -> (F, F, F, F, F, F) {
    let t1518 = t132 * t1517;
    let t1519 = F::new(2.0) / F::new(45.0) * t1518;
    let t1521 = t138 * t1470 * t134;
    let t1522 = F::cast_from(0.002518888888888889_f64) * t1521;
    let t1523 = t350 * t455;
    let t1525 = t139 * t441;
    (t1518, t1519, t1521, t1522, t1523, t1525)
}
