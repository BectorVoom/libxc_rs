//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 532/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk532(t821: f64, t823: f64, t1693: f64, t262: f64, t265: f64, t664: f64, t838: f64) -> (f64, f64, f64, f64) {
    let t2440 = t821 * t823;
    let t2453 = t262 * t1693 * t265;
    let t2454 = 0.23744444444444444444e-1_f64 * t2453;
    let t2455 = t664 * t838;
    (t2440, t2453, t2454, t2455)
}
