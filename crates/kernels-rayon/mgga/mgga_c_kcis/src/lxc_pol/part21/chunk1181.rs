//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1181/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1181(t291: f64, t9916: f64, t417: f64, t9874: f64, t209: f64, t736: f64, t1003: f64, t167: f64, t1646: f64, t3040: f64, t2887: f64, t1141: f64, t14663: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44575 = t9916 * t291;
    let t44657 = t417 * t9874;
    let t44682 = t209 * t736;
    let t44684 = t167 * t1003;
    let t44743 = t1646 * t3040;
    let t44756 = t2887 * t291;
    let t46015 = t14663 * t1141;
    (t44575, t44657, t44682, t44684, t44743, t44756, t46015)
}
