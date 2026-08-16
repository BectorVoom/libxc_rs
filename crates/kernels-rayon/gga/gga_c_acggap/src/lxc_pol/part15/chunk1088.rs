//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1088/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1088(t38795: f64, t7380: f64, t4643: f64, t8489: f64, t2095: f64, t1988: f64, t9543: f64, t1089: f64, t3201: f64, t598: f64, t9541: f64, t1083: f64, t137: f64, t5784: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38796 = t7380 * t38795;
    let t38798 = t4643 * t8489;
    let t38799 = t2095 * t38798;
    let t38801 = t1988 * t9543;
    let t38805 = t598 * t1089 * t3201 * t9541;
    let t38810 = t598 * t1089 * t1083 * t137 * t5784;
    (t38796, t38798, t38799, t38801, t38805, t38810)
}
