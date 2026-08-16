//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 648/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk648(t3833: f64, t3889: f64, t258: f64, t248: f64, t1092: f64, t643: f64, t1090: f64, t638: f64, t3736: f64, t3744: f64, t3746: f64, t3748: f64, t3762: f64, t3764: f64, t3766: f64, t3768: f64, t3867: f64, t3871: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3890 = t3833 + t3889;
    let t3891 = t258 * t3890;
    let t3892 = t248 * t3891;
    let t3893 = t643 * t1092;
    let t3895 = t638 * t1090;
    let t3897 = -t3736 - t3744 - 1.7544670867903938_f64 * t3746 - 51.94757731704439_f64 * t3748 - t3762 - t3764 + t3766 + 3.0_f64 * t3768 + t3892 - t3867 + t3871 - 24.0_f64 * t3893 + 12.0_f64 * t3895;
    (t3890, t3891, t3892, t3893, t3895, t3897)
}
