//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1231/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1231(t1902: f64, t828: f64, t3701: f64, t6995: f64, t1351: f64, t2006: f64, t22811: f64, t604: f64, t9226: f64, t2233: f64, t2239: f64, t601: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30684 = t1902 * t828;
    let t31035 = t3701 * t6995;
    let t31201 = t2006 * t1351;
    let t39041 = 1.0_f64 / t22811;
    let t39046 = t9226 * t604;
    let t39049 = t2233 * t2239;
    let t39054 = t601 * t9238;
    (t30684, t31035, t31201, t39041, t39046, t39049, t39054)
}
