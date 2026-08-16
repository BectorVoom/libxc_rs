//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1001/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1001(t2018: f64, t26161: f64, t26558: f64, t3914: f64, t23938: f64, t6535: f64, t26977: f64, t22561: f64, t7042: f64, t114422: f64, t111: f64, t31699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115227 = 2.0_f64 * t26161 * t26558 * t2018 * t3914;
    let t115229 = 4.0_f64 * t23938 * t6535;
    let t115231 = 4.0_f64 * t26977 * t6535;
    let t115233 = 4.0_f64 * t7042 * t22561;
    let t115238 = 4.0_f64 * t26161 * t26558 * t114422;
    let t115241 = t31699 * t111;
    (t115227, t115229, t115231, t115233, t115238, t115241)
}
