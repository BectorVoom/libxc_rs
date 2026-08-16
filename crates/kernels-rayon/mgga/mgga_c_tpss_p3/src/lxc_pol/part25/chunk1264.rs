//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1264/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1264(t21907: f64, t485: f64, t1795: f64, t4637: f64, t1165: f64, t1338: f64, t13565: f64, t1799: f64, t20289: f64, t21180: f64, t21227: f64, t21786: f64, t3493: f64, t4674: f64, t5801: f64, t6234: f64, t6323: f64) -> (f64, f64, f64) {
    let t21908 = t485 * t21907;
    let t21922 = t1795 * t4637;
    let t21944 = 2.0_f64 * t1165 * t21907 + 4.0_f64 * t1338 * t20289 + 2.0_f64 * t13565 * t1799 + 4.0_f64 * t1799 * t21180 + 2.0_f64 * t1799 * t21227 + 4.0_f64 * t3493 * t6323 + 2.0_f64 * t4674 * t5801 + 4.0_f64 * t6234 * t6323 + t21786 + 2.0_f64 * t21922;
    (t21908, t21922, t21944)
}
