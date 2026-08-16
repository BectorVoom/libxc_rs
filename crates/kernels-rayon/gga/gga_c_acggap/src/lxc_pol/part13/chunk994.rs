//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 994/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk994(t33682: f64, t7887: f64, t7990: f64, t9076: f64, t1620: f64, t7973: f64, t2331: f64, t323: f64, t851: f64, t2137: f64, t32123: f64, t1619: f64, t322: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33683 = t33682 * t7887;
    let t33686 = 0.34694512752820797848e1_f64 * t7990 * t9076;
    let t33691 = 0.26341796731742046394e1_f64 * t7973 * t1620;
    let t33695 = t851 * t2331 * t323;
    let t33698 = t2137 * t32123;
    let t33699 = t1619 * t322;
    (t33683, t33686, t33691, t33695, t33698, t33699)
}
