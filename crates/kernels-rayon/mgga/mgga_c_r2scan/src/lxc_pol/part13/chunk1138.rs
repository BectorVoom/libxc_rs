//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1138/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1138(t37718: f64, t37721: f64, t39628: f64, t39630: f64, t39632: f64, t39635: f64, t39637: f64, t39640: f64, t39642: f64, t39645: f64, t39647: f64, t39650: f64) -> f64 {
    let t39652 = -0.47609969197673950972e-2_f64 * t37718 - 0.14282990759302185292e-1_f64 * t37721 + t39628 + t39630 + 0.26198215989259945075e-1_f64 * t39632 - 0.12713391885412927226e1_f64 * t39635 - 0.16463622957338778997e-1_f64 * t39637 - 0.32927245914677557994e-1_f64 * t39640 + 0.58544643236296698113e-1_f64 * t39642 + 0.26004665220162805689e0_f64 * t39645 + 0.16463622957338778996e0_f64 * t39647 - 0.65495539973149862688e-2_f64 * t39650;
    t39652
}
