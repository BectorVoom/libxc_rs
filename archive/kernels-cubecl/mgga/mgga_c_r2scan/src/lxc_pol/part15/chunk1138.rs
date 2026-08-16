//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1138/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1138<F: Float>(t37718: F, t37721: F, t39628: F, t39630: F, t39632: F, t39635: F, t39637: F, t39640: F, t39642: F, t39645: F, t39647: F, t39650: F) -> F {
    let t39652 = -F::cast_from(0.47609969197673950972e-2_f64) * t37718 - F::cast_from(0.14282990759302185292e-1_f64) * t37721 + t39628 + t39630 + F::cast_from(0.26198215989259945075e-1_f64) * t39632 - F::cast_from(0.12713391885412927226e1_f64) * t39635 - F::cast_from(0.16463622957338778997e-1_f64) * t39637 - F::cast_from(0.32927245914677557994e-1_f64) * t39640 + F::cast_from(0.58544643236296698113e-1_f64) * t39642 + F::cast_from(0.26004665220162805689e0_f64) * t39645 + F::cast_from(0.16463622957338778996e0_f64) * t39647 - F::cast_from(0.65495539973149862688e-2_f64) * t39650;
    t39652
}
