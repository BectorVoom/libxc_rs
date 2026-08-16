//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1315/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1315(t4025: f64, t671: f64, t1441: f64, t2363: f64, t1395: f64, t1453: f64, t2332: f64, t4067: f64, t666: f64, t2358: f64, t4072: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55934 = t4025 * t671;
    let t55962 = t1441 * t2363;
    let t66940 = t1395 * t671;
    let t86592 = t1453 * t2332;
    let t86595 = t4067 * t666;
    let t86598 = t1453 * t2358;
    let t90370 = t649 * t4072;
    (t55934, t55962, t66940, t86592, t86595, t86598, t90370)
}
