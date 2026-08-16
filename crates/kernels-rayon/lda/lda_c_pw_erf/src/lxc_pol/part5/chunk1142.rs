//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1142/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1142(t16140: f64, t184: f64, t1980: f64, t199: f64, t2405: f64, t2023: f64, t7007: f64, t16159: f64, t4763: f64, t6277: f64, t4753: f64, t7680: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21060 = 64.0_f64 / 45.0_f64 * t16140;
    let t21064 = 4.0_f64 / 5.0_f64 * t2405 * t1980 * t184 * t199;
    let t21066 = 8.0_f64 / 15.0_f64 * t7007 * t2023;
    let t21067 = 32.0_f64 / 15.0_f64 * t16159;
    let t21069 = 8.0_f64 / 15.0_f64 * t4763 * t6277;
    let t21071 = 16.0_f64 / 15.0_f64 * t4753 * t7680;
    (t21060, t21064, t21066, t21067, t21069, t21071)
}
