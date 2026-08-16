//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1311/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1311(t1851: f64, t2319: f64, t2363: f64, t576: f64, t4025: f64, t671: f64, t1441: f64, t1395: f64, t1453: f64, t2332: f64, t4067: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55405 = t1851 * t2319;
    let t55571 = t576 * t2363;
    let t55934 = t4025 * t671;
    let t55962 = t1441 * t2363;
    let t66940 = t1395 * t671;
    let t86592 = t1453 * t2332;
    let t86595 = t4067 * t666;
    (t55405, t55571, t55934, t55962, t66940, t86592, t86595)
}
