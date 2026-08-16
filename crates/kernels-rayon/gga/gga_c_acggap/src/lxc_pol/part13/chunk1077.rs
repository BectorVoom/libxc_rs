//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1077/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1077(t1089: f64, t12473: f64, t2302: f64, t598: f64, t3201: f64, t8564: f64, t1980: f64, t7458: f64, t8569: f64, t1083: f64, t137: f64, t4875: f64) -> (f64, f64, f64, f64) {
    let t34775 = t598 * t1089 * t12473 * t2302;
    let t34779 = t598 * t1089 * t3201 * t8564;
    let t34783 = t1980 * t7458 * t3201 * t8569;
    let t34788 = t598 * t1089 * t1083 * t137 * t4875;
    (t34775, t34779, t34783, t34788)
}
