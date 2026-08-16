//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1095/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1095(t14655: f64, t10: f64, t14796: f64, t1832: f64, t1856: f64, t20359: f64, t20371: f64, t20374: f64, t20376: f64, t2610: f64, t2624: f64, t411: f64, t426: f64, t6121: f64, t767: f64, t7927: f64, t7930: f64) -> (f64, f64) {
    let t20390 = 5.84605_f64 * t14655;
    let t20391 = 0.73452_f64 * t20359 + 30.0_f64 * t426 * t10 * t7930 * t411 - 18.0_f64 * t426 * t10 * t2624 * t1832 - 8.81424_f64 * t20371 - t20374 - 3.0_f64 / 2.0_f64 * t20376 + 9.0_f64 / 2.0_f64 * t426 * t10 * t1856 * t2610 + 9.0_f64 / 2.0_f64 * t426 * t10 * t767 * t6121 + 3.0_f64 / 2.0_f64 * t426 * t10 * t7927 * t411 + t20390 - t14796;
    (t20390, t20391)
}
