//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 288/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk288(t1652: f64, t338: f64, t118: f64, t1594: f64, t1596: f64, t1600: f64, t1603: f64, t1605: f64, t1607: f64, t1609: f64, t1616: f64, t1618: f64, t1620: f64, t1622: f64) -> (f64, f64) {
    let t1653 = t338 * t1652;
    let t1654 = t118 * t1653;
    let t1656 = -0.11974241701863808564e0_f64 * t1594 + 0.17961362552795712846e0_f64 * t1596 + 0.59871208509319042821e-1_f64 * t1600 - 0.59871208509319042821e-1_f64 * t1603 + 0.17961362552795712846e0_f64 * t1605 - 0.23948483403727617128e0_f64 * t1607 - 0.59871208509319042821e-1_f64 * t1609 + 0.59871208509319042821e-1_f64 * t1616 + 0.59871208509319042821e-1_f64 * t1618 - 0.59871208509319042821e-1_f64 * t1620 - 0.39914139006212695214e-1_f64 * t1622 + 0.19957069503106347607e-1_f64 * t1654;
    (t1654, t1656)
}
