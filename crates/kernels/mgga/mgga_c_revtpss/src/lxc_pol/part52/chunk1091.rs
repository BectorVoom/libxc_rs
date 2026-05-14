//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1091/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1091<F: Float>(t32636: F, t7898: F, t34266: F, t7235: F, t2033: F, t28196: F, t28286: F, t5778: F, t28177: F, t8698: F, t2014: F, t33651: F, t7536: F, t34270: F, t7316: F, t25082: F, t26405: F, t5591: F) -> (F, F, F, F, F, F, F) {
    let t128235 = t7898 * t32636;
    let t128236 = t7235 * t34266;
    let t128240 = 2.0 * t28196 * t28286 * t2033 * t5778;
    let t128242 = 3.0 * t8698 * t28177;
    let t128244 = t2014 * t7536 * t33651;
    let t128245 = t34270 * t7316;
    let t128251 = 3.0 * t25082 * t26405 * t2033 * t5591;
    (t128235, t128236, t128240, t128242, t128244, t128245, t128251)
}
