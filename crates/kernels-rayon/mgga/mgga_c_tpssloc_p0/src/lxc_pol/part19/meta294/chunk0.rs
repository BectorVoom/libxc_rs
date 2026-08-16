//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1073/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1073(t3698: f64, t3701: f64, t12125: f64, t12128: f64, t12131: f64, t12133: f64, t12135: f64, t12137: f64, t12139: f64, t12141: f64, t12143: f64, t1307: f64, t3719: f64, t3734: f64, t3914: f64, t3918: f64, t3919: f64, t5126: f64, t5160: f64, t6999: f64, t9853: f64, t9859: f64) -> (f64, f64) {
    let t12477 = t3698 * t3701;
    let t12490 = -9.0_f64 * t12477 * t1307 * t3918 + 9.0_f64 * t3719 * t3918 * t3919 + 18.0_f64 * t3734 * t3919 * t5126 - 3.0_f64 * t3914 * t5160 * t6999 + t12125 + t12128 + t12131 + t12133 - t12135 + t12137 + t12139 - t12141 - t12143 + t9853 + t9859;
    (t12477, t12490)
}
