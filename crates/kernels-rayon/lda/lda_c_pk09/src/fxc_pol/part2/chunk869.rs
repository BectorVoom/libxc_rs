//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 869/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk869(t1011: f64, t2355: f64, t3173: f64, t3177: f64, t3191: f64, t4168: f64, t4170: f64, t4177: f64, t7801: f64, t7805: f64, t7809: f64, t7811: f64, t7814: f64, t7817: f64, t7834: f64) -> f64 {
    let t9032 = t4168 - t4170 - t2355 * t1011 / 6.0_f64 - t4177 + 0.06825182070127836_f64 * t7801 + 0.10237773105191754_f64 * t7805 + 0.10237773105191754_f64 * t7809 + 0.10237773105191754_f64 * t7811 + 0.10237773105191754_f64 * t7814 + 0.10237773105191754_f64 * t7817 + 0.10237773105191754_f64 * t7834 + 0.02466859483068398_f64 * t3173 - 0.02466859483068398_f64 * t3177 + 0.02466859483068398_f64 * t3191;
    t9032
}
