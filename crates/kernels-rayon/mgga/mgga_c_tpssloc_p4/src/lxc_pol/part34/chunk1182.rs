//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1182/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1182(t105758: f64, t23788: f64, t20800: f64, t28: f64, t21066: f64, t1649: f64, t5527: f64, t1484: f64, t5966: f64, t5544: f64, t20778: f64, t105731: f64, t25927: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t106624 = t23788 * t105758;
    let t106627 = t28 * t20800;
    let t106636 = t28 * t21066;
    let t106640 = t1649 * t5527;
    let t106647 = t5966 * t1484;
    let t106651 = t1649 * t5544;
    let t106655 = t28 * t20778;
    let t106671 = t25927 * t105731;
    (t106624, t106627, t106636, t106640, t106647, t106651, t106655, t106671)
}
