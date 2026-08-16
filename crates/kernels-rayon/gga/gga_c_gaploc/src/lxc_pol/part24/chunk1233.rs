//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1233/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1233(t10731: f64, t7129: f64, t32429: f64, t32431: f64, t32434: f64, t32439: f64, t32441: f64, t32443: f64, t32446: f64, t32448: f64, t32452: f64, t32456: f64, t32458: f64, t32461: f64, t32464: f64) -> f64 {
    let t32466 = 0.18457262952341338281e0_f64 * t7129 * t10731;
    let t32467 = -t32429 + t32431 + t32434 - t32439 - t32441 + t32443 - t32446 - t32448 - t32452 + t32456 + t32458 - t32461 + t32464 + t32466;
    t32467
}
