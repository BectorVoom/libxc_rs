//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1331/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1331(t10360: f64, t1040: f64, t1043: f64, t204: f64, t1041: f64, t248: f64, t884: f64, t1009: f64, t10358: f64, t1011: f64, t1019: f64, t338: f64, t39177: f64) -> (f64, f64, f64, f64, f64) {
    let t42746 = t10360 * t1040;
    let t42749 = t204 * t1043;
    let t42752 = t1041 * t248 * t42749 * t884;
    let t42754 = t10358 * t1009;
    let t42756 = t42754 * t1011 * t1019;
    let t42759 = t39177 * t338;
    (t42746, t42752, t42754, t42756, t42759)
}
