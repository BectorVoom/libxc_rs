//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1063/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1063<F: Float>(t1972: F, t2974: F, t1382: F, t5194: F, t1592: F, t1962: F, t2865: F, t439: F, t1602: F, t1992: F, t2088: F, t3457: F, t493: F) -> (F, F, F, F) {
    let t12630 = F::new(2.0) / F::new(15.0) * t1972 * t2974;
    let t12631 = t5194 * t1382;
    let t12632 = F::new(4.0) / F::new(45.0) * t12631;
    let t12633 = t1962 * t1592;
    let t12636 = F::new(2.0) / F::new(15.0) * t439 * t12633 * t2865;
    let t12641 = F::new(3.0) / F::new(5.0) * t493 * t1992 * t3457 * t2088 * t1602;
    (t12630, t12632, t12636, t12641)
}
