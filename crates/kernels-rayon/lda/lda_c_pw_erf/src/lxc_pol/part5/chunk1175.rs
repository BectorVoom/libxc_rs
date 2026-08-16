//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1175/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1175(t20711: f64, t549: f64, t3974: f64, t4515: f64, t3965: f64, t4479: f64, t6488: f64, t17123: f64, t17156: f64, t6631: f64, t808: f64, t9934: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21420 = t20711 * t549;
    let t21423 = 16.0_f64 / 15.0_f64 * t3974 * t4515 * t21420;
    let t21426 = 16.0_f64 / 15.0_f64 * t3965 * t4479 * t6488;
    let t21427 = 8.0_f64 / 15.0_f64 * t17123;
    let t21428 = 8.0_f64 / 27.0_f64 * t17156;
    let t21430 = 2.0_f64 / 5.0_f64 * t6631 * t808;
    let t21431 = 16.0_f64 / 405.0_f64 * t9934;
    (t21420, t21423, t21426, t21427, t21428, t21430, t21431)
}
