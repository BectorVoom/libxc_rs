//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1227/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1227(t14030: f64, t15727: f64, t3974: f64, t4818: f64, t568: f64, t7470: f64, t515: f64, t7466: f64, t10427: f64, t2146: f64, t6195: f64, t2188: f64, t6198: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22141 = 16.0_f64 / 9.0_f64 * t3974 * t14030 * t4818 * t15727;
    let t22142 = t7470 * t568;
    let t22143 = 8.0_f64 / 15.0_f64 * t22142;
    let t22144 = t7466 * t515;
    let t22145 = 8.0_f64 / 15.0_f64 * t22144;
    let t22146 = 16.0_f64 / 405.0_f64 * t10427;
    let t22148 = 4.0_f64 / 5.0_f64 * t2146 * t6195;
    let t22150 = 4.0_f64 / 5.0_f64 * t6198 * t2188;
    (t22141, t22143, t22145, t22146, t22148, t22150)
}
