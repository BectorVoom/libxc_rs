//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 845/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk845(t2669: f64, t875: f64, t2608: f64, t140: f64, t7369: f64, t6: f64) -> (f64, f64, f64, f64) {
    let t8108 = t2669 * t875;
    let t8109 = t8108 * t2608;
    let t8112 = t7369 * t140;
    let t8113 = t8112 * t6;
    (t8108, t8109, t8112, t8113)
}
