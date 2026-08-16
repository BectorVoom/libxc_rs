//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 984/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk984(t1985: f64, t8458: f64, t97511: f64, t120550: f64, t120568: f64, t120576: f64, t120446: f64, t120458: f64, t1998: f64, t214: f64, t28107: f64, t120470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t127349 = 0.16449340668482264365e-1_f64 * t1985 * t97511 * t8458;
    let t127350 = 0.16449340668482264365e-1_f64 * t120550;
    let t127354 = 0.16449340668482264365e-1_f64 * t120568;
    let t127355 = 0.16449340668482264365e-1_f64 * t120576;
    let t127356 = 0.76763589786250567036e-1_f64 * t120446;
    let t127357 = 0.16449340668482264365e-1_f64 * t120458;
    let t127361 = 0.16449340668482264365e-1_f64 * t1985 * t214 * t1998 * t28107;
    let t127362 = 0.15352717957250113407e0_f64 * t120470;
    (t127349, t127350, t127354, t127355, t127356, t127357, t127361, t127362)
}
