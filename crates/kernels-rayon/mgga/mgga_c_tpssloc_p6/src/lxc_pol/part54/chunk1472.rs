//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1472/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1472(t116135: f64, t121240: f64, t121253: f64, t121254: f64, t122084: f64, t122088: f64, t122094: f64, t122583: f64, t122587: f64, t122589: f64, t122590: f64, t122593: f64, t2165: f64, t26872: f64, t26974: f64, t27170: f64, t652: f64) -> f64 {
    let t124977 = -2.0_f64 * t2165 * t27170 * t652 - 3.0_f64 * t116135 * t26872 - 3.0_f64 * t116135 * t26974 - t121240 - t121253 - t121254 - t122084 + t122088 + t122094 + t122583 + t122587 - t122589 - t122590 - t122593;
    t124977
}
