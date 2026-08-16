//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1157/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1157(t31381: f64, t6562: f64, t2047: f64, t232: f64, t828: f64, t6646: f64, t1888: f64, t1894: f64, t7084: f64, t214: f64, t1880: f64, t814: f64, t8543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31382 = t6562 * t31381;
    let t31383 = 0.41123351671205660912e-2_f64 * t31382;
    let t31385 = t2047 * t828 * t232;
    let t31386 = t6646 * t31385;
    let t31387 = t1888 * t31386;
    let t31389 = t1894 * t7084;
    let t31390 = t214 * t31389;
    let t31391 = t1880 * t31390;
    let t31394 = t814 * t8543;
    (t31383, t31385, t31386, t31387, t31389, t31390, t31391, t31394)
}
