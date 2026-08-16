//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 731/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk731(t1822: f64, t350: f64, t1083: f64, t1820: f64, t1476: f64, t36: f64, t2911: f64, t764: f64, t1080: f64, t2909: f64, t1576: f64, t4857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4878 = t350 * t1822;
    let t4879 = 0.015996296296296297_f64 * t4878;
    let t4880 = t1820 * t1083;
    let t4881 = t1476 * t4880;
    let t4882 = t36 * t4881;
    let t4884 = t2911 * t764;
    let t4885 = t4884 * t1080;
    let t4886 = t2909 * t4885;
    let t4887 = t36 * t4886;
    let t4889 = t1576 * t4857;
    (t4878, t4879, t4880, t4881, t4882, t4884, t4885, t4886, t4887, t4889)
}
