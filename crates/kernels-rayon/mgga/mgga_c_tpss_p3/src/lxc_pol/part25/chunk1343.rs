//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1343/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1343(t21136: f64, t5791: f64, t1791: f64, t69152: f64, t1792: f64, t18666: f64, t18673: f64, t19342: f64, t19352: f64, t20264: f64, t20282: f64, t21146: f64, t5794: f64, t6073: f64, t62019: f64, t6304: f64, t65189: f64, t67326: f64, t67510: f64, t67512: f64, t69147: f64, t69186: f64, t69281: f64) -> f64 {
    let t71529 = t21136 * t5791;
    let t71535 = t1791 * t69152;
    let t71544 = t69281 * t1792 / 3.0_f64 + t21146 * t5794 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t19352 * t6304 + 2.0_f64 / 3.0_f64 * t6073 * t20282 + 16.0_f64 / 9.0_f64 * t71529 - 880.0_f64 / 27.0_f64 * t67510 - 352.0_f64 / 27.0_f64 * t67512 + 20.0_f64 * t18666 * t69147 - 20.0_f64 * t62019 * t71535 + 20.0_f64 * t67326 * t19342 + 20.0_f64 / 3.0_f64 * t65189 * t20264 + 20.0_f64 / 3.0_f64 * t69186 * t18673;
    t71544
}
