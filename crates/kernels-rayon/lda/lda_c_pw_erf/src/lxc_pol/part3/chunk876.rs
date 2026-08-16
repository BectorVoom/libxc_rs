//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 876/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk876(t36: f64, t8512: f64, t88: f64, t3165: f64, t338: f64, t1063: f64, t35: f64, t8327: f64, t1035: f64, t1064: f64, t1039: f64, t3128: f64, t3130: f64, t905: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8516 = 840.0_f64 * t36 / t8512 * t88;
    let t8518 = t338 * t3165 * t88;
    let t8520 = t35 * t1063;
    let t8524 = t8327 * t88;
    let t8527 = 120.0_f64 * t1064 * t1035;
    let t8528 = t1064 * t1039;
    let t8533 = 3103.50088234237_f64 * t3128 * t935 * t3130 * t905;
    (t8516, t8518, t8520, t8524, t8527, t8528, t8533)
}
