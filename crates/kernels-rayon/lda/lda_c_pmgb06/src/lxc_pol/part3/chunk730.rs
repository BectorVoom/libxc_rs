//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 730/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk730(t36: f64, t4862: f64, t1: f64, t1464: f64, t337: f64, t1476: f64, t1830: f64, t4847: f64, t506: f64, t4852: f64, t1827: f64, t350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4863 = t36 * t4862;
    let t4865 = t1464 * t1;
    let t4866 = t4865 * t337;
    let t4867 = t1476 * t4866;
    let t4868 = t1830 * t4867;
    let t4870 = t506 * t4847;
    let t4871 = t36 * t4870;
    let t4873 = t506 * t4852;
    let t4874 = t1830 * t4873;
    let t4876 = t350 * t1827;
    (t4863, t4865, t4866, t4867, t4868, t4870, t4871, t4873, t4874, t4876)
}
