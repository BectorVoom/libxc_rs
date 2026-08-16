//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 409/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk409(t1524: f64, t221: f64, t565: f64, t568: f64, t1518: f64, t220: f64) -> (f64, f64, f64, f64) {
    let t1526 = 4.0_f64 / 15.0_f64 * t1524 * t221;
    let t1527 = t565 * t568;
    let t1528 = 8.0_f64 / 45.0_f64 * t1527;
    let t1529 = t1518 * t220;
    (t1526, t1527, t1528, t1529)
}
