//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 954/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk954(t330: f64, t4972: f64, t829: f64, t2894: f64, t13462: f64, t4939: f64, t291: f64, t9897: f64, t13467: f64, t2887: f64, t736: f64, t13516: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14484 = t4972 * t330;
    let t14485 = t14484 * t829;
    let t14486 = t2894 * t14485;
    let t14489 = t4939 * t13462;
    let t14492 = t9897 * t291;
    let t14493 = t14492 * t13467;
    let t14496 = t736 * t2887;
    let t14497 = t14496 * t291;
    let t14498 = t14497 * t13516;
    (t14486, t14489, t14492, t14493, t14496, t14497, t14498)
}
