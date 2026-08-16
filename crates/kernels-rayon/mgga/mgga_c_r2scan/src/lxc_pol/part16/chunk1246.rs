//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1246/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1246(t37393: f64, t37398: f64, t37401: f64, t37407: f64, t37413: f64, t37415: f64, t37419: f64, t37423: f64, t42840: f64, t42843: f64, t42845: f64, t42850: f64, t42854: f64, t42858: f64, t42860: f64) -> f64 {
    let t43851 = -t42840 + t42843 - 0.43368970657079495312e-4_f64 * t37393 - t37398 + 0.46116394948205481339e-3_f64 * t37401 + t42845 + t37407 + t37413 - t37415 + t42850 - t42854 + 0.14905073231436680509e-2_f64 * t37419 + t42858 + 0.36021158228745895953e-3_f64 * t37423 - t42860;
    t43851
}
