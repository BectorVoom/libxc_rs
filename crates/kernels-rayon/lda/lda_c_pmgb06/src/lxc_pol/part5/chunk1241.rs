//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1241/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1241(t10079: f64, t10082: f64, t13244: f64, t20521: f64, t20523: f64, t20525: f64, t20529: f64, t20533: f64, t20536: f64, t20539: f64, t20541: f64, t20543: f64) -> f64 {
    let t22005 = t20521 - t20523 - t20525 - t20529 - t13244 - 8.0_f64 / 405.0_f64 * t10079 + t10082 + t20533 + t20536 - t20539 + t20541 + t20543;
    t22005
}
