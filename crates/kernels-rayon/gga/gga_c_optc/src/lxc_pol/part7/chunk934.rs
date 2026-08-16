//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 934/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk934(t1: f64, t438: f64, t8905: f64, t450: f64, t140: f64, t446: f64, t7369: f64, t3183: f64, t1122: f64, t3105: f64) -> (f64, f64, f64, f64, f64) {
    let t8907 = t8905 * t1 * t438;
    let t8908 = t450 * t8907;
    let t8912 = t446 * t7369 * t140;
    let t8913 = t3183 * t8912;
    let t8914 = t3105 * t1122;
    (t8907, t8908, t8912, t8913, t8914)
}
