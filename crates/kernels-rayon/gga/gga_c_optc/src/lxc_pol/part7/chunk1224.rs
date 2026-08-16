//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1224/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1224(t7931: f64, t907: f64, t2684: f64, t2693: f64, t7947: f64, t902: f64, t334: f64, t7946: f64, t317: f64, t2695: f64, t2818: f64, t2367: f64, t7925: f64, t930: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25256 = t7931 * t907;
    let t25260 = t2684 * t2693;
    let t25267 = t902 * t7947;
    let t25277 = 1.0_f64 / t7946 / t334;
    let t25278 = t317 * t25277;
    let t25279 = t2695 * t2695;
    let t25287 = t2818 * t2818;
    let t25297 = t930 * t2367 * t7925;
    (t25256, t25260, t25267, t25278, t25279, t25287, t25297)
}
