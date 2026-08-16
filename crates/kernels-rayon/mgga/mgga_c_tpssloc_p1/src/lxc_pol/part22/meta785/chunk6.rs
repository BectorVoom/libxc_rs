//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2709/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2709(t5: f64, t12568: f64, t12571: f64, t1437: f64, t19299: f64, t19310: f64, t19313: f64, t19318: f64, t39043: f64, t3958: f64, t4021: f64, t45844: f64, t46085: f64, t46086: f64, t46087: f64, t46088: f64, t46089: f64, t46090: f64, t46104: f64, t5389: f64, t5445: f64, t55880: f64, t55921: f64, t645: f64, t75284: f64, t75552: f64, t86: f64) -> f64 {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t75554 = piecewise3(t8, 0.0_f64, (-t46085 - t46086 - t46087 - t46088 + t46089 + t46090 + t39043) * t86 - 4.0_f64 * t75284 * t645 - 12.0_f64 * t55880 * t1437 + 60.0_f64 * t55921 * t3958 - 12.0_f64 * t19299 * t4021 + 60.0_f64 * t46104 * t5389 - 360.0_f64 * t45844 * t19310 + 120.0_f64 * t12571 * t19313 - 12.0_f64 * t12568 * t5445 + 60.0_f64 * t12571 * t19318 + t75552);
    t75554
}
