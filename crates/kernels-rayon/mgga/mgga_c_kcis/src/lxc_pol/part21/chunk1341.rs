//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1341/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1341(t26975: f64, t993: f64, t1856: f64, t330: f64, t1267: f64, t829: f64, t3530: f64, t417: f64, t1851: f64, t3622: f64, t26997: f64, t1268: f64, t9372: f64) -> (f64, f64, f64, f64, f64) {
    let t96735 = t993 * t26975;
    let t96736 = t1856 * t330;
    let t96737 = t829 * t1267;
    let t96739 = t96735 * t96736 * t96737;
    let t96742 = t417 * t3530;
    let t96743 = t3622 * t1851;
    let t96745 = t96742 * t96743 * t26997;
    let t96754 = t1268 * t9372;
    (t96736, t96737, t96739, t96745, t96754)
}
