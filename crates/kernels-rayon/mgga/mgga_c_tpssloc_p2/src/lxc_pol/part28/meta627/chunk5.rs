//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1960/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1960(t23993: f64, t7428: f64, t23998: f64, t1860: f64, t23992: f64, t7445: f64, t26028: f64, t7032: f64, t26016: f64, t84173: f64, t2032: f64, t22534: f64, t23970: f64, t7782: f64, t84237: f64, t90098: f64, t90101: f64, t90104: f64, t90132: f64, t90137: f64, t90153: f64) -> f64 {
    let t91996 = t7428 * t23993;
    let t92001 = 16.0_f64 / 9.0_f64 * t7428 * t23998;
    let t92003 = t1860 * t23992 * t7445;
    let t92008 = 16.0_f64 / 9.0_f64 * t26028 * t7032;
    let t92012 = 160.0_f64 / 9.0_f64 * t26016 * t84173;
    let t92019 = -4.0_f64 / 3.0_f64 * t90153 * t2032 + 88.0_f64 / 27.0_f64 * t91996 - 2.0_f64 / 3.0_f64 * t90132 * t2032 - t92001 + 88.0_f64 / 27.0_f64 * t92003 - 2.0_f64 / 3.0_f64 * t22534 * t7782 - t92008 - 20.0_f64 * t90137 * t84237 - t92012 + 20.0_f64 / 3.0_f64 * t90098 * t23970 + 20.0_f64 / 3.0_f64 * t90101 * t23970 + 20.0_f64 / 3.0_f64 * t90104 * t23970;
    t92019
}
