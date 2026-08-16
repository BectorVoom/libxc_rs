//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 482/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk482(t676: f64, t739: f64, t172: f64, t2368: f64, t2369: f64, t746: f64, t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64) -> (f64, f64, f64, f64) {
    let t2490 = t676 * t739;
    let t2494 = t172 * t2368;
    let t2495 = t2369 * t746;
    let t2504 = -0.57538888888888888889e0_f64 * t2388 + 0.11507777777777777778e1_f64 * t2391 + 0.40256666666666666667e0_f64 * t2394 + 0.366775e-1_f64 * t2398 + 0.73355e-1_f64 * t2400 + 0.137975e0_f64 * t2403;
    (t2490, t2494, t2495, t2504)
}
