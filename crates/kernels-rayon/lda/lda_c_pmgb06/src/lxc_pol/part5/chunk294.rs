//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 294/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk294(t1025: f64, t633: f64, t1024: f64, t942: f64, t945: f64, t947: f64, t951: f64, t953: f64, t955: f64) -> (f64, f64, f64) {
    let t1026 = t1025 * t633;
    let t1028 = 2.0_f64 * t1024 * t1026;
    let t1035 = -0.4219833333333333_f64 * t942 + 0.8439666666666666_f64 * t945 + 0.3986222222222222_f64 * t947 + 0.06825833333333334_f64 * t951 + 0.13651666666666668_f64 * t953 + 0.1369277777777778_f64 * t955;
    (t1026, t1028, t1035)
}
