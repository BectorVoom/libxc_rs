//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1115/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1115(t1392: f64, t1887: f64, t3068: f64, t802: f64, t161: f64, t489: f64, t4940: f64, t3050: f64, t405: f64, t4892: f64, t4889: f64, t4902: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14017 = t1887 * t1392;
    let t14019 = t802 * t3068;
    let t14024 = t161 * t489 * t4940;
    let t14068 = t802 * t3050;
    let t14073 = t405 * t4892;
    let t14078 = t405 * t4889;
    let t14080 = t405 * t4902;
    (t14017, t14019, t14024, t14068, t14073, t14078, t14080)
}
