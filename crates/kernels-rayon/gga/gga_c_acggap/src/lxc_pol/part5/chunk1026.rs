//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1026/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1026(t14283: f64, t542: f64, t4886: f64, t997: f64, t1576: f64, t3237: f64, t13502: f64, t537: f64, t5237: f64, t1032: f64, t4557: f64, t1008: f64, t5267: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17501 = t14283 * t542;
    let t17503 = t997 * t4886;
    let t17505 = t3237 * t1576;
    let t17507 = t13502 * t537;
    let t17509 = t997 * t5237;
    let t17511 = t1032 * t4557;
    let t17513 = t1008 * t5267;
    (t17501, t17503, t17505, t17507, t17509, t17511, t17513)
}
