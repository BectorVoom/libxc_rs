//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1149/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1149(t1713: f64, t9568: f64, t1020: f64, t3178: f64, t4824: f64, t1092: f64, t1133: f64, t4772: f64, t1131: f64, t1096: f64, t1767: f64, t3190: f64) -> (f64, f64, f64, f64, f64) {
    let t14584 = t9568 * t1713;
    let t14585 = t1020 * t14584;
    let t14587 = t3178 * t4824;
    let t14588 = t1092 * t14587;
    let t14590 = t4772 * t1133;
    let t14591 = t1131 * t14590;
    let t14592 = t1096 * t14591;
    let t14593 = t1092 * t14592;
    let t14595 = t1767 * t3190;
    (t14585, t14588, t14590, t14593, t14595)
}
