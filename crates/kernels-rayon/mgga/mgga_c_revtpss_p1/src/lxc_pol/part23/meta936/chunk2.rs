//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3078/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3078(t43888: f64, t56236: f64, t58073: f64, t58075: f64, t58090: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64) -> f64 {
    let t81397 = 2.0_f64 / 9.0_f64 * t68332 + 4.0_f64 / 9.0_f64 * t68334 + 4.0_f64 / 3.0_f64 * t68336 - t58073 + t58075 + 6.0_f64 * t81224 + t81228 / 3.0_f64 - 10.0_f64 / 81.0_f64 * t81230 + 4.0_f64 / 9.0_f64 * t81232 - 2.0_f64 / 3.0_f64 * t81234 - t81236 / 9.0_f64 + t58090 - 28.0_f64 / 27.0_f64 * t56236 - t68389 / 3.0_f64 + 8.0_f64 / 9.0_f64 * t68399 + 10.0_f64 / 9.0_f64 * t81242 - 4.0_f64 * t81245 - 28.0_f64 / 81.0_f64 * t43888 - 4.0_f64 / 3.0_f64 * t68454 - 2.0_f64 * t68456;
    t81397
}
