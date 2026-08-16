//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2250/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2250(t1100: f64, t18730: f64, t1107: f64, t11243: f64, t5992: f64, t1102: f64, t4756: f64, t4764: f64, t3287: f64, t5999: f64, t11265: f64, t4748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18731 = t1100 * t18730;
    let t18742 = t1107 * t18730;
    let t18746 = t11243 * t5992;
    let t18747 = t18746 * t1102;
    let t18749 = t4764 * t4756;
    let t18751 = t3287 * t5999;
    let t18752 = t18751 * t1102;
    let t18754 = t11265 * t5992;
    let t18755 = t18754 * t1102;
    let t18757 = t4748 * t4756;
    (t18731, t18742, t18746, t18747, t18749, t18751, t18752, t18754, t18755, t18757)
}
