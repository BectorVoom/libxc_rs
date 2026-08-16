//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1228/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1228(t2670: f64, t7467: f64, t2644: f64, t2668: f64, t115: f64, t2341: f64, t911: f64, t2718: f64, t297: f64, t7835: f64, t770: f64, t2811: f64, t7420: f64) -> (f64, f64, f64, f64, f64) {
    let t25355 = t7467 * t2670;
    let t25357 = t2668 * t25355 * t2644;
    let t25360 = t2341 * t911 * t115;
    let t25361 = t2718 * t25360;
    let t25364 = t7835 * t297;
    let t25365 = t25364 * t770;
    let t25369 = t2811 * t7420;
    (t25355, t25357, t25361, t25365, t25369)
}
