//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2548/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2548(t21810: f64, t3259: f64, t50834: f64, t51137: f64, t63291: f64, t63306: f64, t63308: f64, t63841: f64, t63843: f64, t63845: f64, t71333: f64, t71335: f64, t71337: f64) -> (f64, f64) {
    let t71547 = 1.0_f64 * t3259 * t21810;
    let t71558 = -0.59793333333333333334e0_f64 * t63291 + 0.19931111111111111111e0_f64 * t63306 - 0.33218518518518518518e0_f64 * t63308 - 0.27385555555555555556e-1_f64 * t71333 + 0.54771111111111111112e-1_f64 * t71335 - 0.32862666666666666666e0_f64 * t71337 + t51137 - 0.93011851851851851854e0_f64 * t50834 - 0.73028148148148148146e-1_f64 * t63841 - 0.32862666666666666666e0_f64 * t63843 + 0.5477111111111111111e-1_f64 * t63845;
    (t71547, t71558)
}
