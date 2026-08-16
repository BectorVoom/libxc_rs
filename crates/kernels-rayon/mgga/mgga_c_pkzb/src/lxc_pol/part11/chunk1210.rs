//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1210/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1210(t192: f64, t3401: f64, t135: f64, t16810: f64, t16813: f64, t16822: f64, t20352: f64, t20359: f64, t20360: f64, t20363: f64, t2575: f64, t2718: f64, t29134: f64, t29137: f64, t7201: f64) -> f64 {
    let t29718 = t3401 * t192;
    let t29725 = 18.0_f64 * t135 * t2575 * t29718 + 18.0_f64 * t2718 * t3401 * t7201 + t16810 - t16813 - t16822 + t20352 - t20359 + t20360 - t20363 - t29134 - t29137;
    t29725
}
