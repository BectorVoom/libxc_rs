//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 700/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk700<F: Float>(t1547: F, t823: F, t132: F, t409: F, t495: F, t177: F, t497: F, t161: F, t1554: F, t852: F, t1083: F, t1825: F, t525: F, t1: F, t1414: F, t337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4836 = t1547 * t823;
    let t4837 = t132 * t4836;
    let t4838 = t4837 / 135.0;
    let t4839 = t409 * t495;
    let t4840 = t177 * t497;
    let t4841 = t4839 * t4840;
    let t4843 = 2.0 / 45.0 * t161 * t4841;
    let t4844 = t1554 * t852;
    let t4845 = t161 * t4844;
    let t4846 = t4845 / 135.0;
    let t4847 = t1825 * t1083;
    let t4848 = t525 * t4847;
    let t4851 = t1414 * t1;
    let t4852 = t4851 * t337;
    (t4836, t4837, t4838, t4839, t4840, t4841, t4843, t4844, t4845, t4846, t4847, t4848, t4852)
}
