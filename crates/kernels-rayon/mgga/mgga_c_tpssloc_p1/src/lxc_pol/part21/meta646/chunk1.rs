//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2440/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2440(t3032: f64, t42741: f64, t3038: f64, t1043: f64, t204: f64, t1041: f64, t248: f64, t884: f64, t10189: f64, t3014: f64, t10337: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42742 = t42741 * t3032;
    let t42743 = t42742 * t3038;
    let t42749 = t204 * t1043;
    let t42752 = t1041 * t248 * t42749 * t884;
    let t42771 = t10189 * t3014;
    let t42811 = t964 * t10337;
    (t42742, t42743, t42749, t42752, t42771, t42811)
}
