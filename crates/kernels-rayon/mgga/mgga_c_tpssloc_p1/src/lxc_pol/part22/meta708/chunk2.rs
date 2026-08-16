//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2304/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2304(t67099: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40673: f64, t40679: f64, t40685: f64, t67095: f64, t67096: f64, t67097: f64, t16693: f64, t16713: f64) -> (f64, f64, f64) {
    let t67100 = 0.5848223622634646207e0_f64 * t67099;
    let t67101 = t67095 - t39309 + t39312 + t39316 + t39320 - t67096 + t40673 - t40679 + t67097 - t40685 - t67100;
    let t67104 = 72.0_f64 * t16693 * t16713;
    (t67100, t67101, t67104)
}
