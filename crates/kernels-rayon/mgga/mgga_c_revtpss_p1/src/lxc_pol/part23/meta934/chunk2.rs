//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3073/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3073(t43888: f64, t56236: f64, t57872: f64, t57874: f64, t57889: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64) -> f64 {
    let t81304 = 0.11872222222222222222e-1_f64 * t68332 + 0.23744444444444444444e-1_f64 * t68334 + 0.71233333333333333332e-1_f64 * t68336 - t57872 + t57874 + 0.32055e0_f64 * t81224 + 0.17808333333333333333e-1_f64 * t81228 - 0.65956790123456790123e-2_f64 * t81230 + 0.23744444444444444444e-1_f64 * t81232 - 0.35616666666666666667e-1_f64 * t81234 - 0.5936111111111111111e-2_f64 * t81236 + t57889 - 0.55403703703703703703e-1_f64 * t56236 - 0.17808333333333333333e-1_f64 * t68389 + 0.47488888888888888888e-1_f64 * t68399 + 0.5936111111111111111e-1_f64 * t81242 - 0.21369999999999999999e0_f64 * t81245 - 0.18467901234567901234e-1_f64 * t43888 - 0.71233333333333333332e-1_f64 * t68454 - 0.10685e0_f64 * t68456;
    t81304
}
