//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1019/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1019(t1409: f64, t9330: f64, t2298: f64, t3966: f64, t12595: f64, t12598: f64, t12606: f64, t2244: f64, t2250: f64, t4007: f64, t4012: f64, t607: f64, t634: f64, t638: f64) -> f64 {
    let t12609 = t9330 * t1409;
    let t12612 = t2298 * t3966;
    let t12619 = -280.0_f64 / 27.0_f64 * t12595 * t2244 + 56.0_f64 / 9.0_f64 * t12598 * t607 + 28.0_f64 / 9.0_f64 * t4007 * t2250 - 4.0_f64 / 3.0_f64 * t634 * t12606 + 280.0_f64 / 27.0_f64 * t12609 * t2244 + 56.0_f64 / 9.0_f64 * t12612 * t607 + 28.0_f64 / 9.0_f64 * t4012 * t2250 + 4.0_f64 / 3.0_f64 * t638 * t12606;
    t12619
}
