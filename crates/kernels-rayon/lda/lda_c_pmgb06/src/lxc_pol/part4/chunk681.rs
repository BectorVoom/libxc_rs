//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 681/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk681(t3833: f64, t3889: f64, t258: f64, t248: f64, t1092: f64, t643: f64, t1090: f64, t638: f64, t1089: f64, t686: f64, t1108: f64, t654: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3890 = t3833 + t3889;
    let t3891 = t258 * t3890;
    let t3892 = t248 * t3891;
    let t3893 = t643 * t1092;
    let t3895 = t638 * t1090;
    let t3899 = t643 * t1090;
    let t3901 = t638 * t1092;
    let t3903 = t1089 * t686;
    let t3904 = t248 * t3903;
    let t3906 = t1108 * t654;
    (t3890, t3891, t3892, t3893, t3895, t3899, t3901, t3903, t3904, t3906)
}
