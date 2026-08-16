//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 833/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk833(t598: f64, t9691: f64, t1891: f64, t2001: f64, t1896: f64, t1901: f64, t1734: f64, t599: f64, t142: f64, t2030: f64, t1795: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9692 = t598 * t9691;
    let t9694 = t2001 * t1891;
    let t9696 = t2001 * t1896;
    let t9698 = t2001 * t1901;
    let t9700 = t599 * t1734;
    let t9701 = t142 * t9700;
    let t9702 = t2030 * t9701;
    let t9704 = t604 * t1795;
    (t9692, t9694, t9696, t9698, t9700, t9701, t9702, t9704)
}
