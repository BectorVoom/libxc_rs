//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3206/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3206(t43888: f64, t56236: f64, t56343: f64, t56345: f64, t56360: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64) -> f64 {
    let t84174 = 0.9877777777777777778e-2_f64 * t68332 + 0.19755555555555555556e-1_f64 * t68334 + 0.59266666666666666668e-1_f64 * t68336 - t56343 + t56345 + 0.26670000000000000001e0_f64 * t81224 + 0.14816666666666666667e-1_f64 * t81228 - 0.5487654320987654321e-2_f64 * t81230 + 0.19755555555555555556e-1_f64 * t81232 - 0.29633333333333333334e-1_f64 * t81234 - 0.4938888888888888889e-2_f64 * t81236 + t56360 - 0.46096296296296296297e-1_f64 * t56236 - 0.14816666666666666667e-1_f64 * t68389 + 0.39511111111111111112e-1_f64 * t68399 + 0.4938888888888888889e-1_f64 * t81242 - 0.17780000000000000001e0_f64 * t81245 - 0.15365432098765432099e-1_f64 * t43888 - 0.59266666666666666668e-1_f64 * t68454 - 0.88900000000000000002e-1_f64 * t68456;
    t84174
}
