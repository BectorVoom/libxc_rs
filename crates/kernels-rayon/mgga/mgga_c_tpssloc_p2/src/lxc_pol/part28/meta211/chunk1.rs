//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 962/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk962(t1637: f64, t3216: f64, t1068: f64, t1070: f64, t193: f64, t336: f64, t4353: f64, t4356: f64, t4358: f64, t4361: f64, t4398: f64, t4402: f64, t4480: f64, t4482: f64, t4485: f64, t4487: f64, t4491: f64, t4495: f64, t4500: f64, t4696: f64, t4700: f64) -> (f64, f64) {
    let t4701 = t1637 * t3216;
    let t4704 = t1070 * t193 * t336 * t4696 - t1068 * t4700 * t4701 - t4353 + t4356 + t4358 - t4361 + t4398 + t4402 + t4480 + t4482 - t4485 - t4487 + t4491 - t4495 - t4500;
    (t4701, t4704)
}
