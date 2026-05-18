//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 994/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk994<F: Float>(t1887: F, t1928: F, t4810: F, t802: F, t1554: F, t161: F, t2624: F, t132: F, t1547: F, t2630: F, t4844: F, t831: F) -> (F, F, F, F, F) {
    let t17919 = t1887 * t1928;
    let t17921 = t802 * t4810;
    let t17926 = t161 * t1554 * t2624;
    let t17931 = t132 * t1547 * t2630;
    let t17935 = t831 * t4844;
    (t17919, t17921, t17926, t17931, t17935)
}
