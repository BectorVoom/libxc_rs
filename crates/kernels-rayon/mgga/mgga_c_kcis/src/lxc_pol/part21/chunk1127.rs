//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1127/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1127(t1121: f64, t1800: f64, t27763: f64, t1092: f64, t1133: f64, t14628: f64, t26760: f64, t2909: f64, t417: f64, t1009: f64, t1704: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27764 = t1800 * t1121;
    let t27765 = t27763 * t27764;
    let t27766 = t1092 * t27765;
    let t27768 = t14628 * t1133;
    let t27769 = t26760 * t27768;
    let t27770 = t1092 * t27769;
    let t27772 = t417 * t2909;
    let t27773 = t1009 * t1704;
    (t27764, t27765, t27766, t27768, t27769, t27770, t27772, t27773)
}
