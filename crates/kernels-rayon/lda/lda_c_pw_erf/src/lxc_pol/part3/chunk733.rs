//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 733/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk733(t817: f64, t925: f64, t1945: f64, t325: f64, t1950: f64, t1955: f64, t4606: f64, t3589: f64, t743: f64, t951: f64, t3633: f64, t11: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4657 = t925 * t817;
    let t4659 = t325 * t1945;
    let t4661 = t325 * t1950;
    let t4662 = 0.002518888888888889_f64 * t4661;
    let t4663 = t4606 * t1955;
    let t4665 = t3589 * t743;
    let t4666 = t4665 * t951;
    let t4667 = t3633 * t4666;
    let t4668 = t11 * t4667;
    (t4657, t4659, t4661, t4662, t4663, t4665, t4666, t4667, t4668)
}
