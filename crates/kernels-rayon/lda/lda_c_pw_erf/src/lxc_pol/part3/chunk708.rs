//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 708/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk708(t3975: f64, t811: f64, t1309: f64, t3974: f64, t3966: f64, t784: f64, t1314: f64, t3965: f64, t806: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4475 = t3975 * t811;
    let t4476 = t4475 * t1309;
    let t4478 = 16.0_f64 / 45.0_f64 * t3974 * t4476;
    let t4479 = t3966 * t784;
    let t4480 = t4479 * t1314;
    let t4482 = 16.0_f64 / 45.0_f64 * t3965 * t4480;
    let t4483 = t3966 * t806;
    (t4475, t4476, t4478, t4479, t4480, t4482, t4483)
}
