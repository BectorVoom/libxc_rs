//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1151/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1151(t3813: f64, t4772: f64, t16980: f64, t2678: f64, t40480: f64, t17068: f64, t24: f64, t862: f64, t4776: f64, t16997: f64, t2586: f64, t893: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51027 = t3813 * t4772;
    let t51035 = t2678 * t40480 * t16980;
    let t51085 = t862 * t24 * t17068;
    let t51102 = t3813 * t4776;
    let t51125 = t2586 * t16997;
    let t51126 = t893 * t51125;
    (t51027, t51035, t51085, t51102, t51125, t51126)
}
