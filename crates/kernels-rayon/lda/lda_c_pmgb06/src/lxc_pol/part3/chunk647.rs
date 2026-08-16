//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 647/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk647(t1018: f64, t1036: f64, t138: f64, t1040: f64, t109: f64, t1044: f64, t1003: f64, t1009: f64, t1054: f64, t1055: f64, t1061: f64, t1180: f64, t269: f64, t282: f64, t30: f64, t3719: f64, t3834: f64, t3842: f64, t3851: f64, t3859: f64, t3862: f64, t3867: f64, t3871: f64, t3874: f64, t3877: f64, t3881: f64, t666: f64, t668: f64, t991: f64, t992: f64, t994: f64) -> (f64, f64, f64, f64) {
    let t3884 = 0.053425_f64 * t138 * t1018 * t1036;
    let t3885 = t109 * t1040;
    let t3888 = 0.8591797547176487_f64 * t138 * t3885 * t1044;
    let t3889 = 0.03253074390090522_f64 * t138 * t3834 * t1055 + 0.10274_f64 * t138 * t109 * t991 * t994 - t3719 + 3.5089341735807875_f64 * t1061 * t3842 - 6.0_f64 * t992 * t668 * t1003 + 0.0016562821945185185_f64 * t30 * t1180 * t269 + 96.49187699215521_f64 * t1009 * t3851 * t666 + 0.0005696894717424259_f64 * t30 * t1180 * t282 + 51.94757731704439_f64 * t1061 * t3859 - 3.5089341735807875_f64 * t1054 * t3862 + t3867 - t3871 - t3874 - t3877 - t3881 + t3884 + t3888;
    (t3884, t3885, t3888, t3889)
}
