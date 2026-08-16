//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 432/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk432(t25: f64, t28: f64, t522: f64, t588: f64, t592: f64, t514: f64, t606: f64, t1081: f64, t517: f64, t157: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1274 = 4.0_f64 * t588 * t522;
    let t1276 = 4.0_f64 * t592 * t522;
    let t1279 = piecewise3(t26, 0.0_f64, 4.0_f64 / 3.0_f64 * t514 * t606);
    let t1282 = piecewise3(t29, 0.0_f64, 4.0_f64 / 3.0_f64 * t517 * t1081);
    let t1284 = (t1279 + t1282) * t157;
    (t1274, t1276, t1284)
}
