//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2173/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2173(t22892: f64, t22893: f64, t28138: f64, t1336: f64, t1352: f64, t16060: f64, t19810: f64, t26404: f64, t26442: f64, t26456: f64, t26458: f64, t28152: f64, t3777: f64, t5234: f64, t5287: f64, t5344: f64, t544: f64, t553: f64, t7745: f64, t91065: f64, t91077: f64, t93795: f64, t93796: f64, t97172: f64, t97181: f64, t97189: f64, t97200: f64, t97468: f64, t97488: f64, t97491: f64) -> f64 {
    let t97494 = t22892 * t22893 * t28138;
    let t97496 = -2.0_f64 * t5344 * t97189 * t1352 - 0.19190897446562641759e-1_f64 * t97200 + t91065 + t544 * t553 * t97468 - 2.0_f64 * t1336 * t26458 * t5287 - 2.0_f64 * t16060 * t7745 - 2.0_f64 * t5234 * t26442 - 2.0_f64 * t5234 * t26456 - 2.0_f64 * t19810 * t26404 - t3777 * t28152 + t91077 - t93795 - t5344 * t97181 * t1352 + t93796 - t5344 * t97172 * t1352 + 0.16449340668482264365e-1_f64 * t97488 + 0.3289868133696452873e-1_f64 * t97491 + 0.82246703342411321825e-2_f64 * t97494;
    t97496
}
