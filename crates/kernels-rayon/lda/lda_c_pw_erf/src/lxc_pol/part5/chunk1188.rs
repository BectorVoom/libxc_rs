//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1188/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1188(t17434: f64, t34: f64, t473: f64, t16144: f64, t2479: f64, t266: f64, t17436: f64, t2497: f64, t806: f64, t1325: f64, t494: f64, t5289: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21576 = 8.0_f64 / 45.0_f64 * t17434;
    let t21577 = t34 * t473;
    let t21581 = 4.0_f64 / 5.0_f64 * t21577 * t16144 * t266 * t2479;
    let t21582 = 8.0_f64 / 15.0_f64 * t17436;
    let t21583 = t2497 * t806;
    let t21587 = 8.0_f64 / 5.0_f64 * t1325 * t5289 * t21583 * t494;
    (t21576, t21577, t21581, t21582, t21583, t21587)
}
