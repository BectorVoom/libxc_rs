//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 921/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk921(t17263: f64, t17275: f64, t1426: f64, t4835: f64, t4846: f64, t10348: f64, t13649: f64, t13651: f64, t13653: f64, t16650: f64, t16747: f64, t16750: f64, t16763: f64, t16766: f64, t8362: f64, t8364: f64) -> (f64, f64, f64, f64) {
    let t17276 = t17263 + t17275;
    let t17284 = t4835 * t1426;
    let t17287 = t1426 * t4846;
    let t17299 = 0.52444444444444444444e2_f64 * t13649 - 0.31466666666666666667e3_f64 * t13651 + 0.15733333333333333334e3_f64 * t13653 - t8362 - 0.72691666666666666667e3_f64 * t16650 - t8364 - 0.47199999999999999999e3_f64 * t16747 + 0.47199999999999999999e3_f64 * t16763 + 0.15733333333333333333e3_f64 * t16750 - 0.78666666666666666666e2_f64 * t16766 - 0.26222222222222222223e3_f64 * t10348;
    (t17276, t17284, t17287, t17299)
}
