//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1158/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1158(t5401: f64, t595: f64, t4010: f64, t808: f64, t10419: f64, t10422: f64, t10425: f64, t2061: f64, t830: f64, t11845: f64, t2062: f64, t11: f64, t13290: f64, t1349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13556 = 4.0_f64 / 5.0_f64 * t5401 * t595;
    let t13558 = 2.0_f64 / 15.0_f64 * t4010 * t808;
    let t13559 = 32.0_f64 / 135.0_f64 * t10419;
    let t13560 = 8.0_f64 / 45.0_f64 * t10422;
    let t13561 = 4.0_f64 / 45.0_f64 * t10425;
    let t13562 = t2061 * t830;
    let t13564 = t11845 * t2062;
    let t13568 = t11 * t1349 * t13290;
    (t13556, t13558, t13559, t13560, t13561, t13562, t13564, t13568)
}
