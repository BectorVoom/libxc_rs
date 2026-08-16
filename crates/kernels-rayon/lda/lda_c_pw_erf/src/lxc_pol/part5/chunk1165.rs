//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1165/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1165(t16955: f64, t16957: f64, t16961: f64, t16963: f64, t9593: f64, t16989: f64, t2134: f64, t2443: f64, t12572: f64, t12616: f64, t6297: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21322 = 16.0_f64 / 45.0_f64 * t16955;
    let t21323 = 4.0_f64 / 15.0_f64 * t16957;
    let t21324 = 8.0_f64 / 45.0_f64 * t16961;
    let t21325 = 4.0_f64 / 15.0_f64 * t16963;
    let t21326 = 32.0_f64 / 405.0_f64 * t9593;
    let t21327 = 8.0_f64 / 15.0_f64 * t16989;
    let t21328 = t2443 * t2134;
    let t21329 = 4.0_f64 / 15.0_f64 * t21328;
    let t21330 = 32.0_f64 / 405.0_f64 * t12572;
    let t21331 = 32.0_f64 / 405.0_f64 * t12616;
    let t21332 = t822 * t6297;
    (t21322, t21323, t21324, t21325, t21326, t21327, t21329, t21330, t21331, t21332)
}
