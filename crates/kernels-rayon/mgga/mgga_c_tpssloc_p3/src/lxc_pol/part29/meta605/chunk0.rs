//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2042/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2042(t23272: f64, t81651: f64, t82074: f64, t23204: f64, t23218: f64, t6562: f64, t23171: f64, t23228: f64, t6572: f64, t212: f64, t6554: f64, t852: f64) -> (f64, f64, f64, f64) {
    let t82076 = t81651 * t82074 * t23272;
    let t82079 = t6562 * t23204 * t23218;
    let t82082 = t23171 * t23228 * t6572;
    let t82087 = t23171 * t212 * t852 * t6554;
    (t82076, t82079, t82082, t82087)
}
