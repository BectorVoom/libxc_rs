//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 319/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk319(t1101: f64, t286: f64, t1067: f64, t1090: f64, t1093: f64, t1095: f64, t1098: f64, t248: f64, t961: f64, t970: f64, t972: f64, t975: f64, t982: f64) -> (f64, f64) {
    let t1103 = 20.0_f64 * t1101 * t286;
    let t1104 = -t961 - t970 - 1.1696447245269292_f64 * t972 - 0.0003662289461201309_f64 * t975 + t982 + t1067 + t248 * t1090 + 2.0_f64 * t1093 - 8.0_f64 * t1095 - t1098 + t1103;
    (t1103, t1104)
}
