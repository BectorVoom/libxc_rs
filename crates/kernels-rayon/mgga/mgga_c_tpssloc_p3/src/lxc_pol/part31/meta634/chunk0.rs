//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1897/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1897(t22779: f64, t28060: f64, t19661: f64, t1992: f64, t22897: f64, t19736: f64, t22892: f64, t22893: f64, t28138: f64, t28116: f64, t81228: f64, t81326: f64) -> (f64, f64, f64, f64, f64) {
    let t97463 = t22779 * t28060;
    let t97488 = t1992 * t22897 * t19661;
    let t97491 = t1992 * t22897 * t19736;
    let t97494 = t22892 * t22893 * t28138;
    let t97503 = t81228 * t81326 * t28116;
    (t97463, t97488, t97491, t97494, t97503)
}
