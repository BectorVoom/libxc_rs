//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 738/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk738<F: Float>(t1966: F, t4766: F, t439: F, t1417: F, t1972: F, t1559: F, t1962: F, t1560: F, t2002: F, t3213: F, t806: F, t1872: F, t441: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4767 = t1966 * t4766;
    let t4769 = t439 * t4767 / F::cast_from(5.0_f64);
    let t4771 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1972 * t1417;
    let t4772 = t1962 * t1559;
    let t4774 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t439 * t4772;
    let t4776 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t2002 * t1560;
    let t4777 = t3213 * t806;
    let t4778 = F::cast_from(2.0_f64) / F::cast_from(405.0_f64) * t4777;
    let t4779 = t441 * t1872;
    (t4767, t4769, t4771, t4772, t4774, t4776, t4777, t4778, t4779)
}
