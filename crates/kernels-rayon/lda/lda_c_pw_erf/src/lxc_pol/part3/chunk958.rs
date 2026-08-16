//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 958/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk958(t10953: f64, t169: f64, t289: f64, t3196: f64, t462: f64, t1089: f64, t39: f64, t343: f64, t678: f64, t2817: f64, t5: f64, t168: f64, t286: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10956 = 0.031835665774679375_f64 * t169 * t289 * t10953;
    let t10957 = t462 * t3196;
    let t10961 = t39 * t1089;
    let t10963 = t343 * t678;
    let t10967 = t5 * t2817;
    let t10970 = 0.9106331049773876_f64 * t168 * t10967 * t286;
    (t10956, t10957, t10961, t10963, t10967, t10970)
}
