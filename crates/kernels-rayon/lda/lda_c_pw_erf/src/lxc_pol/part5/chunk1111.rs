//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1111/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1111(t20701: f64, t3974: f64, t5160: f64, t5166: f64, t18188: f64, t2026: f64, t3965: f64, t2334: f64, t833: f64, t352: f64, t13829: f64, t4506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20704 = 16.0_f64 / 15.0_f64 * t3974 * t5160 * t20701;
    let t20707 = 8.0_f64 / 9.0_f64 * t3974 * t5166 * t20701;
    let t20710 = 8.0_f64 / 15.0_f64 * t3965 * t18188 * t2026;
    let t20711 = t2334 * t833;
    let t20712 = t20711 * t352;
    let t20715 = 32.0_f64 / 27.0_f64 * t4506 * t13829 * t20712;
    (t20704, t20707, t20710, t20711, t20712, t20715)
}
