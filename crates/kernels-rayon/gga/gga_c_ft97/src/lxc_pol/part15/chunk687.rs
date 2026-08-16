//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 687/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk687(t20113: f64, t7750: f64, t27: f64, t89: f64, t3013: f64, t4495: f64, t28: f64, t11076: f64, t15606: f64, t15609: f64, t15612: f64, t15899: f64, t20101: f64, t20105: f64, t20109: f64, t8190: f64) -> (f64, f64, f64, f64, f64) {
    let t20114 = t7750 * t20113;
    let t20116 = t89 * t27 * t20114;
    let t20117 = t3013 * t4495;
    let t20119 = t89 * t28 * t20117;
    let t20123 = -t20101 / 6.0_f64 - t20105 / 3.0_f64 - t20109 / 3.0_f64 - t15899 / 9.0_f64 - t8190 - 2.0_f64 / 9.0_f64 * t11076 - t20116 + t20119 - t15609 / 9.0_f64 + t15612 / 18.0_f64 + t15606 / 27.0_f64;
    (t20114, t20116, t20117, t20119, t20123)
}
