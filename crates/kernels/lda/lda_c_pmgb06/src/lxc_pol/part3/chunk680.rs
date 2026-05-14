//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 680/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk680<F: Float>(t4852: F, t525: F, t2918: F, t764: F, t1080: F, t1476: F, t36: F, t1820: F, t506: F, t1: F, t1464: F, t337: F, t1830: F, t4847: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4853 = t525 * t4852;
    let t4856 = t2918 * t764;
    let t4857 = t4856 * t1080;
    let t4858 = t1476 * t4857;
    let t4859 = t36 * t4858;
    let t4861 = t1820 * t1080;
    let t4862 = t506 * t4861;
    let t4863 = t36 * t4862;
    let t4865 = t1464 * t1;
    let t4866 = t4865 * t337;
    let t4867 = t1476 * t4866;
    let t4868 = t1830 * t4867;
    let t4870 = t506 * t4847;
    let t4871 = t36 * t4870;
    let t4873 = t506 * t4852;
    let t4874 = t1830 * t4873;
    (t4853, t4856, t4857, t4858, t4859, t4861, t4862, t4863, t4865, t4866, t4867, t4868, t4870, t4871, t4873, t4874)
}
