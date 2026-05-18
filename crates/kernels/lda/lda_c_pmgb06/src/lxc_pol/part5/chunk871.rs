//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 871/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk871<F: Float>(t1041: F, t1043: F, t3697: F, t632: F, t2799: F, t654: F, t2801: F, t3891: F, t638: F, t643: F, t1003: F, t993: F) -> (F, F, F, F, F, F, F) {
    let t8841 = F::new(64.32791799477015) * t1041 * t3697 * t1043 * t632;
    let t8844 = t2799 * t654;
    let t8846 = t2801 * t654;
    let t8850 = t638 * t3891;
    let t8853 = F::new(16.0) * t643 * t3891;
    let t8863 = t1003 * t1003;
    let t8867 = t993 * t993;
    (t8841, t8844, t8846, t8850, t8853, t8863, t8867)
}
