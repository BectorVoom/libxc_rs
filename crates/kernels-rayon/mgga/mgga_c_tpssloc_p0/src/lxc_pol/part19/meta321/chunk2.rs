//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1140/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1140(t12012: f64, t12303: f64, t193: f64, t3918: f64, t3919: f64, t3924: f64, t39590: f64, t39593: f64, t39595: f64, t39597: f64, t39602: f64, t39604: f64, t39606: f64, t39608: f64, t39610: f64, t39612: f64, t39615: f64, t39621: f64, t39622: f64, t5126: f64) -> f64 {
    let t39626 = 12.0_f64 * t12012 * t3918 * t3919 + 72.0_f64 * t12303 * t3919 * t5126 + 18.0_f64 * t193 * t3924 * t39622 + t39590 - t39593 + t39595 - t39597 + t39602 + t39604 - t39606 - t39608 + t39610 - t39612 + t39615 + t39621;
    t39626
}
