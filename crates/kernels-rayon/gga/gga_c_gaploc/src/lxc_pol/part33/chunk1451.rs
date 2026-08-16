//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1451/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1451(t12284: f64, t12294: f64, t32413: f64, t32415: f64, t32417: f64, t32429: f64, t32431: f64, t32434: f64, t32439: f64, t32441: f64, t32443: f64, t32446: f64, t32448: f64, t32452: f64, t32456: f64, t7137: f64) -> f64 {
    let t39425 = t32413 + t32415 - t32417 - t32429 + t32431 + t32434 - t32439 - t32441 + t32443 - t32446 - t32448 - t32452 + t32456 - 0.61524209841137794271e-1_f64 * t7137 * t12284 + 0.41016139894091862847e-1_f64 * t7137 * t12294;
    t39425
}
