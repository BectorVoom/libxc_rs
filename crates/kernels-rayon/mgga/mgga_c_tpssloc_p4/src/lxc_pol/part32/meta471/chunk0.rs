//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1765/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1765(t225: f64, t7319: f64, t23598: f64, t50: f64, t131: f64, t467: f64) -> (f64, f64, f64, f64) {
    let t24788 = t7319 * t225;
    let t24810 = t50 * t23598;
    let t24811 = t24810 * t131;
    let t24812 = t24811 * t467;
    (t24788, t24810, t24811, t24812)
}
