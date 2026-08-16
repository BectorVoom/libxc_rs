//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 130/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk130(t353: f64, t68: f64, t336: f64, t225: f64, t293: f64, t328: f64, t330: f64) -> (f64, f64, f64, f64) {
    let t354 = t353 * t68;
    let t357 = 1.0_f64 / t336;
    let t358 = t68 * t357;
    let t360 = f64::exp(-(-t293 + t328 + t330) * t225 * t358);
    (t354, t357, t358, t360)
}
