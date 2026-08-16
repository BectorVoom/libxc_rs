//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 631/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk631(t1977: f64, t4606: f64, t1251: f64, t34: f64, t817: f64, t925: f64, t1945: f64, t325: f64, t1950: f64, t1955: f64, t1333: f64, t462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4607 = t4606 * t1977;
    let t4632 = t1251 * t34;
    let t4657 = t925 * t817;
    let t4659 = t325 * t1945;
    let t4661 = t325 * t1950;
    let t4662 = 0.002518888888888889_f64 * t4661;
    let t4663 = t4606 * t1955;
    let t4688 = t1333 * t34;
    let t4711 = 4.0_f64 * t462;
    (t4607, t4632, t4657, t4659, t4661, t4662, t4663, t4688, t4711)
}
