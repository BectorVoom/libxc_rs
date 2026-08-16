//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1001/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1001(t130: f64, t485: f64, t5067: f64, t5091: f64, t1381: f64, t3038: f64, t5068: f64, t851: f64, t432: f64, t5041: f64, t11889: f64, t11890: f64, t11891: f64, t11892: f64, t11893: f64, t11894: f64, t11895: f64, t11898: f64, t11902: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11903 = t485 * t130;
    let t11904 = t11903 * t5067;
    let t11906 = 4.0_f64 / 15.0_f64 * t11904 * t5091;
    let t11910 = 4.0_f64 / 15.0_f64 * t5068 * t3038 * t851 * t1381;
    let t11912 = t432 * t5041 / 10.0_f64;
    let t11913 = t11889 - t11890 - t11891 - t11892 + t11893 + t11894 + t11895 + t11898 + t11902 + t11906 + t11910 - t11912;
    (t11903, t11904, t11906, t11910, t11912, t11913)
}
