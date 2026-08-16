//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1122/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1122(t10462: f64, t1697: f64, t1130: f64, t4772: f64, t2835: f64, t4768: f64, t14453: f64, t291: f64, t9916: f64, t417: f64, t9874: f64, t1003: f64, t1704: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42972 = t1697 * t10462;
    let t43053 = t1130 * t4772;
    let t43526 = t4768 * t2835;
    let t44544 = t14453 * t291;
    let t44575 = t9916 * t291;
    let t44657 = t417 * t9874;
    let t44658 = t1704 * t1003;
    (t42972, t43053, t43526, t44544, t44575, t44657, t44658)
}
