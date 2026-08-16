//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 738/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk738(t2043: f64, t432: f64, t1395: f64, t2064: f64, t137: f64, t132: f64, t3058: f64, t822: f64, t1512: f64, t824: f64, t443: f64, t472: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4973 = t432 * t2043 / 15.0_f64;
    let t4974 = t1395 * t2064;
    let t4975 = t137 * t4974;
    let t4977 = t132 * t4975 / 15.0_f64;
    let t4978 = t3058 * t822;
    let t4979 = t137 * t4978;
    let t4981 = t132 * t4979 / 30.0_f64;
    let t4983 = t1512 * t824 / 30.0_f64;
    let t4989 = t472 * t443;
    (t4973, t4974, t4975, t4977, t4978, t4979, t4981, t4983, t4989)
}
