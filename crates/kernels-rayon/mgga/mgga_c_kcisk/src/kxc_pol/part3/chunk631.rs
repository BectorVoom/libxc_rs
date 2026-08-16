//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 631/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk631(t2042: f64, t2049: f64, t240: f64, t5212: f64, t5215: f64, t5221: f64, t5340: f64, t5525: f64, t5527: f64, t5532: f64, t5533: f64, t5552: f64, t802: f64) -> f64 {
    let t5556 = t5212 - t5215 + t5221 - t5340 + t240 * (-t2042 * t5552 - 2.0_f64 * t2049 * t5527 + t5525 * t802 + 2.0_f64 * t5532 * t5533 - t5212 + t5215 - t5221 + t5340);
    t5556
}
