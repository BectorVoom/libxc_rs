//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1053/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1053(t1278: f64, t348: f64, t739: f64, t4488: f64, t4494: f64, t12118: f64, t4491: f64, t12329: f64, t1314: f64, t2098: f64, t4489: f64, t3846: f64, t4490: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12334 = t739 * t1278 * t348;
    let t12337 = 8.0_f64 / 15.0_f64 * t4488 * t4494 * t12334;
    let t12338 = t12118 * t4491;
    let t12339 = 32.0_f64 / 45.0_f64 * t12338;
    let t12341 = 16.0_f64 / 15.0_f64 * t12329 * t4491;
    let t12345 = 16.0_f64 / 15.0_f64 * t4488 * t4489 * t2098 * t1314;
    let t12348 = 8.0_f64 / 15.0_f64 * t4488 * t4490 * t3846;
    (t12334, t12337, t12339, t12341, t12345, t12348)
}
