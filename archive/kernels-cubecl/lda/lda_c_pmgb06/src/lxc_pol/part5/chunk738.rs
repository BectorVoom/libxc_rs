//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 738/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk738<F: Float>(t6868: F, t6903: F, t518: F, t166: F, t161: F, t1925: F, t1972: F, t2555: F, t3451: F, t439: F, t486: F, t493: F, t5497: F, t5500: F, t6783: F, t6788: F, t6791: F, t6833: F, t6837: F, t6839: F, t6841: F, t6844: F, t6846: F, t6852: F) -> (F, F, F, F) {
    let t6904 = t6868 + t6903;
    let t6905 = t518 * t6904;
    let t6906 = t166 * t6905;
    let t6909 = -t493 * t6783 / F::cast_from(45.0_f64) - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1972 * t1925 - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t439 * t6788 - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t493 * t6791 + t486 * t2555 / F::cast_from(30.0_f64) + t161 * t6833 / F::cast_from(30.0_f64) + t6837 / F::cast_from(45.0_f64) + t6839 / F::cast_from(45.0_f64) + F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6841 + t6844 / F::cast_from(45.0_f64) + t6846 / F::cast_from(45.0_f64) - F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t5497 - F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t5500 + t3451 / F::cast_from(135.0_f64) + F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6852 - t161 * t6906 / F::cast_from(30.0_f64);
    (t6904, t6905, t6906, t6909)
}
