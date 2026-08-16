//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 908/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk908(t1437: f64, t5445: f64, t1864: f64, t5398: f64, t1426: f64, t5392: f64, t584: f64, t9212: f64) -> (f64, f64, f64, f64, f64) {
    let t20204 = t1437 * t5445;
    let t20207 = t1864 * t5398;
    let t20210 = t5392 * t1426;
    let t20215 = -t584 - t9212;
    let t20216 = 6.0_f64 * t20215;
    (t20204, t20207, t20210, t20215, t20216)
}
