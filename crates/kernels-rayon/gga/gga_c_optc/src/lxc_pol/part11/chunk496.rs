//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 496/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk496(t3086: f64, t496: f64, t1: f64, t1244: f64, t598: f64, t104: f64, t95: f64, t176: f64, t185: f64, t102: f64, t108: f64, t110: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3284 = t3086 * t496;
    let t3305 = t1244 * t1;
    let t3306 = t3305 * t598;
    let t3308 = t95 * t104;
    let t3313 = t176 * t185;
    let t3314 = t102 * t108;
    let t3315 = t3314 * t110;
    (t3284, t3305, t3306, t3308, t3313, t3314, t3315)
}
