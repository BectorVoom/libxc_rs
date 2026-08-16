//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 959/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk959(t11700: f64, t1200: f64, t1565: f64, t16135: f64, t17574: f64, t17582: f64, t17585: f64, t17610: f64, t2886: f64, t4249: f64, t485: f64, t5458: f64, t5469: f64, t9304: f64) -> f64 {
    let t17612 = 6.0_f64 * t11700 * t5458 - t1200 * t17610 - 3.0_f64 * t16135 * t1565 + t17574 * t485 - 6.0_f64 * t9304 * t17582 + 6.0_f64 * t2886 * t17585 - 3.0_f64 * t4249 * t5469;
    t17612
}
