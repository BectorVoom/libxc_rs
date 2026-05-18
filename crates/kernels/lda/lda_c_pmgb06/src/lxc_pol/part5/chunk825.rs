//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 825/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk825<F: Float>(t3032: F, t7811: F, t137: F, t132: F, t6837: F, t6839: F, t6841: F, t6844: F, t6846: F, t5497: F, t6852: F, t3368: F, t3380: F, t4909: F, t6800: F, t6811: F, t6819: F, t6873: F, t6875: F, t6877: F, t7596: F, t7614: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7812 = t3032 * t7811;
    let t7813 = t137 * t7812;
    let t7815 = t132 * t7813 / F::new(5.0);
    let t7816 = t6837 / F::new(15.0);
    let t7817 = t6839 / F::new(15.0);
    let t7818 = F::new(2.0) / F::new(15.0) * t6841;
    let t7819 = t6844 / F::new(15.0);
    let t7820 = t6846 / F::new(15.0);
    let t7821 = F::new(2.0) / F::new(135.0) * t5497;
    let t7822 = F::new(2.0) / F::new(15.0) * t6852;
    let t7832 = -F::new(0.03999074074074074) * t7596 - F::new(0.035991666666666665) * t7614 + F::new(0.023994444444444443) * t6800 - F::new(0.07198333333333333) * t6811 + F::new(0.035991666666666665) * t6819 - F::new(0.02666666666666667) * t6873 + F::new(0.013333333333333334) * t6875 + F::new(0.0044444444444444444) * t6877 - t3368 - t3380 - F::new(0.022222222222222223) * t4909;
    (t7812, t7813, t7815, t7816, t7817, t7818, t7819, t7820, t7821, t7822, t7832)
}
