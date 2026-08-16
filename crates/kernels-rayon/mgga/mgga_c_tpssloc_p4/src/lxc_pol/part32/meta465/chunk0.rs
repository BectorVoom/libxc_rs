//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1753/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1753(t491: f64, t7319: f64, t3439: f64, t461: f64, t225: f64) -> (f64, f64, f64, f64, f64) {
    let t24590 = t7319 * t491;
    let t24594 = t3439 * t461;
    let t24595 = t24594 * t491;
    let t24600 = t461 * t491;
    let t24601 = t24600 * t225;
    (t24590, t24594, t24595, t24600, t24601)
}
