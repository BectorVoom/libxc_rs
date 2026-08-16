//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1124/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1124(t4506: f64, t4508: f64, t6396: f64, t17645: f64, t1949: f64, t12428: f64, t1944: f64, t2466: f64, t10011: f64, t7749: f64, t16867: f64, t2030: f64, t4488: f64) -> (f64, f64, f64, f64, f64) {
    let t20861 = 16.0_f64 / 15.0_f64 * t4506 * t4508 * t6396;
    let t20864 = 16.0_f64 / 15.0_f64 * t4506 * t17645 * t1949;
    let t20868 = 8.0_f64 / 9.0_f64 * t4506 * t12428 * t2466 * t1944;
    let t20869 = t10011 * t7749;
    let t20870 = 32.0_f64 / 45.0_f64 * t20869;
    let t20873 = 8.0_f64 / 5.0_f64 * t4488 * t16867 * t2030;
    (t20861, t20864, t20868, t20870, t20873)
}
