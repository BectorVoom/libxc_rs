//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 336/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk336(t1069: f64, t1072: f64, t1444: f64, t1451: f64, t1454: f64, t1457: f64) -> f64 {
    let t1471 = 0.3529725e1_f64 * t1451 - t1069 - 0.516475e0_f64 * t1444 + 0.6311625e0_f64 * t1454 - t1072 - 0.104195e0_f64 * t1457;
    t1471
}
