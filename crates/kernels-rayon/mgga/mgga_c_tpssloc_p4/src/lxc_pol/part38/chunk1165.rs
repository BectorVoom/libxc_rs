//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1165/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1165(t11094: f64, t1637: f64, t14257: f64, t14262: f64, t14376: f64, t14378: f64, t14381: f64, t14384: f64, t14387: f64, t14391: f64, t14394: f64, t14398: f64, t14424: f64, t14472: f64, t14475: f64, t14477: f64, t14479: f64, t14482: f64, t14484: f64, t14486: f64, t3209: f64, t3213: f64, t4700: f64, t4701: f64) -> f64 {
    let t14667 = t1637 * t11094;
    let t14673 = 2.0_f64 * t14667 * t3213 * t4700 - t3209 * t4700 * t4701 - t14257 - t14262 - t14376 + t14378 - t14381 - t14384 - t14387 + t14391 + t14394 + t14398 + t14424 + t14472 - t14475 - t14477 + t14479 - t14482 - t14484 - t14486;
    t14673
}
