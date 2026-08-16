//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 154/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk154(t440: f64, t449: f64, t300: f64, t425: f64, t427: f64, t436: f64, t338: f64, t51: f64) -> (f64, f64, f64) {
    let t450 = t440 * t449;
    let t453 = t300 * (-0.310907e-1_f64 * t427 * t436 + t425 - 0.19751673498613801407e-1_f64 * t450);
    let t455 = 0.19751673498613801407e-1_f64 * t300 * t450;
    let t456 = t51 * t338;
    (t453, t455, t456)
}
