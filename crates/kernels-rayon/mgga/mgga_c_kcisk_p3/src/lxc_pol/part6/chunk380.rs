//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 380/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk380(t645: f64, t2436: f64, t2442: f64, t340: f64, t639: f64, t642: f64, sigma2: f64) -> f64 {
    let t646 = t645 < -0.66725e-1_f64;
    let t2447 = piecewise3(t646, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t2436 * t642 - 10.0_f64 / 27.0_f64 * t340 * t639 * t2442);
    let t2448 = t2447 * sigma2;
    t2448
}
