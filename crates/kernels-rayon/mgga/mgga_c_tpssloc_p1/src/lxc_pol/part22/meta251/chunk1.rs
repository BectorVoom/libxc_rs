//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1367/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1367(t10969: f64, t61: f64, t2770: f64, t976: f64, t10947: f64, t3185: f64, t3199: f64, t1014: f64, t10471: f64, t10470: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10970 = t61 * t10969;
    let t10996 = t976 * t2770;
    let t11034 = t10947 * t3185;
    let t11037 = t10947 * t3199;
    let t11045 = t10471 * t1014;
    let t11046 = t10470 * t11045;
    (t10970, t10996, t11034, t11037, t11045, t11046)
}
