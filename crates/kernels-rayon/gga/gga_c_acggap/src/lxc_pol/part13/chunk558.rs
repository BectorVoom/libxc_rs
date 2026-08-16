//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 558/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk558(t322: f64, t3828: f64, t381: f64, t1240: f64, t879: f64, t1004: f64, t1241: f64, t1248: f64, t377: f64, t1261: f64, t310: f64, t1244: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3829 = t3828 * t322;
    let t3830 = t381 * t3829;
    let t3832 = t1240 * t879;
    let t3833 = t381 * t3832;
    let t3835 = t1004 * t1241;
    let t3837 = t377 * t1248;
    let t3839 = t310 * t1261;
    let t3842 = 0.19756347548806534796e1_f64 * t1004 * t1244;
    (t3830, t3833, t3835, t3837, t3839, t3842)
}
