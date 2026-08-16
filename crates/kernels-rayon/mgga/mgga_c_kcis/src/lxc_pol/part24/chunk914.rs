//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 914/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk914(t1092: f64, t19650: f64, t113: f64, t10443: f64, t18443: f64, t8: f64, t1131: f64, t1021: f64, t13106: f64, t1768: f64, t13322: f64, t4819: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19651 = t1092 * t19650;
    let t19653 = 2.0_f64 * t113;
    let t19655 = t18443 * t8 + t10443 + t19653;
    let t19656 = t1131 * t19655;
    let t19657 = t1021 * t19656;
    let t19658 = t1092 * t19657;
    let t19660 = t13106 * t1768;
    let t19661 = t1092 * t19660;
    let t19663 = t13322 * t4819;
    (t19651, t19655, t19656, t19658, t19661, t19663)
}
