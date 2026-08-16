//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2476/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2476(t10408: f64, t1616: f64, t17187: f64, t17980: f64, t3070: f64, t3071: f64, t42552: f64, t4575: f64, t4650: f64, t49691: f64, t49693: f64, t50193: f64, t5677: f64, t61950: f64, t61981: f64, t62013: f64, t62032: f64, t62038: f64) -> f64 {
    let t70432 = t61981 / 2304.0_f64 + 5.0_f64 / 3888.0_f64 * t42552 + t50193 * t17980 / 1024.0_f64 + t3070 * t3071 * t17187 * t1616 / 1536.0_f64 + t61950 * t4575 / 1536.0_f64 + t62013 / 1152.0_f64 - t49691 - t49693 + 5.0_f64 / 4608.0_f64 * t3070 * t10408 * t5677 * t4650 + t62032 / 2304.0_f64 + 5.0_f64 / 3456.0_f64 * t62038;
    t70432
}
