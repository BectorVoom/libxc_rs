//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 469/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk469(t730: f64, t731: f64, t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64, t723: f64, t159: f64, t167: f64, t676: f64, t682: f64) -> (f64, f64, f64, f64, f64) {
    let t2461 = t730 * t730;
    let t2462 = t2461 * t731;
    let t2471 = -0.78438333333333333333e0_f64 * t2388 + 0.15687666666666666667e1_f64 * t2391 + 0.68863333333333333333e0_f64 * t2394 + 0.14025833333333333333e0_f64 * t2398 + 0.28051666666666666667e0_f64 * t2400 + 0.17365833333333333333e0_f64 * t2403;
    let t2472 = t2471 * t731;
    let t2475 = t723 * t723;
    let t2476 = 1.0_f64 / t2475;
    let t2477 = t159 * t2476;
    let t2478 = t167 * t167;
    let t2479 = 1.0_f64 / t2478;
    let t2480 = t2461 * t2479;
    let t2483 = t676 * t682;
    (t2462, t2472, t2477, t2480, t2483)
}
