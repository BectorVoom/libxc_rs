//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1380/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1380(t3399: f64, t445: f64, t1143: f64, t3375: f64, t1124: f64, t3331: f64, t11282: f64, t440: f64, t11135: f64, t11203: f64, t1127: f64, t3355: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11292 = 1.0_f64 / t3399 / t445;
    let t11297 = t1143 * t3375;
    let t11303 = t1124 * t3331;
    let t11310 = t440 * t11282;
    let t11314 = 0.16068111111111111111e1_f64 * t11135;
    let t11317 = 0.46308888888888888888e0_f64 * t11203;
    let t11349 = 1.0_f64 / t3355 / t1127;
    (t11292, t11297, t11303, t11310, t11314, t11317, t11349)
}
