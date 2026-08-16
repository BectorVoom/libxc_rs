//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1178/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1178(t12012: f64, t1390: f64, t22573: f64, t6875: f64, t191: f64, t192: f64, t9419: f64, t12451: f64, t3701: f64, t24486: f64, t576: f64, t111: f64, t7222: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83863 = t1390 * t12012;
    let t83886 = t6875 * t22573;
    let t83904 = t9419 * t191 * t192;
    let t83911 = t3701 * t12451;
    let t84031 = t576 * t24486;
    let t84033 = t7222 * t111;
    (t83863, t83886, t83904, t83911, t84031, t84033)
}
