//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1292/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1292(t19: f64, t9223: f64, t2239: f64, t601: f64, t83: f64, t84: f64, t85: f64, t24: f64) -> (f64, f64, f64, f64) {
    let t9225 = 0.75936e3_f64 * t19 * t9223;
    let t9231 = t601 * t2239;
    let t9238 = 1.0_f64 / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    (t9225, t9231, t9238, t9239)
}
