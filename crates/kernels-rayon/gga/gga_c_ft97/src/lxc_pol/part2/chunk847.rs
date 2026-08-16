//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 847/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk847(t13246: f64, t184: f64, t1064: f64, t12092: f64, t12244: f64, t12253: f64, t12257: f64, t12261: f64, t1577: f64, t1580: f64, t185: f64, t21: f64, t2236: f64, t2240: f64, t2306: f64, t3597: f64, t3601: f64, t363: f64, t3660: f64, t3668: f64, t5: f64, t623: f64, t920: f64) -> f64 {
    let t13247 = t13246 * t184;
    let t13254 = t623 * t12092 + t623 * t12244 / 4.0_f64 + t3601 * t2306 / 4.0_f64 + t2240 * t3668 / 2.0_f64 + t2240 * t3660 / 2.0_f64 + t623 * t12253 / 2.0_f64 + t623 * t12257 / 2.0_f64 + t623 * t12261 / 4.0_f64 + t5 * t2236 * t920 / 4.0_f64 + t5 * t185 * t1577 / 2.0_f64 + t5 * t3597 * t363 / 2.0_f64 + t5 * t13247 * t21 / 4.0_f64 + t5 * t1064 * t1580 / 4.0_f64;
    t13254
}
