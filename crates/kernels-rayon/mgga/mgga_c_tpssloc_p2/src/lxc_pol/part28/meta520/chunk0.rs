//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1768/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1768(t131: f64, t1365: f64, t22648: f64, t6897: f64, t794: f64, t154: f64, t21: f64, t6896: f64, t6898: f64, t22797: f64, t3770: f64, t213: f64, t6924: f64, t9223: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80730 = t1365 * t131;
    let t80738 = t6897 * t794 * t22648;
    let t80741 = t21 * t154;
    let t80742 = t80741 * t6896;
    let t80743 = t80742 * t6898;
    let t80761 = t22797 * t3770;
    let t80766 = t9223 * t6924 * t213;
    (t80730, t80738, t80741, t80742, t80743, t80761, t80766)
}
