//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 501/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk501(t140: f64, t2086: f64, t1256: f64, t6: f64, t1281: f64, t669: f64, t106: f64, t145: f64, t1299: f64, t2105: f64, t146: f64, t692: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3440 = t2086 * t140;
    let t3441 = t6 * t1256;
    let t3454 = t1281 * t669;
    let t3461 = t106 * t145;
    let t3462 = t2105 * t1299;
    let t3466 = t146 * t692;
    (t3440, t3441, t3454, t3461, t3462, t3466)
}
