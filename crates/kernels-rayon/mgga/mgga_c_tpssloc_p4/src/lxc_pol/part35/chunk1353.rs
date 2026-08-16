//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1353/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1353(t20756: f64, t6553: f64, t6554: f64, t81984: f64, t1527: f64, t22986: f64, t23270: f64, t98253: f64, t1880: f64, t21033: f64, t6571: f64, t21049: f64, t82252: f64) -> (f64, f64, f64, f64) {
    let t105232 = t81984 * t6553 * t6554 * t20756;
    let t105240 = t22986 * t23270 * t98253 * t1527;
    let t105250 = t1880 * t6553 * t6571 * t21033;
    let t105254 = t1880 * t6553 * t82252 * t21049;
    (t105232, t105240, t105250, t105254)
}
