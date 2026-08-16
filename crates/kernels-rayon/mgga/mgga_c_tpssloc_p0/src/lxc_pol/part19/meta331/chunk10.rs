//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1191/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1191(t40645: f64, t40660: f64, t145: f64, t185: f64, t2531: f64, t9892: f64, t67: f64, t758: f64, t9915: f64, t10126: f64, t2379: f64, t2522: f64, t2523: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40622: f64, t40627: f64, t4314: f64, t776: f64, t9516: f64) -> (f64, f64, f64, f64, f64) {
    let t40661 = t40645 + t40660;
    let t40663 = t145 * t40661 * t185;
    let t40667 = t2531 * t9892;
    let t40668 = 0.20779030926817756511e3_f64 * t40667;
    let t40670 = t9915 * t67 * t758;
    let t40671 = 0.73245789224026180216e-3_f64 * t40670;
    let t40672 = 36.0_f64 * t10126 * t2379 * t4314 + 12.0_f64 * t2522 * t2523 * t9516 + 12.0_f64 * t2522 * t40622 * t776 - t39249 - t39256 - t39309 + t39312 + t39316 + t39320 + t40627 + t40663 - t40668 - t40671;
    (t40661, t40663, t40668, t40671, t40672)
}
