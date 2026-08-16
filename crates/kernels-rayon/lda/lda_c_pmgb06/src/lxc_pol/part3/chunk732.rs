//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 732/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk732(t4861: f64, t525: f64, t2082: f64, t405: f64, t2079: f64, t1576: f64, t4880: f64, t3358: f64, t4885: f64, t103: f64, t2060: f64, t4848: f64, t4853: f64, t4859: f64, t4863: f64, t4868: f64, t4871: f64, t4874: f64, t4876: f64, t4879: f64, t4882: f64, t4887: f64, t4889: f64) -> (f64, f64, f64, f64) {
    let t4892 = t525 * t4861;
    let t4896 = 0.017777777777777778_f64 * t405 * t2082;
    let t4898 = 0.002962962962962963_f64 * t405 * t2079;
    let t4899 = t1576 * t4880;
    let t4902 = t3358 * t4885;
    let t4905 = 0.013333333333333334_f64 * t103 * t4848 - 0.05333333333333334_f64 * t2060 * t4853 + 0.14396666666666666_f64 * t4859 - 0.21595_f64 * t4863 + 0.09597777777777777_f64 * t4868 + 0.07198333333333333_f64 * t4871 - 0.2879333333333333_f64 * t4874 - 0.047988888888888886_f64 * t4876 + t4879 - 0.023994444444444443_f64 * t4882 - 0.03999074074074074_f64 * t4887 + 0.013333333333333334_f64 * t103 * t4889 - 0.04_f64 * t103 * t4892 - t4896 + t4898 - 0.0022222222222222222_f64 * t103 * t4899 - 0.002962962962962963_f64 * t103 * t4902;
    (t4892, t4899, t4902, t4905)
}
