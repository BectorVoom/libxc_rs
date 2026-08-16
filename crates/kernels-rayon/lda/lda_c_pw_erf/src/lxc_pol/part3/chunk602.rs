//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 602/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk602(t2017: f64, t3429: f64, t1318: f64, t1529: f64, t565: f64, t1524: f64, t568: f64, t2070: f64, t220: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3430 = t2017 * t3429;
    let t3432 = 8.0_f64 / 9.0_f64 * t1318 * t3430;
    let t3433 = t565 * t1529;
    let t3434 = 4.0_f64 / 45.0_f64 * t3433;
    let t3435 = t1524 * t568;
    let t3436 = 8.0_f64 / 15.0_f64 * t3435;
    let t3437 = t2070 * t220;
    (t3430, t3432, t3433, t3434, t3435, t3436, t3437)
}
