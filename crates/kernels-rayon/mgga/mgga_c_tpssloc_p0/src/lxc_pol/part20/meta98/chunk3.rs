//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 669/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk669(t2461: f64, t731: f64, t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64) -> (f64, f64) {
    let t2462 = t2461 * t731;
    let t2471 = -0.78438333333333333333e0_f64 * t2388 + 0.15687666666666666667e1_f64 * t2391 + 0.68863333333333333333e0_f64 * t2394 + 0.14025833333333333333e0_f64 * t2398 + 0.28051666666666666667e0_f64 * t2400 + 0.17365833333333333333e0_f64 * t2403;
    (t2462, t2471)
}
