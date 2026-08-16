//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1064/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1064(t1181: f64, t30698: f64, t38789: f64, t599: f64, t1479: f64, t535: f64, t7380: f64, t4643: f64, t8489: f64, t2095: f64, t1988: f64, t9543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38792 = t30698 * t1181 * t599 * t38789;
    let t38795 = t535 * t1479;
    let t38796 = t7380 * t38795;
    let t38798 = t4643 * t8489;
    let t38799 = t2095 * t38798;
    let t38801 = t1988 * t9543;
    (t38792, t38795, t38796, t38798, t38799, t38801)
}
