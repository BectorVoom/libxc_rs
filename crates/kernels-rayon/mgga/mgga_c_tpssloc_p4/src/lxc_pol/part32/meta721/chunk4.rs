//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2294/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2294(t24574: f64, t29804: f64, t18525: f64, t19249: f64, t2123: f64, t2155: f64, t24589: f64, t24590: f64, t29532: f64, t29808: f64, t29812: f64, t3487: f64, t6140: f64, t64595: f64, t7283: f64, t7295: f64, t7392: f64, t85701: f64, t86403: f64, t94427: f64, t94436: f64, t94439: f64, t94446: f64, t94451: f64, t94456: f64) -> f64 {
    let t103261 = t24574 * t29804;
    let t103279 = -t64595 * t2155 - t94427 + 0.18277045187202515961e-2_f64 * t85701 + 0.54831135561607547883e-2_f64 * t103261 - 0.54831135561607547884e-2_f64 * t24589 * t86403 * t29808 - 0.36554090374405031923e-2_f64 * t94436 - t94439 - t94446 - t19249 * t7392 + t94451 - t94456 + 4.0_f64 * t3487 * t29532 - 0.82246703342411321825e-2_f64 * t7283 * t18525 * t2123 - 0.82246703342411321825e-2_f64 * t7283 * t6140 * t7295 + 0.27415567780803773942e-2_f64 * t24589 * t24590 * t29812;
    t103279
}
