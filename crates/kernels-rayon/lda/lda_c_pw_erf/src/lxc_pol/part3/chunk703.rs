//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 703/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk703(t4410: f64, t1077: f64, t1765: f64, t1: f64, t1750: f64, t887: f64, t1755: f64, t1746: f64, t1769: f64, t2951: f64, t1904: f64, t462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4411 = 0.019751789702565206_f64 * t4410;
    let t4412 = t1765 * t1077;
    let t4413 = 1.169644679491041_f64 * t4412;
    let t4415 = t887 * t1750 * t1;
    let t4416 = t4415 * t1755;
    let t4418 = t1769 * t1746;
    let t4420 = 2.339289358982082_f64 * t2951;
    let t4422 = t462 * t1904;
    (t4411, t4412, t4413, t4415, t4416, t4418, t4420, t4422)
}
