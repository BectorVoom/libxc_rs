//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1017/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1017(t2986: f64, t960: f64, t11132: f64, t1034: f64, t3154: f64, t357: f64, t1024: f64, t3105: f64, t905: f64, t606: f64, t1052: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11554 = t960 * t2986;
    let t11560 = 0.28842592592592592592e-1_f64 * t11132;
    let t11574 = 0.53272592592592592592e-1_f64 * t11132;
    let t11626 = t1034 * t1034;
    let t11627 = 1.0_f64 / t11626;
    let t11631 = t3154 * t357;
    let t11656 = t1024 * t3105;
    let t11660 = t3154 * t905;
    let t11661 = t11660 * t606;
    let t11670 = t360 * t1052;
    (t11554, t11560, t11574, t11627, t11631, t11656, t11661, t11670)
}
