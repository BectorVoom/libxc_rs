//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 784/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk784(t598: f64, t8497: f64, t7312: f64, t8447: f64, t8451: f64, t8453: f64, t8455: f64, t8459: f64, t8466: f64, t8470: f64, t8474: f64, t8478: f64, t8482: f64, t8487: f64, t8492: f64, t8494: f64) -> f64 {
    let t8498 = t598 * t8497;
    let t8500 = 0.15724046144802076034e-2_f64 * t8447 + 0.94344276868812456204e-3_f64 * t8451 + 0.42874018118069736972e-3_f64 * t8453 - 0.17149607247227894789e-2_f64 * t8455 + t7312 - 0.7862023072401038017e-3_f64 * t8459 - 0.47172138434406228102e-2_f64 * t8466 + 0.15724046144802076034e-2_f64 * t8470 - 0.23586069217203114051e-2_f64 * t8474 + 0.31448092289604152068e-3_f64 * t8478 - 0.10718504529517434243e-3_f64 * t8482 + 0.47172138434406228102e-3_f64 * t8487 + 0.31448092289604152068e-3_f64 * t8492 - 0.21437009059034868486e-3_f64 * t8494 - 0.21437009059034868486e-3_f64 * t8498;
    t8500
}
