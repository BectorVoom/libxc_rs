//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1367/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1367<F: Float>(t10969: F, t61: F, t2770: F, t976: F, t10947: F, t3185: F, t3199: F, t1014: F, t10471: F, t10470: F) -> (F, F, F, F, F, F) {
    let t10970 = t61 * t10969;
    let t10996 = t976 * t2770;
    let t11034 = t10947 * t3185;
    let t11037 = t10947 * t3199;
    let t11045 = t10471 * t1014;
    let t11046 = t10470 * t11045;
    (t10970, t10996, t11034, t11037, t11045, t11046)
}
