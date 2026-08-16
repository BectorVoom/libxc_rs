//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1054/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1054(t1562: f64, t8876: f64, t46736: f64, t739: f64, t16503: f64, t3369: f64, t571: f64, t8430: f64, t1357: f64, t34976: f64, t8435: f64, t10030: f64, t34761: f64) -> (f64, f64, f64, f64, f64) {
    let t47312 = t1562 * t8876;
    let t47316 = t739 * t46736;
    let t47321 = t16503 * t3369 * t571 * t8430;
    let t47325 = t16503 * t34976 * t1357 * t8435;
    let t47327 = t34761 * t10030;
    (t47312, t47316, t47321, t47325, t47327)
}
