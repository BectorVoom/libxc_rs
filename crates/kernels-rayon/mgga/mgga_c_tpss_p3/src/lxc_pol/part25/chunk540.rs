//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 540/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk540(t2576: f64, t2453: f64, t891: f64, t895: f64, t314: f64, t894: f64) -> (f64, f64, f64, f64) {
    let t2577 = 1.0_f64 / t2576;
    let t2581 = 0.12361111111111111111e-1_f64 * t2453;
    let t2589 = t891 * t895;
    let t2592 = t894 * t314;
    let t2593 = 1.0_f64 / t2592;
    (t2577, t2581, t2589, t2593)
}
