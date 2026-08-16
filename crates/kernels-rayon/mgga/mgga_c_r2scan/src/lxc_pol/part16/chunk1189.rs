//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1189/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1189(t39628: f64, t39630: f64, t39635: f64, t39640: f64, t39673: f64, t41478: f64, t41480: f64, t43195: f64, t43200: f64, t43203: f64, t43205: f64, t43209: f64) -> f64 {
    let t43211 = 0.13099107994629972538e-1_f64 * t43195 + t39628 + t39630 - 0.25426783770825854452e1_f64 * t39635 - t41478 - 0.32927245914677557992e-1_f64 * t39640 + t41480 + 0.13099107994629972538e-1_f64 * t43200 - 0.87327386630866483584e-2_f64 * t43203 - t39673 - 0.13099107994629972538e-1_f64 * t43205 + 0.65495539973149862688e-2_f64 * t43209;
    t43211
}
