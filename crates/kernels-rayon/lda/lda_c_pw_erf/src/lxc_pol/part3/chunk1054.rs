//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1054/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1054(t3850: f64, t4488: f64, t4490: f64, t12321: f64, t3403: f64, t806: f64, t4561: f64, t565: f64, t1522: f64, t184: f64, t1958: f64, t221: f64) -> (f64, f64, f64, f64) {
    let t12351 = 8.0_f64 / 15.0_f64 * t4488 * t4490 * t3850;
    let t12355 = 8.0_f64 / 9.0_f64 * t4488 * t12321 * t806 * t3403;
    let t12356 = t565 * t4561;
    let t12357 = 4.0_f64 / 45.0_f64 * t12356;
    let t12361 = 4.0_f64 / 5.0_f64 * t1522 * t1958 * t184 * t221;
    (t12351, t12355, t12357, t12361)
}
