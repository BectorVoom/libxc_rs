//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1897/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1897(t1410: f64, t9228: f64, t2235: f64, t3961: f64, t3967: f64, t4072: f64, t649: f64, t12813: f64, t88: f64, t1458: f64, t2311: f64, t1845: f64, t3914: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t90337 = t9228 * t1410;
    let t90340 = t2235 * t3961;
    let t90343 = t2235 * t3967;
    let t90370 = t649 * t4072;
    let t90375 = t88 * t12813;
    let t90381 = t2311 * t1458;
    let t90437 = t1845 * t3914;
    (t90337, t90340, t90343, t90370, t90375, t90381, t90437)
}
