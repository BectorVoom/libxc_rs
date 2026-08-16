//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 439/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk439(t50: f64, t75: f64, t80: f64, t1279: f64, t299: f64, t1285: f64, t295: f64, t1328: f64, t78: f64, t76: f64, t1296: f64, t252: f64) -> (f64, f64, f64, f64, f64) {
    let t4695 = t75 * t50;
    let t4697 = 1320.0_f64 * t4695 * t80;
    let t4698 = t1279 * t299;
    let t4700 = t295 * t1285;
    let t4703 = 1.0_f64 / t78 / t1328;
    let t4705 = 2184.0_f64 * t76 * t4703;
    let t4709 = t1296 * t252;
    (t4697, t4698, t4700, t4705, t4709)
}
