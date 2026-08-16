//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 395/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk395(t1450: f64, t519: f64, t523: f64, t945: f64, t522: f64, t187: f64, t504: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1451 = t519 * t1450;
    let t1452 = 16.0_f64 / 135.0_f64 * t1451;
    let t1453 = t523 * t945;
    let t1454 = t522 * t1453;
    let t1456 = 4.0_f64 / 45.0_f64 * t519 * t1454;
    let t1458 = 1.0_f64 / t187 / t504;
    (t1451, t1452, t1453, t1454, t1456, t1458)
}
