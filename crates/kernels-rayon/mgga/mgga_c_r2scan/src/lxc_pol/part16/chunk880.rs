//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 880/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk880(t2155: f64, t9319: f64, t538: f64, t9246: f64, t6155: f64, t113: f64, t8847: f64, t2115: f64, t2562: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9371 = t2155 * t9319;
    let t9373 = t538 * t9246;
    let t9374 = t6155 * t9373;
    let t9376 = t8847 * t113;
    let t9377 = t2115 * t9376;
    let t9378 = t2155 * t9377;
    let t9380 = t2562 * t910;
    (t9371, t9373, t9374, t9376, t9377, t9378, t9380)
}
