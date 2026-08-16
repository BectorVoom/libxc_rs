//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 282/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk282(t1027: f64, t1113: f64, t446: f64, t871: f64, t140: f64, t464: f64) -> (f64, f64, f64, f64) {
    let t1114 = t1113 * t1027;
    let t1119 = t446 * t871;
    let t1120 = t1119 * t140;
    let t1121 = t464 * t1120;
    (t1114, t1119, t1120, t1121)
}
