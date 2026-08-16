//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 849/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk849(t29948: f64, t301: f64, t694: f64, t1268: f64, t1679: f64, t2541: f64, t8022: f64, t96: f64, t1674: f64, t7278: f64, t922: f64, t811: f64, t9097: f64) -> (f64, f64, f64, f64, f64) {
    let t29950 = t694 * t29948 * t301;
    let t29953 = t1679 * t2541 * t1268;
    let t29955 = t96 * t8022;
    let t29958 = t1674 * t7278 * t922;
    let t29961 = t1679 * t9097 * t811;
    (t29950, t29953, t29955, t29958, t29961)
}
